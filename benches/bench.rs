use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matrix_m::Matrix;

#[cfg(target_feature = "sse4.2")]
fn benchmark(c: &mut Criterion) {
    println!("SIMD ver");
    let mut result = Matrix::from(vec![0.0; 1000 * 1000], 1000, 1000);
    c.bench_function("multiply 1000*1000", |b| {
        let m1 = black_box(Matrix::random(1000, 1000));
        let mut m2 = black_box(Matrix::random(1000, 1000));
        b.iter(|| {
            m1.mul(&mut m2, &mut result);
        })
    });
}

#[cfg(not(target_feature = "sse4.2"))]
fn benchmark(c: &mut Criterion) {
    let mut result = Matrix::from(vec![0.0; 1000 * 1000], 1000, 1000);
    c.bench_function("multiply 1000*1000", |b| {
        let m1 = black_box(Matrix::random(1000, 1000));
        let mut m2 = black_box(Matrix::random(1000, 1000));
        b.iter(|| {
            m1.mul(&mut m2, &mut result);
        })
    });
}
criterion_group!(benches, benchmark);
criterion_main!(benches);
