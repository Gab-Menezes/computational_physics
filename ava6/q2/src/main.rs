use std::{
    f64::consts::{E, PI},
    ops::Range,
};

use plotters::prelude::*;

fn hermite(n: u128, x: f64) -> f64 {
    match n {
        0 => 1f64,
        1 => 2f64 * x,
        _ => 2f64 * x * hermite(n - 1, x) - 2f64 * (n - 1) as f64 * hermite(n - 2, x),
    }
}

fn factorial(n: u128) -> u128 {
    match n {
        0 => 1,
        _ => (1..=n).reduce(|acc, n| acc * n).unwrap(),
    }
}

fn harmonic(n: u128, x: f64) -> f64 {
    let bottom = (2f64.powi(n as i32) * factorial(n) as f64 * PI.sqrt()).sqrt();
    1f64 / bottom * E.powf(-1f64 * x * x / 2f64) * hermite(n, x)
}

// a
// fn main() {
//     let x_range = -4f64..4f64;

//     let root = BitMapBackend::new("output/a.png", (1920, 1080)).into_drawing_area();
//     root.fill(&WHITE).unwrap();
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Harmonic Oscilator", ("sans-serif", 30).into_font())
//         .margin(10)
//         .x_label_area_size(60)
//         .y_label_area_size(60)
//         .build_cartesian_2d(x_range.clone(), -2f64..2f64)
//         .unwrap();

//     chart
//         .configure_mesh()
//         .axis_desc_style(("sans-serif", 20))
//         .draw()
//         .unwrap();

//     let colors = &[RED, GREEN, BLUE, CYAN];
//     for (p, c) in colors.iter().enumerate() {
//         chart
//             .draw_series(LineSeries::new(
//                 x_range
//                     .clone()
//                     .step(0.1f64)
//                     .values()
//                     .map(|x| (x, harmonic(p as u128, x))),
//                 c,
//             ))
//             .unwrap()
//             .label(format!("H_{p}"))
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

// b
fn main() {
    let x_range = -10f64..10f64;
    let root = BitMapBackend::new("output/b.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Harmonic Oscilator", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range.clone(), -2f64..2f64)
        .unwrap();

    chart
        .configure_mesh()
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(x_range
            .clone()
            .step(0.01f64)
            .values()
            .map(|x| (x, harmonic(30, x))), RED))
        .unwrap()
        .label("H_30")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
}