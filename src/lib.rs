use rand::prelude::*;
#[cfg(target_feature = "sse")]
use std::arch::x86_64::*;

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

    // #[cfg(not(target_feature = "sse3"))]
    // pub fn mul(&self, rhs: &mut Self, result: &mut Self) {
    //     if self.cols != rhs.rows { panic!("could not multiply matrices") }
    //     for j in 0..self.rows {
    //         let start = j * self.cols;
    //         let row = &self.data[start..start + self.cols];
    //         for i in 0..rhs.cols {
    //             let mut col: Vec<f32> = Vec::new();
    //             for j1 in 0..rhs.rows {
    //                 col.push(rhs.data[j1 * rhs.cols + i]);
    //             }
    //             result.data[j * rhs.cols + i] = row.iter().zip(col.iter()).map(|(a,b)| a*b).sum();
    //             col.clear();
    //         }
    //     }
    // }

    #[cfg(target_feature = "sse3")]
    pub fn mul(&self, rhs: &mut Self, result: &mut Self) {
        let mut col = Vec::with_capacity(rhs.rows);

        for i in 0..self.rows {
            let start = i * self.cols;
            let row = &self.data[start..start + self.cols];

            for j in 0..rhs.cols {
                for k in 0..rhs.rows {
                    col.push(rhs.data[k * rhs.cols + j]);
                }

                let total = multiply(&row, &col);
                result.data[i * rhs.cols + j] = total;
                col.clear();
            }
        }
    }

    #[cfg(not(target_feature = "sse3"))]
    pub fn mul(&self, rhs: &mut Self, result: &mut Self) {
        if self.cols != rhs.rows {
            panic!("could not multiply matrices")
        }
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut temp = 0.0;
                for k in 0..rhs.rows {
                    let a = self.data[i * self.cols + k];
                    let b = rhs.data[k * rhs.cols + j];
                    temp = temp + (a * b);
                }
                result.data[i * rhs.cols + j] = temp;
            }
        }
    }
}

// assumes 4x4 matrix
// #[cfg(target_feature = "sse3")]
// fn transpose_16(src: Vec<f32>, dst: &mut Vec<f32>) {
//     let row1 = _mm_load_ps(pad(data[0..4]));
//     let row2 = _mm_load_ps(pad(data[4..8]));
//     let row3 = _mm_load_ps(pad(data[8..12]));
//     let row4 = _mm_load_ps(pad(data[12..16]));
//     _MM_TRANSPOSE4_PS(row1, row2, row3, row4);
//     _mm_store_ps(&mut dst[0..4], row1);
//     _mm_store_ps(&mut dst[4..8], row2);
//     _mm_store_ps(&mut dst[8..12], row3);
//     _mm_store_ps(&mut dst[12..16], row4);
// }

#[cfg(target_feature = "sse3")]
fn multiply(a: &[f32], b: &[f32]) -> f32 {
    unsafe {
        let a_ = arr_pad(a);
        let b_ = arr_pad(b);
        let mut result = 0.0;
        for set_a in a_.chunks(4) {
            let r = _mm_loadu_ps(set_a.as_ptr());
            for set_b in b_.chunks(4) {
                let c = _mm_loadu_ps(set_b.as_ptr());
                let imm = _mm_mul_ps(r, c);
                let res = _mm_hadd_ps(imm, imm);
                let res = _mm_hadd_ps(res, res);
                let xs: [f32; 4] = std::mem::transmute(res);
                // println!("a {:?} b {:?}", set_a, set_b);
                // println!("sets {:?} {:?} {:?} {:?}", r, c, imm, res);
                result = result + xs[0];
            }
        }
        return result;
    }
}

fn arr_pad(xs: &[f32]) -> Vec<f32> {
    let mut v = xs.to_vec();
    while v.len() % 4 != 0 {
        v.push(0.0);
    }
    v
}

// #[cfg(target_feature = "sse")]
// fn pad(xs: &[f32]) -> __m128 {
//     unsafe {
//         match xs.len() {
//             1 => _mm_set_ps(xs[0], 0.0, 0.0, 0.0),
//             2 => _mm_set_ps(xs[0], xs[1], 0.0, 0.0),
//             3 => _mm_set_ps(xs[0], xs[1], xs[2], 0.0),
//             4 => _mm_set_ps(xs[0], xs[3], xs[2], xs[3]),
//             _ => unreachable!(),
//         }
//     }
// }

// #[test]
// #[cfg(target_feature = "sse3")]
// fn transpose_test() {
//     let v_a: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
//     let mut m1 = Matrix::from(v_a, 2, 2);
//     m1.transpose();

//     let expected = vec![1.0, 3.0, 2.0, 4.0];
//     assert_eq!(m1.data, expected);
// }
