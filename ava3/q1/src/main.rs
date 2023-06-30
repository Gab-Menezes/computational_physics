#![feature(array_windows)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(stmt_expr_attributes)]
#![feature(sort_floats)]
#![feature(generic_arg_infer)]

use nalgebra::*;
use plotters::prelude::*;

macro_rules! sum {
    () => {
        0
    };
    ($x:expr $(, $xs:expr)*) => {
        $x + sum!($($xs),*)
    };
}

macro_rules! repeat_slice {
    ($(($elem:expr; $n:expr)),*) => (
        {
            use std::mem::MaybeUninit;
            const N: usize = sum!($($n),*);
            let mut s: [MaybeUninit<_>; N] = MaybeUninit::uninit_array();

            let mut begin = 0usize;
            let mut end = 0usize;

            $(
                let elem = $elem;

                end += $n;
                for i in begin..end {
                    s[i].write(elem.clone());
                }
                #[allow(unused_assignments)]
                begin += $n;
            )*
            unsafe {
                MaybeUninit::array_assume_init(s)
            }
        }
    );
}

fn main() {
    const NUM_BODIES: usize = 300;
    const MASS: f32 = 1f32;
    const K1: f32 = 1f32;
    const K2: f32 = 5f32;

    let springs = repeat_slice![(K1; NUM_BODIES/2 - 2), (K2; 2), (K1; NUM_BODIES/2 - 1)];
    let mut matrix = SMatrix::<f32, NUM_BODIES, NUM_BODIES>::zeros();

    assert_eq!(NUM_BODIES - 1, springs.len());

    let first = springs.first().unwrap() / MASS;
    let last = springs.last().unwrap() / MASS;
    matrix[(0, 0)] = first;
    matrix[(0, 1)] = -first;
    matrix[(NUM_BODIES - 1, NUM_BODIES - 1)] = last;
    matrix[(NUM_BODIES - 1, NUM_BODIES - 2)] = -last;
    for (i, window) in springs.array_windows::<2>().enumerate() {
        let i = i + 1;
        matrix[(i, i - 1)] = -window[0] / MASS;
        matrix[(i, i)] = (window[0] + window[1]) / MASS;
        matrix[(i, i + 1)] = -window[1] / MASS;
    }
    let e = nalgebra_lapack::Eigen::new(matrix, false, true).unwrap();
    let eval = &e.eigenvalues_re;
    let evec = e
        .eigenvectors
        .as_ref()
        .unwrap()
        .as_view::<Const<_>, Const<_>, Const<_>, Const<_>>();

    {
        let x_range = (0f32..20f32).step(0.2f32).use_floor();
        let y_range = 0u32..((NUM_BODIES / 5) as u32);

        let name = format!("output/state-density-{NUM_BODIES}.png");
        let root = BitMapBackend::new(&name, (1920, 1080)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption("State density", ("sans-serif", 30).into_font())
            .margin(10)
            .x_label_area_size(60)
            .y_label_area_size(60)
            .build_cartesian_2d(x_range, y_range)
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("w^2")
            .y_desc("#")
            .axis_desc_style(("sans-serif", 20))
            .draw()
            .unwrap();

        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .style(RED.filled())
                    .data(eval.iter().map(|x| (*x, 1))),
            )
            .unwrap();

        root.present().unwrap();
    }
    {
        let mut zipped = eval.iter().zip(evec.column_iter()).collect::<Vec<_>>();
        zipped.sort_unstable_by(|(v1, _), (v2, _)| v1.partial_cmp(v2).unwrap());

        for zipped in [
            &zipped[..5],
            &zipped[NUM_BODIES - 5..],
        ] {
            for (val, vec) in zipped.iter() {
                let x_range = 0usize..NUM_BODIES;
                let y_range = (vec.min() - 0.1f32)..(vec.max() + 0.1f32);
                let name = format!("output/relative-displacement-{NUM_BODIES}-{val}.png");
                let root = BitMapBackend::new(&name, (1920, 1080)).into_drawing_area();
                root.fill(&WHITE).unwrap();
                let mut chart = ChartBuilder::on(&root)
                    .caption(format!("Relative Displacement"), ("sans-serif", 30).into_font())
                    .margin(10)
                    .x_label_area_size(60)
                    .y_label_area_size(60)
                    .build_cartesian_2d(x_range, y_range)
                    .unwrap();

                chart
                    .configure_mesh()
                    .x_desc("idx")
                    .y_desc("Relative Displacement")
                    .axis_desc_style(("sans-serif", 20))
                    .draw()
                    .unwrap();

                    chart
                        .draw_series(LineSeries::new(
                            vec.iter().enumerate().map(|(i, v)| (i, *v)),
                            RED,
                        ))
                        .unwrap()
                        .label(val.to_string())
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
                    
                    chart
                    .configure_series_labels()
                    .background_style(WHITE.mix(0.8))
                    .border_style(BLACK)
                    .draw()
                    .unwrap();
                
                root.present().unwrap();
            }
        }
    }
}
