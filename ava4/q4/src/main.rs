pub fn factorial(num: u128) -> u128 {
    match num {
        0 => 1,
        _ => (1..=num).product(),
    }
}

fn sin(x: f64) -> f64 {
    let mut s = 0f64;
    for i in 0u128..10 {
        let n = i as f64;
        let v = ((-1f64).powf(n) * x.powf(2f64 * n + 1f64)) / factorial(2u128 * i + 1u128) as f64;
        if v.abs() <= s * 1e-7 {
            break;
        }
        s += v;
    }
    s
}

fn main() {
    let approx = sin(0.1f64);
    let actual = 0.1f64.sin();
    println!("approximation: {approx}");
    println!("actual: {actual}");
    println!("diff: {}", actual - approx);
}
