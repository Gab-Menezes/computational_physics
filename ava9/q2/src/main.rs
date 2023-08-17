use std::fs::read_to_string;

use plotters::prelude::*;

struct Md {
    atoms: u32,
    temperature: f32,
    energy: f32
}

impl Md {
    fn new(atoms: u32, temperature: f32, energy: f32) -> Self {
        Self {
            atoms,
            temperature,
            energy
        }
    }
}

// a
// fn main() {
//     let runs = &[
//         Md::new(4u32, 0.000675f32, -1.436195f32),
//         Md::new(32u32, 0.006359f32, -6.570068f32),
//         Md::new(108u32, 0.000142f32, -7.450716f32),
//         Md::new(256u32,  0.000059f32, -7.463084f32),
//         Md::new(500u32,  0.000057f32, -7.463092f32),
//         Md::new(864u32,  0.000054f32, -7.463099f32),
//         Md::new(1372u32,  0.000056f32, -7.463094f32),
//     ];

//     {
//         let root = BitMapBackend::new("./out/a_temperature.png", (1920, 1080)).into_drawing_area();
//         root.fill(&WHITE).unwrap();
//         let mut chart = ChartBuilder::on(&root)
//             .caption(format!("Mean Temperature"), ("sans-serif", 30).into_font())
//             .margin(30)
//             .x_label_area_size(60)
//             .y_label_area_size(60)
//             .build_cartesian_2d((2u32..1500u32).log_scale(), 0.00005f32..0.007f32)
//             .unwrap();
        
//         chart
//             .configure_mesh()
//             .x_desc("#Atoms")
//             .y_desc("K")
//             .axis_desc_style(("sans-serif", 20))
//             .draw()
//             .unwrap();
        
//         chart.draw_series(LineSeries::new(
//             runs.iter().map(|r| (r.atoms, r.temperature)), RED)
//         )
//         .unwrap();
        
//         chart.draw_series(PointSeries::of_element(
//             runs.iter().map(|r| (r.atoms, r.temperature)),
//             3,
//             RED.filled(),
//             &|coord, size, style| {
//                 EmptyElement::at(coord)
//                     + Circle::new((0, 0), size, style)
//                     + Text::new(format!("{:?}", coord), (-35, -20), ("sans-serif", 13))
//             },
//         ))
//         .unwrap();
        
//         root.present().unwrap();
//     }

//     {
//         let root = BitMapBackend::new("./out/a_energy.png", (1920, 1080)).into_drawing_area();
//         root.fill(&WHITE).unwrap();
//         let mut chart = ChartBuilder::on(&root)
//             .caption(format!("Mean Energy"), ("sans-serif", 30).into_font())
//             .margin(30)
//             .x_label_area_size(60)
//             .y_label_area_size(60)
//             .build_cartesian_2d((2u32..1500u32).log_scale(), -8f32..0f32)
//             .unwrap();
        
//         chart
//             .configure_mesh()
//             .x_desc("#Atoms")
//             .y_desc("J")
//             .axis_desc_style(("sans-serif", 20))
//             .draw()
//             .unwrap();
        
//         chart.draw_series(LineSeries::new(
//             runs.iter().map(|r| (r.atoms, r.energy)), RED)
//         )
//         .unwrap();
        
//         chart.draw_series(PointSeries::of_element(
//             runs.iter().map(|r| (r.atoms, r.energy)),
//             3,
//             RED.filled(),
//             &|coord, size, style| {
//                 EmptyElement::at(coord)
//                     + Circle::new((0, 0), size, style)
//                     + Text::new(format!("{:?}", coord), (-35, -20), ("sans-serif", 13))
//             },
//         ))
//         .unwrap();
        
//         root.present().unwrap();
//     }
// }

// b
// fn main() {
//     let root_temperature = BitMapBackend::new("./out/b_temperature.png", (1920*2, 1080*2)).into_drawing_area();
//     root_temperature.fill(&WHITE).unwrap();
//     let root_temperature_area = root_temperature.titled("Temperature", ("sans-serif", 30)).unwrap();
//     let root_temperature_areas = root_temperature_area.split_evenly((4, 1));

//     let root_energy = BitMapBackend::new("./out/b_energy.png", (1920*2, 1080*2)).into_drawing_area();
//     root_energy.fill(&WHITE).unwrap();
//     let root_energy_area = root_energy.titled("Temperature", ("sans-serif", 30)).unwrap();
//     let root_energy_areas = root_energy_area.split_evenly((4, 1));

//     let files = &[
//         ("./b/o002", 0.002f32, "0.002"),
//         ("./b/o005", 0.005f32, "0.005"),
//         ("./b/o01", 0.01f32, "0.01"),
//         ("./b/o05", 0.05f32, "0.05"),
//     ];

//     for ((file, dt, caption), (temperature_area, energy_area)) in files.iter().zip(root_temperature_areas.iter().zip(root_energy_areas.iter())) {
//         let data = read_to_string(file).unwrap();
//         let data: Vec<_> = data.lines()
//         .filter_map(|line| {
//             if line.starts_with("#") {
//                 return None;
//             }
//             let mut it = line.split_ascii_whitespace();
//             let step: u32 = it.next().unwrap().parse().unwrap();
//             let temperature: f32 = it.next().unwrap().parse().unwrap();
//             let energy: f32 = it.skip(2).next().unwrap().parse().unwrap();
//             Some((step as f32 * dt, temperature, energy))
//         })
//         .collect();
        
//         let mut chart_temperature = ChartBuilder::on(&temperature_area)
//             .caption(caption, ("sans-serif", 30).into_font())
//             .x_label_area_size(60)
//             .y_label_area_size(100)
//             .margin(30)
//             .build_cartesian_2d(0f32..101f32, 0f32..0.00012f32)
//             .unwrap();
//         chart_temperature
//             .configure_mesh()
//             .x_desc("s")
//             .y_desc("K")
//             .axis_desc_style(("sans-serif", 40))
//             .draw()
//             .unwrap();
        
//         let mut chart_energy = ChartBuilder::on(&energy_area)
//             .caption(caption, ("sans-serif", 30).into_font())
//             .x_label_area_size(60)
//             .y_label_area_size(100)
//             .margin(30)
//             .build_cartesian_2d(0f32..101f32, -7.5f32..-7.45f32)
//             .unwrap();
//         chart_energy
//             .configure_mesh()
//             .x_desc("s")
//             .y_desc("J")
//             .axis_desc_style(("sans-serif", 40))
//             .draw()
//             .unwrap();


//         chart_temperature.draw_series(LineSeries::new(
//             data.iter().map(|d| (d.0, d.1)), RED)
//         )
//         .unwrap();
//         chart_temperature.draw_series(PointSeries::of_element(
//             data.iter().map(|d| (d.0, d.1)),
//             0.5f32,
//             RED.filled(),
//             &|coord, size, style| Circle::new(coord, size, style),
//         ))
//         .unwrap();

//         chart_energy.draw_series(LineSeries::new(
//             data.iter().map(|d| (d.0, d.2)), RED)
//         )
//         .unwrap();
//         chart_energy.draw_series(PointSeries::of_element(
//             data.iter().map(|d| (d.0, d.2)),
//             0.5f32,
//             RED.filled(),
//             &|coord, size, style| Circle::new(coord, size, style),
//         ))
//         .unwrap();
//     }

//     root_temperature.present().unwrap();
//     root_energy.present().unwrap();
// }

// c
fn main() {
    let root_temperature = BitMapBackend::new("./out/c_temperature.png", (1920*2, 1080*2)).into_drawing_area();
    root_temperature.fill(&WHITE).unwrap();
    let root_temperature_area = root_temperature.titled("Temperature", ("sans-serif", 30)).unwrap();
    let root_temperature_areas = root_temperature_area.split_evenly((4, 1));

    let root_energy = BitMapBackend::new("./out/c_energy.png", (1920*2, 1080*2)).into_drawing_area();
    root_energy.fill(&WHITE).unwrap();
    let root_energy_area = root_energy.titled("Energy", ("sans-serif", 30)).unwrap();
    let root_energy_areas = root_energy_area.split_evenly((4, 1));

    let files = &[
        ("./c/o07", 0.0025f32, "0.7"),
        ("./c/o08", 0.0025f32, "0.8"),
        ("./c/o095", 0.0025f32, "0.95"),
        ("./c/o11", 0.0025f32, "1.1"),
    ];

    for ((file, dt, caption), (temperature_area, energy_area)) in files.iter().zip(root_temperature_areas.iter().zip(root_energy_areas.iter())) {
        let data = read_to_string(file).unwrap();
        let data: Vec<_> = data.lines()
        .filter_map(|line| {
            if line.starts_with("#") {
                return None;
            }
            let mut it = line.split_ascii_whitespace();
            let step: u32 = it.next().unwrap().parse().unwrap();
            let temperature: f32 = it.next().unwrap().parse().unwrap();
            let energy: f32 = it.skip(2).next().unwrap().parse().unwrap();
            Some((step as f32 * dt, temperature, energy))
        })
        .collect();
        
        let mut chart_temperature = ChartBuilder::on(&temperature_area)
            .caption(caption, ("sans-serif", 30).into_font())
            .x_label_area_size(60)
            .y_label_area_size(100)
            .margin(30)
            .build_cartesian_2d(0f32..51f32, 0f32..1.2f32)
            .unwrap();
        chart_temperature
            .configure_mesh()
            .x_desc("s")
            .y_desc("K")
            .axis_desc_style(("sans-serif", 40))
            .draw()
            .unwrap();
        
        let mut chart_energy = ChartBuilder::on(&energy_area)
            .caption(caption, ("sans-serif", 30).into_font())
            .x_label_area_size(60)
            .y_label_area_size(100)
            .margin(30)
            .build_cartesian_2d(0f32..51f32, -7f32..-3f32)
            .unwrap();
        chart_energy
            .configure_mesh()
            .x_desc("s")
            .y_desc("J")
            .axis_desc_style(("sans-serif", 40))
            .draw()
            .unwrap();


        chart_temperature.draw_series(LineSeries::new(
            data.iter().map(|d| (d.0, d.1)), RED)
        )
        .unwrap();
        chart_temperature.draw_series(PointSeries::of_element(
            data.iter().map(|d| (d.0, d.1)),
            0.5f32,
            RED.filled(),
            &|coord, size, style| Circle::new(coord, size, style),
        ))
        .unwrap();

        chart_energy.draw_series(LineSeries::new(
            data.iter().map(|d| (d.0, d.2)), RED)
        )
        .unwrap();
        chart_energy.draw_series(PointSeries::of_element(
            data.iter().map(|d| (d.0, d.2)),
            0.5f32,
            RED.filled(),
            &|coord, size, style| Circle::new(coord, size, style),
        ))
        .unwrap();
    }

    root_temperature.present().unwrap();
    root_energy.present().unwrap();

    let runs = &[
        (0.7f32, -5.887193f32),
        (0.8f32, -5.765684f32),
        (0.95f32, -5.552616f32),
        (1f32,  -5.160293f32),
        (1.05f32,  -5.087878f32),
        (1.1f32,  -5.032189f32),
    ];

    {
        let root = BitMapBackend::new("./out/c_potential.png", (1920, 1080)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption(format!("Mean Potential"), ("sans-serif", 30).into_font())
            .margin(30)
            .x_label_area_size(60)
            .y_label_area_size(60)
            .build_cartesian_2d(0.5f32..1.2f32, -6f32..-5f32)
            .unwrap();
        
        chart
            .configure_mesh()
            .x_desc("K")
            .y_desc("J")
            .axis_desc_style(("sans-serif", 20))
            .draw()
            .unwrap();
        
        chart.draw_series(LineSeries::new(
            runs.iter().map(|r| (r.0, r.1)), RED)
        )
        .unwrap();
        
        chart.draw_series(PointSeries::of_element(
            runs.iter().map(|r| (r.0, r.1)),
            3,
            RED.filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(format!("{:?}", coord), (-35, -20), ("sans-serif", 13))
            },
        ))
        .unwrap();
        
        root.present().unwrap();
    }
}