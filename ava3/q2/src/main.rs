use plotters::prelude::*;

// a
fn main() {
    let mut points = Vec::new();
    for r in 100..400u32 {
        let r = r as f32 * 0.01f32;
        let mut x = 0.5f32;
        for _ in 0..1000 {
            x = r * x * (1f32 - x);
        }
        for _ in 0..1000 {
            x = r * x * (1f32 - x);
            points.push(Circle::new((r, x), 1, RED.filled()));
        }
    }

    let root = BitMapBackend::new("output/a.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Feigenbaum", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(1f32..4.05f32, 0f32..1f32)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("r")
        .y_desc("x")
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    chart.draw_series(points).unwrap();

    root.present().unwrap();
}
