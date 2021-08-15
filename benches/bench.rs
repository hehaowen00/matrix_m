use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matrix_m::Matrix;

#[cfg(target_feature = "sse3")]
fn benchmark(c: &mut Criterion) {
    println!("SIMD ver");
    let mut result = Matrix::from(vec![0.0; 100 * 100], 100, 100);
    c.bench_function("multiply simd", |b| {
        let m1 = Matrix::random(100, 100);
        let mut m2 = Matrix::random(100, 100);
        b.iter(|| {
            m1.mul(&mut m2, &mut result);
        })
    });
}

#[cfg(not(target_feature = "sse3"))]
fn benchmark(c: &mut Criterion) {
    println!("NOT SIMD ver");
    let mut result = Matrix::from(vec![0.0; 100 * 100], 100, 100);
    c.bench_function("multiply seq", |b| {
        let m1 = Matrix::random(100, 100);
        let mut m2 = Matrix::random(100, 100);
        b.iter(|| {
            m1.mul(&mut m2, &mut result);
        })
    });
}
criterion_group!(benches, benchmark);
criterion_main!(benches);
