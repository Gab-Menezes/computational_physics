use std::{f32::consts::E, ops::Range};

use plotters::prelude::*;

fn p(r: f32) -> f32 {
    4f32*r*r*E.powf(-2f32*r)
}

fn simpson(g: impl Fn(f32) -> f32, x_range: Range<f32>, slices: u32) -> f32 {
    let step = (x_range.end - x_range.start) / slices as f32;
    let start = g(x_range.start) + g(x_range.end);
    let sum = (1..slices).fold(start, |acc, i| {
        let fac = if i % 2 == 0 { 2f32 } else { 4f32 };
        acc + fac * g(x_range.start + i as f32 * step)
    });
    sum * step * 1f32 / 3f32
}

// b
// fn main() {
//     let x_range = 0f32..2.1f32;
//     let points: Vec<_> = x_range.clone()
//     .step(0.1f32)
//     .values()
//     .map(|r| (r, simpson(p, 0f32..r, 100)))
//     .collect();

//     let name = format!("output/p.png");
//     let root = BitMapBackend::new(&name, (1920, 1080)).into_drawing_area();
//     root.fill(&WHITE).unwrap();
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Probability vs R", ("sans-serif", 30).into_font())
//         .margin(10)
//         .x_label_area_size(60)
//         .y_label_area_size(60)
//         .build_cartesian_2d(x_range, 0f32..1f32)
//         .unwrap();

//     chart
//         .configure_mesh()
//         .x_desc("R")
//         .y_desc("P")
//         .axis_desc_style(("sans-serif", 20))
//         .draw()
//         .unwrap();

//     chart.draw_series(LineSeries::new(points, &RED)).unwrap();

//     root.present().unwrap();
// }

fn search(g: impl Fn(f32) -> f32, mut x_range: Range<f32>, target: f32, precision: f32) -> (f32, f32) {
    loop {
        let middle = (x_range.end - x_range.start)/2f32 + x_range.start;
        let v = g(middle);
        if (target - v).abs() <= precision {
            return (middle, v);
        }

        if v > target {
            x_range.end = middle;
        } else {
            x_range.start = middle;
        }
    }
}
//c
fn main() {
    let x_range = 0f32..2f32;
    let g = search(|r| simpson(p, 0f32..r, 100), x_range, 0.01f32, 1e-5);
    println!("{g:?}");
}