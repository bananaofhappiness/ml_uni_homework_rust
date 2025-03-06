use ndarray::Array1;
use num::{integer::Roots, Float, Num};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

fn main() -> Result<()> {
    

    Ok(())
}

fn cosine_similarity(vec1: Array1<f64>, vec2: Array1<f64>) -> f64 
{
    (&vec1 * &vec2).sum() / ((vec1.pow2().sum().sqrt()) * (vec2.pow2().sum().sqrt()))
}

#[cfg(test)]
mod task_10{
    use ndarray::array;

    use super::*;

    #[test]
    fn test_1() {
        let vec1 = array![-2., 1.,  0., -5., 4., 3., -3.];
        let vec2 = array![ 0., 2., -2., 10., 6., 0.,  0.];

        assert_eq!(cosine_similarity(vec1, vec2), -0.25)
    }

    #[test]
    fn test_2() {
        let vec1 = array![-4., 2.,  9., -8., 9., 0., -2.];
        let vec2 = array![ 3., 2., -4., -1., 3., 2.,  2.];

        assert_eq!(cosine_similarity(vec1, vec2), -0.119929)
    }
}