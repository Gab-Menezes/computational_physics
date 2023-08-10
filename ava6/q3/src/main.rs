use std::{
    f64::consts::{E, PI},
    ops::Range,
};

use plotters::prelude::*;

fn gamma(a: usize, x: f64) -> f64 {
    x.powi((a - 1) as i32)*E.powf(-x)
}

// a
fn main() {
    let x_range = 0f64..5f64;

    let root = BitMapBackend::new("output/a.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Gamma Function", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range.clone(), -1f64..2f64)
        .unwrap();

    chart
        .configure_mesh()
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    let colors = &[RED, GREEN, BLUE];
    for (p, c) in colors.iter().enumerate() {
        let p = p + 2;
        chart
            .draw_series(LineSeries::new(
                x_range
                    .clone()
                    .step(0.1f64)
                    .values()
                    .map(|x| (x, gamma(p, x))),
                c,
            ))
            .unwrap()
            .label(format!("Gamma({p})"))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], c.clone()));
    }

    chart
    .configure_series_labels()
    .background_style(&WHITE.mix(0.8))
    .border_style(&BLACK)
    .draw()
    .unwrap();

    root.present().unwrap();
}