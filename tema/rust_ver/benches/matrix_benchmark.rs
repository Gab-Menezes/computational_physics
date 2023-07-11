use criterion::{criterion_group, criterion_main, Criterion};
use rust_ver::matrix::{Matrix, StateDefault, StateParallel, StateParallelBuffer, StateParallelSimd};
use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    const PAR: usize = 8;
    let mut rng = rand::thread_rng();
    let vals = [0f32, 0.5f32, 1.0f32, 1.5f32];
    let sizes = [16usize, 32, 64, 128, 256, 512, 1024, 2048];
    let samples = [None, None, None, None, None, None, Some(15usize), Some(15usize)];
    for (size, sample) in sizes.into_iter().zip(samples) {
        let mut m1 = Matrix::<StateDefault>::zeros(size, size);
        let mut m2 = Matrix::<StateDefault>::zeros(size, size);
        for i in 0..m1.rows() {
            for j in 0..m1.cols() {
                m1[(i, j)] = *vals.choose(&mut rng).unwrap();
                m2[(i, j)] = *vals.choose(&mut rng).unwrap();
            }
        }
        let mut g = c.benchmark_group(format!("{size}/matrix_mult"));
        if sample.is_some() {
            g.sample_size(sample.unwrap());
        }
    
        g.bench_function("default", |b| b.iter(|| m1.mul(&m2)));
        
        let m1 = m1.morph::<StateParallel>();
        let m2 = m2.morph::<StateParallel>();
        g.bench_function("parallel", |b| b.iter(|| m1.mul::<PAR>(&m2)));
    
        let m1 = m1.morph::<StateParallelBuffer>();
        let m2 = m2.morph::<StateParallelBuffer>();
        g.bench_function("parallel_buffer", |b| b.iter(|| m1.mul::<PAR>(&m2)));
    
        let m1 = m1.morph::<StateParallelSimd>();
        let m2 = m2.morph::<StateParallelSimd>();
        g.bench_function("parallel_simd", |b| b.iter(|| m1.mul::<PAR>(&m2)));
    }

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);