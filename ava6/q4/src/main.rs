use std::{
    f64::consts::{E, PI},
    ops::Range,
};

use plotters::prelude::*;

fn w(t: f64, w: f64) -> f64 {
    const K: f64 = 1.3806e-23;
    const C: f64 = 299_792_458f64;
    const H_: f64 = 1.054571e-34;
    let const_part = (K.powi(4) * t.powi(4)) / (4f64 * PI.powi(2) * C.powi(2) * H_.powi(3));
    const_part * w.powi(3) / (E.powf(w) - 1f64)
}

fn change_varible(g: impl Fn(f64) -> f64, z: f64) -> f64 {
    1f64 / (1f64 - z).powi(2) * g(z / (1f64 - z))
}

fn simpson(g: impl Fn(f64) -> f64, x_range: Range<f64>, slices: u32) -> f64 {
    let step = (x_range.end - x_range.start) / slices as f64;
    let start = g(x_range.start) + g(x_range.end);
    let sum = (1..slices).fold(start, |acc, i| {
        let fac = if i % 2 == 0 { 2f64 } else { 4f64 };
        acc + fac * g(x_range.start + i as f64 * step)
    });
    sum * step * 1f64 / 3f64
}

// b
fn main() {
    const T: f64 = 500f64;
    let temperature_w = |x| w(T, x);
    let changed_variable = |x| change_varible(temperature_w, x);
    let v = simpson(changed_variable, 0f64..1f64, 10_000);
    println!("{v}");
}
