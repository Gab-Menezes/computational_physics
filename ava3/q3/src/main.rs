use std::f32::consts::PI;
use std::f32::consts::E;

use plotters::prelude::*;

// a
// fn main() {
//     let points = (0u32..(2f32*PI*1000f32) as u32)
//     .map(|theta| {
//         let theta = theta as f32 * 0.001f32;
//         let x = 2f32*theta.cos() + (2f32*theta).cos();
//         let y = 2f32*theta.sin() - (2f32*theta).sin();
//         (x, y)
//     });

//     let root = BitMapBackend::new("output/a.png", (1920, 1080)).into_drawing_area();
//     root.fill(&WHITE).unwrap();
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Deltoid", ("sans-serif", 30).into_font())
//         .margin(10)
//         .x_label_area_size(60)
//         .y_label_area_size(60)
//         .build_cartesian_2d(-2f32..4f32, -3f32..3f32)
//         .unwrap();

//     chart
//         .configure_mesh()
//         .x_desc("x")
//         .y_desc("y")
//         .axis_desc_style(("sans-serif", 20))
//         .draw()
//         .unwrap();

//     chart.draw_series(LineSeries::new(points, &RED)).unwrap();

//     root.present().unwrap();
// }

// b
// fn main() {
//     let points = (0u32..(10f32*PI*1000f32) as u32)
//     .map(|theta| {
//         let theta = theta as f32 * 0.001f32;
//         let r = theta * theta;
//         let x = r*theta.cos();
//         let y = r*theta.sin();
//         (x, y)
//     });

//     let root = BitMapBackend::new("output/b.png", (1920, 1080)).into_drawing_area();
//     root.fill(&WHITE).unwrap();
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Theta Squared", ("sans-serif", 30).into_font())
//         .margin(20)
//         .x_label_area_size(60)
//         .y_label_area_size(60)
//         .build_cartesian_2d(-1000f32..1000f32, -1000f32..1000f32)
//         .unwrap();

//     chart
//         .configure_mesh()
//         .x_desc("x")
//         .y_desc("y")
//         .axis_desc_style(("sans-serif", 20))
//         .draw()
//         .unwrap();

//     chart.draw_series(LineSeries::new(points, &RED)).unwrap();

//     root.present().unwrap();
// }

// c
fn main() {
    let points = (0u32..(24f32*PI*10000f32) as u32)
    .map(|theta| {
        let theta = theta as f32 * 0.0001f32;
        let mut r = E.powf(theta.cos());
        r -= 2f32*(4f32*theta).cos();
        r += (theta/12f32).sin().powf(5f32);
        let x = r*theta.cos();
        let y = r*theta.sin();
        (x, y)
    });

    let root = BitMapBackend::new("output/c.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Fey's Function", ("sans-serif", 30).into_font())
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(-4f32..4f32, -4f32..4f32)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("x")
        .y_desc("y")
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    chart.draw_series(LineSeries::new(points, &RED)).unwrap();

    root.present().unwrap();
}