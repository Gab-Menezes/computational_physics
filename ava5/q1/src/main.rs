use std::{f32::consts::PI, ops::Range};

use plotters::prelude::*;

fn q1(x: f32) -> f32 {
    x * x - 2f32 * x + 1f32
}

fn integral_q1(x: f32) -> f32 {
    (x * x * x) / 3f32 - x * x + x
}

fn q2(x: f32) -> f32 {
    x.powi(4) - 2f32 * x + 1f32
}

fn q3(x: f32) -> f32 {
    (200f32*x).sqrt().sin().powi(2)
}

fn q4(x: f32) -> f32 {
    (100f32*x).sqrt().cos().powi(2)
}

fn plot(g: impl Fn(f32) -> f32, x_range: Range<f32>, name: &str, title: &str) {
    let mut y_min = f32::MAX;
    let mut y_max = f32::MIN;
    let points: Vec<_> = x_range
        .clone()
        .step(0.1f32)
        .values()
        .map(|x| {
            let y = g(x);
            y_max = y_max.max(y);
            y_min = y_min.min(y);
            (x, y)
        })
        .collect();

    let name = format!("output/{name}.png");
    let root = BitMapBackend::new(&name, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range, y_min..y_max)
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

fn trapezoid(g: impl Fn(f32) -> f32, x_range: Range<f32>, slices: u32) -> f32 {
    let step = (x_range.end - x_range.start) / slices as f32;
    let start = (g(x_range.start) + g(x_range.end)) / 2f32;
    let sum = (1..slices).fold(start, |acc, i| {
        let a = g(x_range.start + i as f32 * step);
        acc + a
    });
    sum * step
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

fn adaptive_simpson(
    g: impl Fn(f32) -> f32,
    x_range: Range<f32>,
    mut slices: u32,
    precision: f32,
) -> f32 {
    let step = (x_range.end - x_range.start) / slices as f32;
    let mut s = 0f32;
    let mut t = 0f32;
    for i in 1..slices {
        let y = g(x_range.start + i as f32 * step);
        if i % 2 == 0 {
            s += y;
        } else {
            t += y;
        }
    }

    s = 1f32 / 3f32 * (g(x_range.start) + g(x_range.end) + 2f32 * s);
    t *= 2f32 / 3f32;
    let mut result = step * (s + 2f32 * t);

    println!("slices: {slices:4} | e: {:10} | adaptive simpson: {result:4.8}", "");

    loop {
        slices *= 2;
        let step = (x_range.end - x_range.start) / slices as f32;
        let new_s = s + t;
        let new_t = 2f32 / 3f32
            * (1..slices)
                .step_by(2)
                .map(|i| g(x_range.start + i as f32 * step))
                .sum::<f32>();
        let new_result = step * (new_s + 2f32 * new_t);
        let epsilon = ((new_result - result) / 15f32).abs();

        println!("slices: {slices:4} | e: {epsilon:.8} | adaptive simpson: {new_result:4.8}");

        if epsilon < precision {
            return new_result;
        }

        s = new_s;
        t = new_t;
        result = new_result;
    }
}

fn print_row(i: u32, vs: &[f32]) {
    print!("{i:3}: ");
    for v in vs {
        print!("{v:4.8} ");
    }
    println!("");
}

fn romberg(g: &impl Fn(f32) -> f32, x_range: Range<f32>, precision: f32) -> f32 {
    let mut last_buffer = vec![trapezoid(g, x_range.clone(), 1)];
    print_row(0, &last_buffer);

    let mut slices = 1;
    for i in 1.. {
        slices *= 2;
        let mut current_buffer = vec![trapezoid(g, x_range.clone(), slices)];
        for (m, last_r) in last_buffer.iter().enumerate() {
            let m = (m + 1) as i32;
            let prev_r = current_buffer.last().unwrap();
            let r = prev_r + (prev_r - last_r)/(4f32.powi(m) - 1f32);
            current_buffer.push(r);
        }

        print_row(i, &current_buffer);

        let m = current_buffer.len() as i32;
        let last_r = last_buffer.last().unwrap();
        let current_r = current_buffer.last().unwrap();
        let epsilon = ((current_r - last_r)/(4f32.powi(m) - 1f32)).abs();
        if epsilon < precision {
            return *current_r;
        }
        last_buffer = current_buffer;
    }

    *last_buffer.last().unwrap()
}

// q1.a
// fn main() {
//     let x_range = -10f32..10.1f32;
//     plot(q1, x_range.clone(), "f", "x^2 - 2x + 1");
//     plot(
//         integral_q1,
//         x_range.clone(),
//         "integral_f",
//         "(x^3)/3 - x^2 + x",
//     );
// }

// q1.b,c,d
// fn main() {
//     let actual = 2f32 / 3f32;
//     let x_range = 0f32..2f32;
//     for slices in &[10u32, 100, 1000] {
//         let trapezoid = trapezoid(q1, x_range.clone(), *slices);
//         let simp = simpson(q1, x_range.clone(), *slices);
//         println!(
//             "slices: {slices:3} | e: {:.8} | trapezoid: {trapezoid}",
//             trapezoid - actual
//         );
//         println!(
//             "slices: {slices:3} | e: {:.8} | simpson: {simp}",
//             simp - actual
//         );
//         println!("----------------");
//     }
// }

// q1.e
// fn main() {
//     let x_range = 0f32..2f32;
//     adaptive_simpson(q1, x_range, 2, 1e-6);
// }

// q2.b.a,b,c
// fn main() {
//     let actual = 4.4f32;
//     let x_range = 0f32..2f32;
//     for slices in &[10u32, 100, 1000] {
//         let trapezoid = trapezoid(q2, x_range.clone(), *slices);
//         let simp = simpson(q2, x_range.clone(), *slices);
//         println!(
//             "slices: {slices:3} | e: {:.8} | trapezoid: {trapezoid}",
//             trapezoid - actual
//         );
//         println!(
//             "slices: {slices:3} | e: {:.8} | simpson: {simp}",
//             simp - actual
//         );
//         println!("----------------");
//     }
// }

// q3.a,b,c
// fn main() {
//     let actual = 0.50285f32;
//     let x_range = 0f32..1f32;
//     for slices in &[10u32, 100, 1000] {
//         let trapezoid = trapezoid(q3, x_range.clone(), *slices);
//         let simp = simpson(q3, x_range.clone(), *slices);
//         println!(
//             "slices: {slices:3} | e: {:.8} | trapezoid: {trapezoid}", trapezoid - actual
//         );
//         println!(
//             "slices: {slices:3} | e: {:.8} | simpson: {simp}", simp - actual
//         );
//         println!("----------------");
//     }
// }

// q3.d
// fn main() {
//     let x_range = 0f32..1f32;
//     adaptive_simpson(q3, x_range, 2, 1e-6);
// }

// q3.e
// fn main() {
//     let x_range = 0f32..1f32;
//     romberg(&q3, x_range, 1e-6);
// }

// q4.a,b,c
// fn main() {
//     let actual = 0.54417f32;
//     let x_range = 0f32..1f32;
//     for slices in &[10u32, 100, 1000] {
//         let trapezoid = trapezoid(q4, x_range.clone(), *slices);
//         let simp = simpson(q4, x_range.clone(), *slices);
//         println!(
//             "slices: {slices:3} | e: {:.8} | trapezoid: {trapezoid}", trapezoid - actual
//         );
//         println!(
//             "slices: {slices:3} | e: {:.8} | simpson: {simp}", simp - actual
//         );
//         println!("----------------");
//     }
// }

// q4.d
// fn main() {
//     let x_range = 0f32..1f32;
//     adaptive_simpson(q4, x_range, 2, 1e-6);
// }

// q4.e
fn main() {
    let x_range = 0f32..1f32;
    romberg(&q4, x_range, 1e-6);
}