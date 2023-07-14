use plotters::prelude::*;

fn chebyshev(pol: usize, x: f32) -> f32 {
    match pol {
        0 => 1f32,
        1 => x,
        _ => 2f32 * x * chebyshev(pol - 1, x) - chebyshev(pol - 2, x),
    }
}

fn bisection(pol: usize, mut a: f32, mut b: f32, precision: f32) -> (f32, f32) {
    assert!(chebyshev(pol, a) * chebyshev(pol, b) < 0f32);

    let mut c = a;
    while b - a >= precision {
        c = (a+b)/2f32;

        let fc = chebyshev(pol, c);
        if fc == 0f32 { 
            break;
        }

        if fc * chebyshev(pol, a) < 0f32 {
            b = c;
        } else {
            a = c;
        }
    }
    (c, chebyshev(pol, c))
}

// b
// fn main() {
//     let root = BitMapBackend::new("output/a.png", (1920, 1080)).into_drawing_area();
//     root.fill(&WHITE).unwrap();
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Chebyshev", ("sans-serif", 30).into_font())
//         .margin(10)
//         .x_label_area_size(60)
//         .y_label_area_size(60)
//         .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
//         .unwrap();

//     chart
//         .configure_mesh()
//         .axis_desc_style(("sans-serif", 20))
//         .draw()
//         .unwrap();

//     let colors = &[RED, GREEN, BLUE, CYAN, BLACK];
//     for (p, c) in colors.iter().enumerate() {
//         chart
//             .draw_series(LineSeries::new(
//                 (-1f32..1f32)
//                     .step(0.01f32)
//                     .values()
//                     .map(|x| (x, chebyshev(p, x))),
//                 c,
//             ))
//             .unwrap()
//             .label(format!("T_{p}"))
//             .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], c.clone()));
//     }

//     chart
//     .configure_series_labels()
//     .background_style(&WHITE.mix(0.8))
//     .border_style(&BLACK)
//     .draw()
//     .unwrap();

//     root.present().unwrap();
// }

// d
fn main() {
    let precisions = &[1e-1f32, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6];
    let ab = &[(-1f32, 1f32), (0.2f32, 1f32), (-1f32, 1f32), (-0.8f32, 1f32)];
    for (p, (a, b)) in ab.iter().enumerate() {
        let p = p + 1;
        for precision in precisions {
            let begin = std::time::Instant::now();
            let val = bisection(p, *a, *b, *precision);
            let end = begin.elapsed();
            println!("{p} | {precision} | {end:?} | {val:.6?}");
        }
        println!("");
    }
}
