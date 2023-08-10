use ndarray::prelude::*;
use ndarray_linalg::*;
use std::{
    f64::consts::{E, PI},
    ops::Range,
};

fn f(x: f64, y: f64, z: f64) -> f64 {
    (x.sin() + y.cos())*z.exp()
}

fn gaussxw(n: usize) -> (Array1<f64>, Array1<f64>) {
    let a = Array1::linspace(3.0, 4.0 * n as f64 - 1.0, n) / (4.0 * n as f64 + 2.0);
    let mut x =
        (PI * &a + 1f64 / (8f64 * n as f64 * n as f64 * a.mapv_into(f64::tan))).mapv_into(f64::cos);

    let epsilon = 1e-15f64;
    let mut delta = 1f64;
    let mut last_dp = None;
    while delta > epsilon {
        let mut p0 = Array1::ones(n);
        let mut p1 = x.clone();
        for k in 1..n {
            let temp = p1.clone();
            p1 = ((2f64 * k as f64 + 1f64) * &x * p1 - k as f64 * p0) / (k as f64 + 1f64);
            p0 = temp;
        }
        let dp = (n as f64 + 1f64) * (&p0 - &x * &p1) / (1f64 - &x * &x);
        let dx = &p1 / &dp;
        last_dp = Some(dp);
        x = x - &dx;
        delta = dx
            .iter()
            .map(|v| v.abs())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();
    }

    let dp = last_dp.unwrap();
    let w = 2.0 * (n as f64 + 1.0) * (n as f64 + 1.0)
        / (n as f64 * n as f64 * (1.0 - &x * &x) * &dp * &dp);

    (x, w)
}

fn gaussxwab(n: usize, range: Range<f64>) -> (Array1<f64>, Array1<f64>) {
    let a = range.start;
    let b = range.end;
    let (x, w) = gaussxw(n);
    let x_mapped = 0.5f64 * (b - a) * &x + 0.5f64 * (b + a);
    let w_scaled = 0.5f64 * (b - a) * &w;
    (x_mapped, w_scaled)
}

fn main() {
    const N: usize = 100;
    const E_0: f64 = 8.854187e-12f64;

    let target_point = array![3.1415f64, 2.7182f64, 1.6180f64];
    let x_range = -0.5f64..0.5f64;
    let y_range = -0.7f64..0.7f64;
    let z_range = -0.9f64..0.9f64;

    let (xp, xwp) = gaussxwab(N, x_range);
    let (yp, ywp) = gaussxwab(N, y_range);
    let (zp, zwp) = gaussxwab(N, z_range);
    let mut electric_field_vector = Array1::<f64>::zeros(3);
    let mut cube_charge = 0f64;
    for (x, xw) in xp.iter().zip(xwp) {
        for (y, yw) in yp.iter().zip(ywp.iter()) {
            for (z, zw) in zp.iter().zip(zwp.iter()) {
                let charge = xw * yw * zw * f(*x, *y, *z);
                cube_charge += charge;
                let current_point = array![*x, *y, *z];
                let dir_vec = &target_point - current_point;
                let mag = dir_vec.norm();
                let r = dir_vec / mag;
                electric_field_vector = electric_field_vector + charge / mag.powi(2) * r;
            }
        }
    }
    electric_field_vector = electric_field_vector / (4f64 * PI * E_0);

    let electric_field = electric_field_vector.norm();
    let theta = (electric_field_vector[2] / electric_field).acos() * 180f64 / PI;
    let phi = (electric_field_vector[1] / electric_field_vector[0]).atan() * 180f64 / PI;
    println!("Cube Charge:           {cube_charge:.5} C");
    println!("Electric Field Vector: {electric_field_vector:.5e}");
    println!("Electric Field Mag:    {electric_field:.5e} N/C");
    println!("phi:                   {phi:.5} degrees");
    println!("theta:                 {theta:3.5} degrees");
}
