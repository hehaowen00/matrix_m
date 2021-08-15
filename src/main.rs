mod lib;
use lib::Matrix;
use std::time::Instant;

fn main() {
    let a0 = 1000;
    let b0 = 1000;
    let a1 = b0;
    let b1 = a0;

    let m1 = Matrix::random(a0, b0);
    let mut m2 = Matrix::random(a1, b1);
    let mut result = Matrix::from(vec![0.0; a0 * b1], a0, b1);

    // println!("{:?}", m1);
    // println!("{:?}", m2);

    let start = Instant::now();
    m1.mul(&mut m2, &mut result);
    let end = start.elapsed();

    //println!("{:?}", result);
    println!("run time: {:?}s", end.as_millis() as f64 / 1000.0);
}

#[test]
fn test1() {
    let v_a: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
    let v_b: Vec<f32> = vec![2.0, 3.0, 4.0, 5.0];
    let m1 = Matrix::from(v_a, 2, 2);
    let mut m2 = Matrix::from(v_b, 2, 2);
    let mut result = Matrix::from(vec![0.0; 4], 2, 2);
    m1.mul(&mut m2, &mut result);

    let expected = Matrix::from(vec![10.0, 13.0, 22.0, 29.0], 2, 2);
    assert_eq!(result, expected);
}

#[test]
fn test2() {
    let v_a: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let v_b: Vec<f32> = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let m1 = Matrix::from(v_a, 3, 2);
    let mut m2 = Matrix::from(v_b, 2, 3);
    let mut result = Matrix::from(vec![0.0; 4], 2, 2);
    m1.mul(&mut m2, &mut result);

    let expected = Matrix::from(vec![28.0, 34.0, 64.0, 79.0], 2, 2);
    assert_eq!(result, expected);
}

#[test]
fn test3() {
    let v_a: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let v_b: Vec<f32> = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let mut m1 = Matrix::from(v_a, 3, 2);
    let m2 = Matrix::from(v_b, 2, 3);
    let mut result = Matrix::from(vec![0.0; 9], 3, 3);
    m2.mul(&mut m1, &mut result);

    let expected = Matrix::from(
        vec![14.0, 19.0, 24.0, 24.0, 33.0, 42.0, 34.0, 47.0, 60.0],
        3,
        3,
    );
    assert_eq!(result, expected);
}
