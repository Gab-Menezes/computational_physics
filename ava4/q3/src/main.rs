use plotters::prelude::*;

fn up(n: u32) -> f64 {
    let mut s = 0f64;
    for i in 1..=n {
        s += 1f64 / i as f64;
    }
    return s;
}

fn down(n: u32) -> f64 {
    let mut s = 0f64;
    for i in (1..=n).rev() {
        s += 1f64 / i as f64;
    }
    return s;
}

fn main() {
    let x = 1_000;
    let points: Vec<_> = (1u32..=x)
    .map(|x| {
        let u = up(x);
        let d = down(x);
        let y = (u - d)/(u.abs() + d.abs());
        (x, y)
    })
    .collect();
    let max = points.iter().map(|(_, y)| *y).max_by(|y1, y2| y1.total_cmp(y2)).unwrap();
    let min = points.iter().map(|(_, y)| *y).min_by(|y1, y2| y1.total_cmp(y2)).unwrap();
    let root = BitMapBackend::new("output/a.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Error vs N", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d((1u32..x).log_scale(), (min..max).log_scale())
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("N")
        .y_desc("Error")
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    chart.draw_series(LineSeries::new(points, &RED)).unwrap();

    root.present().unwrap();
}
