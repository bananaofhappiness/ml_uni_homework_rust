use ndarray::{array, Array1, Axis};
use num::Num;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

/*
Напишите функцию, вычисляющую какую-нибудь первообразную данного полинома
*/

fn main() {}

fn antiderivative(coefs: &Array1<f64>) -> Array1<f64> {
    let indexes = Array1::from_iter((1..=coefs.len()).rev().map(|x| x as f64));
    let mut result = coefs / indexes;
    result.append(Axis(0), array![0.].view()).unwrap();
    result
}

#[cfg(test)]
mod task_7 {
    use super::*;

    #[test]
    fn test_1() {
        let coefs = array![4., 6., 0., 1.];
        let res = antiderivative(&coefs);
        assert_eq!(res, array![1., 2., 0., 1., 0.]);
    }

    #[test]
    fn test_2() {
        let coefs = array![1., 7., -12., 21., -6.,];
        let res = antiderivative(&coefs);
        assert_eq!(res, array![0.2, 1.75, -4., 10.5, -6., 0.]);
    }
}