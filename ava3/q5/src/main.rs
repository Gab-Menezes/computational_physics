#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(iter_array_chunks)]
#![feature(array_chunks)]
#![feature(portable_simd)]
#![feature(array_zip)]

use plotters::prelude::*;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{
    arch::x86_64::{__m256i, _mm256_set1_epi32, _mm256_testz_si256},
    simd::{f32x8, u32x8, SimdPartialOrd},
    time::Instant,
};

fn main() {
    let root = BitMapBackend::new("output/mandel.png", (1920, 1080)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Mandelbrot set", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(-2f32..2f32, -2f32..2f32)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .unwrap();

    let plotting_area = chart.plotting_area();
    let range = plotting_area.get_pixel_range();

    let (width, height) = (range.0.end - range.0.start, range.1.end - range.1.start);
    let (real, complex) = (chart.x_range(), chart.y_range());

    let step = (
        (real.end - real.start) / width as f32,
        (complex.end - complex.start) / height as f32,
    );

    const MAX_ITERS: u32 = 8_192u32;
    const MAX_MAG: f32 = 2f32;
    const LANES: usize = 8;

    let begin = Instant::now();
    let step1 = f32x8::splat(step.1);
    let complex_start = f32x8::splat(complex.start);
    let max_mag = f32x8::splat(MAX_MAG);

    let pixels = (0..(width as u32))
        .into_par_iter()
        .map(|i| {
            let mut pixels = Vec::with_capacity(height as usize);
            let c_real = real.start + step.0 * i as f32;
            let c_real_simd = f32x8::splat(c_real);
            let mut it = (0..(height as u32)).array_chunks::<LANES>();
            for j in it.by_ref() {
                let j = u32x8::from_array(j).cast::<f32>();
                let c = (c_real_simd, complex_start + step1 * j);

                let mut z = (f32x8::splat(0f32), f32x8::splat(0f32));
                let mut iters = u32x8::splat(0u32);
                let mut iter_mask = u32x8::splat(1u32);
                for _ in 0..MAX_ITERS {
                    let z02 = z.0 * z.0;
                    let z12 = z.1 * z.1;
                    let z01 = z.0 * z.1;
                    z = (z02 - z12 + c.0, z01 + z01 + c.1);
                    let mag = z02 + z12;

                    iters += iter_mask;
                    iter_mask &= mag.simd_le(max_mag).to_int().cast();

                    let stop: i32 = unsafe {
                        _mm256_testz_si256(__m256i::from(iter_mask), _mm256_set1_epi32(-1))
                    };
                    if stop == 1i32 {
                        break;
                    }
                }
                let x = c.0.to_array();
                let y = c.1.to_array();
                let iters = iters.to_array();
                for v in x.zip(y).zip(iters) {
                    pixels.push(v);
                }
            }

            let Some(remainder) = it.into_remainder() else { return pixels; };

            for j in remainder {
                let c = (c_real, complex.start + step.1 * j as f32);
                let mut z = (0.0f32, 0.0f32);
                let mut mag = 0f32;
                let mut iters = 0u32;
                while mag <= MAX_MAG && iters < MAX_ITERS {
                    let z02 = z.0 * z.0;
                    let z12 = z.1 * z.1;
                    let z01 = z.0 * z.1;
                    z = (z02 - z12 + c.0, z01 + z01 + c.1);
                    mag = z02 + z12;
                    iters += 1;
                }

                pixels.push((c, iters));
            }
            pixels
        })
        .collect::<Vec<_>>();

    for (coord, iters) in pixels.into_iter().flatten() {
        let c: u8 = if iters != MAX_ITERS { 255 } else { 0 };
        let c = RGBColor(c, c, c);
        plotting_area.draw_pixel(coord, &c).unwrap();
    }
    println!("{:?}", begin.elapsed().as_millis());

    root.present().unwrap();
}
