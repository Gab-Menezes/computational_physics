fn f(x: f64) -> f64 {
    x * (x - 1f64)
}

fn main() {
    let d = [1e-2, 1e-4, 1e-6, 1e-8, 1e-10, 1e-12, 1e-14, 1e-15];
    let x = 1f64;
    for d in d {
        let df = (f(x + d) - f(x)) / d;
        println!("{d}: {df}");
    }
}
