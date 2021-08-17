#![feature(iter_zip)]

use std::arch::x86_64::*;
use std::iter::zip;
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Matrix {
    data: Vec<f32>,
    cols: usize,
    rows: usize,
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        Matrix {
            data: self.data.clone(),
            cols: self.cols,
            rows: self.rows,
        }
    }
}

impl Matrix {
    pub fn from(xs: Vec<f32>, cols: usize, rows: usize) -> Matrix {
        Matrix {
            data: xs,
            cols,
            rows,
        }
    }

    pub fn random(cols: usize, rows: usize) -> Matrix {
        Matrix::from(Matrix::random_vec(cols * rows), cols, rows)
    }

    fn random_vec(num: usize) -> Vec<f32> {
        let mut rng = thread_rng();
        let mut xs = Vec::with_capacity(num);
        for _ in 0..num {
            xs.push(rng.gen_range(-1.0..1.0));
        }
        xs
    }

    #[cfg(target_feature = "sse3")]
    pub fn mul(&self, rhs: &mut Self, result: &mut Self) {
        assert_eq!(self.cols, 100);
        
        let mut r = [0.0; 100];
        let mut col = [0.0; 100];

        for i in 0..self.rows {
            let start = i * self.cols;
            let row = &self.data[start..start + self.cols];

            for j in 0..rhs.cols {
                for k in 0..rhs.rows {
                    col[k] = rhs.data[k * rhs.cols + j];
                }

                r.copy_from_slice(row);
                let total = dot_product(&r, &col);
                result.data[i * rhs.cols + j] = total;
            }
        }
    }

    #[cfg(not(target_feature = "sse3"))]
    pub fn mul(&self, rhs: &mut Self, result: &mut Self) {
        if self.cols != rhs.rows {
            panic!("could not multiply matrices")
        }

        unsafe { matrix_mult(result.data.as_mut_ptr(), self.data.as_ptr(), self.cols, self.rows, rhs.data.as_ptr(), rhs.cols, rhs.rows); }
        // for i in 0..self.rows {
        //     for j in 0..rhs.cols {
        //         let mut temp = 0.0;
        //         for k in 0..rhs.rows {
        //             let a = self.data[i * self.cols + k];
        //             let b = rhs.data[k * rhs.cols + j];
        //             temp = temp + (a * b);
        //         }
        //         result.data[i * rhs.cols + j] = temp;
        //     }
        // }
    }
}

#[cfg(target_feature = "sse3")]
fn dot_product<const N: usize>(a: &[f32; N], b: &[f32; N]) -> f32 {
    unsafe {
        let mut result = 0.0;

        for (a_slice, b_slice) in zip(a.chunks_exact(4), b.chunks_exact(4)) {
            let r = _mm_loadu_ps(a_slice.as_ptr());
            let c = _mm_loadu_ps(b_slice.as_ptr());
            let imm = _mm_mul_ps(r, c);
            let res = _mm_hadd_ps(imm, imm);
            let res = _mm_hadd_ps(res, res);
            let xs: [f32; 4] = std::mem::transmute(res);
            result = result + xs[0];
        }

        return result
    }
}

#[link(name = "matrix", kind = "static")]
extern "C" {
    fn matrix_mult(
        dest: *mut f32,
        a1:  * const f32, a_cols: usize, a_rows: usize,
        b1:  * const f32, b_cols: usize, b_rows: usize,
    );
}
