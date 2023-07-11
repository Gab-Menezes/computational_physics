use rayon::prelude::*;
use std::{
    arch::x86_64::{
        _mm256_add_ps, _mm256_loadu_ps, _mm256_mul_ps,
        _mm256_set1_ps, _mm256_setzero_ps, _mm256_storeu_ps,
    },
    cmp::min,
    marker::PhantomData,
    ops::{Index, IndexMut, Mul},
};
pub struct MatrixRef<'a, T> {
    v: &'a [f32],
    row: usize,
    col: usize,
    phantom: PhantomData<T>,
}

impl<'a, T> MatrixRef<'a, T> {
    pub fn new(v: &'a [f32], row: usize, col: usize) -> Self {
        assert_eq!(v.len(), row * col);
        Self {
            v,
            row,
            col,
            phantom: PhantomData,
        }
    }

    pub fn rows(&self) -> usize {
        self.row
    }

    pub fn cols(&self) -> usize {
        self.col
    }

    pub fn mul_buffered(&self, rhs: &Self, buffer: &mut [f32]) {
        let m = self.cols();
        assert_eq!(m, rhs.rows());
        assert_eq!(buffer.len(), self.rows() * rhs.cols());

        let n = self.rows();
        let p = rhs.cols();

        for i in 0..n {
            for j in 0..p {
                let mut s = 0f32;
                for k in 0..m {
                    s += self[(i, k)] * rhs[(k, j)];
                }
                buffer[i * rhs.col + j] = s;
            }
        }
    }
}

impl<'a, T> Index<(usize, usize)> for MatrixRef<'a, T> {
    type Output = f32;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.v[idx.0 * self.col + idx.1]
    }
}

impl<'a, T> Mul<Self> for &MatrixRef<'a, T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols(), rhs.rows());
        
        let m = self.cols();
        let n = self.rows();
        let p = rhs.cols();
        let mut out = Self::Output::zeros(n, p);
        for i in 0..n {
            for j in 0..p {
                let mut s = 0f32;
                for k in 0..m {
                    s += self[(i, k)] * rhs[(k, j)];
                }
                out[(i, j)] = s;
            }
        }
        out
    }
}

pub struct StateDefault;
pub struct StateParallel;
pub struct StateParallelBuffer;
pub struct StateParallelSimd;

pub struct Matrix<T> {
    v: Vec<f32>,
    row: usize,
    col: usize,
    phantom: PhantomData<T>,
}

impl<T> Matrix<T> {
    pub fn new(v: Vec<f32>, row: usize, col: usize) -> Self {
        assert_eq!(v.len(), row * col);
        Self {
            v,
            row,
            col,
            phantom: PhantomData,
        }
    }

    pub fn zeros(row: usize, col: usize) -> Self {
        Self {
            v: vec![0f32; row * col],
            // v: unsafe { zeroed_aligned_vec(row * col) },
            row,
            col,
            phantom: PhantomData,
        }
    }

    pub fn rows(&self) -> usize {
        self.row
    }

    pub fn cols(&self) -> usize {
        self.col
    }

    pub fn as_ref<'a>(&'a self) -> MatrixRef<'a, T> {
        MatrixRef {
            v: &self.v,
            row: self.row,
            col: self.col,
            phantom: PhantomData,
        }
    }

    pub fn into_inner(self) -> Vec<f32> {
        self.v
    }

    pub fn morph<U>(self) -> Matrix<U> {
        Matrix::<U> {
            v: self.v,
            row: self.row,
            col: self.col,
            phantom: PhantomData,
        }
    }

    pub fn calc_idx(&self, idx: (usize, usize)) -> usize {
        idx.0 * self.col + idx.1
    }

    pub fn as_slice(&self, idx: (usize, usize), n: usize) -> &[f32] {
        let idx = self.calc_idx(idx);
        &self.v[idx..(idx+n)]
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = f32;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let idx = self.calc_idx(idx);
        &self.v[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let idx = self.calc_idx(idx);
        &mut self.v[idx]
    }
}

impl Matrix<StateDefault> {
    pub fn mul(&self, rhs: &Self) -> Matrix<StateDefault> {
        let ref_self = self.as_ref();
        let ref_rhs = rhs.as_ref();
        &ref_self * &ref_rhs
    }
}

impl Matrix<StateParallel> {
    pub fn mul<const PAR: usize>(&self, rhs: &Self) -> Self {
        assert_eq!(self.cols(), rhs.rows());

        let ref_rhs = rhs.as_ref();
        let rows_per_iter = (self.rows() as f32 / PAR as f32).ceil() as usize;
        let out = self
            .v
            .par_chunks(rows_per_iter * self.cols())
            .map(|chunk| {
                let row = chunk.len() / self.cols();
                let ref_m = MatrixRef::new(chunk, row, self.cols());
                (&ref_m * &ref_rhs).into_inner()
            })
            .flatten()
            .collect::<Vec<_>>();
        Self::new(out, self.rows(), rhs.cols())
    }
}

impl Matrix<StateParallelBuffer> {
    pub fn mul<const PAR: usize>(&self, rhs: &Self) -> Self {
        assert_eq!(self.cols(), rhs.rows());

        let ref_rhs = rhs.as_ref();
        let rows_per_iter = (self.rows() as f32 / PAR as f32).ceil() as usize;

        let mut out = vec![0f32; self.rows() * rhs.cols()];
        self.v
            .par_chunks(rows_per_iter * self.cols())
            .zip_eq(out.par_chunks_mut(rows_per_iter * rhs.cols()))
            .for_each(|(chunk, buffer)| {
                let row = chunk.len() / self.cols();
                let ref_m = MatrixRef::new(chunk, row, self.cols());
                ref_m.mul_buffered(&ref_rhs, buffer);
            });
        Self::new(out, self.rows(), rhs.cols())
    }
}

impl Matrix<StateParallelSimd> {
    pub fn mul<const PAR: usize>(&self, rhs: &Self) -> Self {
        assert_eq!(self.rows() % (2 * PAR), 0);
        assert_eq!(rhs.cols() % 16, 0);
        assert_eq!(self.cols(), rhs.rows());

        let m = self.cols();
        let n = self.rows();
        let p = rhs.cols();
        let jb = min(512usize, p);
        let kb = min(24usize, m);
        let mut result = vec![0f32; n * p];

        let rows_per_iter = self.rows()/PAR;
        self.v
        .par_chunks_exact(rows_per_iter * self.cols())
        .zip_eq(result.par_chunks_exact_mut(rows_per_iter * rhs.cols()))
        .for_each(|(chunk, buffer)| {
            let n = rows_per_iter;

            for jj in (0..p).step_by(jb) {
                for kk in (0..m).step_by(kb) {
                    for i in (0..n).step_by(2) {
                        for j in (jj..(jj + jb)).step_by(16) {
                            let idx_11 = i       * p + j;
                            let idx_12 = i       * p + j + 8;
                            let idx_21 = (i + 1) * p + j;
                            let idx_22 = (i + 1) * p + j + 8;

                            unsafe {
                                let (mut sum_11, mut sum_12, mut sum_21, mut sum_22) = if kk == 0 {
                                    (
                                        _mm256_setzero_ps(), 
                                        _mm256_setzero_ps(),
                                        _mm256_setzero_ps(),
                                        _mm256_setzero_ps(),
                                    )
                                } else {
                                    (
                                        _mm256_loadu_ps(buffer[idx_11..(idx_11 + 8)].as_ptr()),
                                        _mm256_loadu_ps(buffer[idx_12..(idx_12 + 8)].as_ptr()),
                                        _mm256_loadu_ps(buffer[idx_21..(idx_21 + 8)].as_ptr()),
                                        _mm256_loadu_ps(buffer[idx_22..(idx_22 + 8)].as_ptr()),
                                    )
                                };

                                for k in kk..min(m, kk + kb) {
                                    let rhs_1 = _mm256_loadu_ps(rhs.as_slice((k, j    ), 8).as_ptr());
                                    let rhs_2 = _mm256_loadu_ps(rhs.as_slice((k, j + 8), 8).as_ptr());
    
                                    let bc = _mm256_set1_ps(chunk[i * self.cols() + k]);
                                    sum_11 = _mm256_add_ps(sum_11, _mm256_mul_ps(bc, rhs_1));
                                    sum_12 = _mm256_add_ps(sum_12, _mm256_mul_ps(bc, rhs_2));
    
                                    let bc = _mm256_set1_ps(chunk[(i + 1) * self.cols() + k]);
                                    sum_21 = _mm256_add_ps(sum_21, _mm256_mul_ps(bc, rhs_1));
                                    sum_22 = _mm256_add_ps(sum_22, _mm256_mul_ps(bc, rhs_2));
                                }

                                _mm256_storeu_ps(buffer[idx_11..(idx_11 + 8)].as_mut_ptr(), sum_11);
                                _mm256_storeu_ps(buffer[idx_12..(idx_12 + 8)].as_mut_ptr(), sum_12);
                                _mm256_storeu_ps(buffer[idx_21..(idx_21 + 8)].as_mut_ptr(), sum_21);
                                _mm256_storeu_ps(buffer[idx_22..(idx_22 + 8)].as_mut_ptr(), sum_22);
                            }
                        }
                    }
                }
            }
        });
        Self::new(result, n, p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn default() {
        let mut rng = rand::thread_rng();
        let vals = [0f32, 0.5f32, 1.0f32, 1.5f32];

        const PAR: usize = 8;
        const MATRIX_SIZE_1: usize = 64;
        const MATRIX_SIZE_2: usize = 32;
        const MATRIX_SIZE_3: usize = 128;
        let mut m1 = Matrix::<StateDefault>::zeros(MATRIX_SIZE_1, MATRIX_SIZE_2);
        let mut m2 = Matrix::<StateDefault>::zeros(MATRIX_SIZE_2, MATRIX_SIZE_3);
        for i in 0..m1.rows() {
            for j in 0..m1.cols() {
                m1[(i, j)] = *vals.choose(&mut rng).unwrap();
            }
        }
        for i in 0..m2.rows() {
            for j in 0..m2.cols() {
                m2[(i, j)] = *vals.choose(&mut rng).unwrap();
            }
        }

        let def = m1.mul(&m2).into_inner();

        let m1 = m1.morph::<StateParallel>();
        let m2 = m2.morph::<StateParallel>();
        let par = m1.mul::<PAR>(&m2).into_inner();

        let m1 = m1.morph::<StateParallelBuffer>();
        let m2 = m2.morph::<StateParallelBuffer>();
        let buf = m1.mul::<PAR>(&m2).into_inner();

        let m1 = m1.morph::<StateParallelSimd>();
        let m2 = m2.morph::<StateParallelSimd>();
        let simd = m1.mul::<PAR>(&m2).into_inner();

        assert_eq!(def, par);
        assert_eq!(par, buf);
        assert_eq!(buf, simd);
    }
}
