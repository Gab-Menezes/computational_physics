#![feature(array_windows)]
use chrono::{prelude::*, Months};
use plotters::prelude::*;
use std::fs::read_to_string;

// a
// fn main() {
//     let begin_date = NaiveDate::from_ymd_opt(1749, 1, 1).unwrap();
//     let data = read_to_string("sunspots.txt").unwrap();
//     let mut max_pos = 0f32;
//     let parsed: Vec<_> = data
//         .lines()
//         .map(|line| {
//             let mut it = line.split_whitespace();
//             let month: u32 = it.next().unwrap().parse().unwrap();
//             let date = begin_date + Months::new(month);

//             let pos: f32 = it.next().unwrap().parse().unwrap();
//             max_pos = max_pos.max(pos);

//             (date, pos)
//         })
//         .collect();

//     let x_range = (parsed.first().unwrap().0..parsed.last().unwrap().0).monthly();
//     let y_range = 0f32..(max_pos + 50f32);
//     let root = BitMapBackend::new("output/a.png", (1920, 1080)).into_drawing_area();
//     root.fill(&WHITE).unwrap();
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Sun Spot", ("sans-serif", 30).into_font())
//         .margin(10)
//         .x_label_area_size(60)
//         .y_label_area_size(60)
//         .build_cartesian_2d(x_range, y_range)
//         .unwrap();

//     chart
//         .configure_mesh()
//         .x_desc("Date")
//         .y_desc("Spot")
//         .axis_desc_style(("sans-serif", 20))
//         .draw()
//         .unwrap();

//     chart.draw_series(LineSeries::new(parsed, &RED)).unwrap();

//     root.present().unwrap();
// }

//b
fn main() {
    let begin_date = NaiveDate::from_ymd_opt(1749, 1, 1).unwrap();
    let data = read_to_string("sunspots.txt").unwrap();
    let parsed: Vec<_> = data
        .lines()
        .take(1000)
        .map(|line| {
            let mut it = line.split_whitespace();
            let month: u32 = it.next().unwrap().parse().unwrap();
            let date = begin_date + Months::new(month);

            let pos: f32 = it.next().unwrap().parse().unwrap();

            (date, pos)
        })
        .collect();

    let x_range = (parsed.first().unwrap().0..parsed.last().unwrap().0).step(Months::new(3));
    let y_range = 0f32..300f32;
    let root = BitMapBackend::new("output/b.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Sun Spot", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range, y_range)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Date")
        .y_desc("Spot")
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    chart.draw_series(LineSeries::new(parsed, &RED)).unwrap();

    root.present().unwrap();
}

//c
// fn main() {
//     let begin_date = NaiveDate::from_ymd_opt(1749, 1, 1).unwrap();
//     let data = read_to_string("sunspots.txt").unwrap();
//     let parsed = data
//         .lines()
//         .take(1000)
//         .map(|line| {
//             let mut it = line.split_whitespace();
//             let month: u32 = it.next().unwrap().parse().unwrap();
//             let pos: f32 = it.next().unwrap().parse().unwrap();
//             (month, pos)
//         })
//         .collect::<Vec<_>>()
//         .array_windows::<11>()
//         .map(|window| {
//             let date = begin_date + Months::new(window[5].0);
//             let mean = window.iter().map(|e| e.1).sum::<f32>() / 10f32;
//             (date, mean)
//         })
//         .collect::<Vec<_>>();

//     let x_range = (parsed.first().unwrap().0..parsed.last().unwrap().0).monthly();
//     let y_range = 0f32..300f32;
//     let root = BitMapBackend::new("output/c.png", (1920, 1080)).into_drawing_area();
//     root.fill(&WHITE).unwrap();
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Sun Spot", ("sans-serif", 30).into_font())
//         .margin(10)
//         .x_label_area_size(60)
//         .y_label_area_size(60)
//         .build_cartesian_2d(x_range, y_range)
//         .unwrap();

//     chart
//         .configure_mesh()
//         .x_desc("Date")
//         .y_desc("Spot")
//         .axis_desc_style(("sans-serif", 20))
//         .draw()
//         .unwrap();

//     chart.draw_series(LineSeries::new(parsed, &RED)).unwrap();

//     root.present().unwrap();
// }
