use ndarray::{array, Array2};
use num::Num;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

fn main() {
    let mut array = array![[1, 2, 3, 4], [0, 5, 6, 7], [0, 0, 8, 9], [0, 0, 0, 10]];

    make_symmetric(&mut array);
}


fn make_symmetric(matrix: &mut Array2<i32>) 
{
    *matrix = matrix.t().to_owned() + matrix.clone() - Array2::from_diag(&matrix.clone().into_diag());
}

#[cfg(test)]
mod task_8 {
    use super::*;

    #[test]
    fn test_1() {
        let mut array = array![[1, 2, 3, 4], [0, 5, 6, 7], [0, 0, 8, 9], [0, 0, 0, 10]];

        make_symmetric(&mut array);
        
        assert_eq!(array, array![[ 1,  2,  3,  4],
                                [ 2,  5,  6,  7],
                                [ 3,  6,  8,  9],
                                [ 4,  7,  9, 10]]);
    }

    #[test]
    fn test_2() {
        let mut array = array![[10, 21, 32, 49], [0, 53, 62, 78], [0, 0, 82, 92], [0, 0, 0, 10]];

        make_symmetric(&mut array);
        
        assert_eq!(array, array![[10, 21, 32, 49],
                                [21, 53, 62, 78],
                                [32, 62, 82, 92],
                                [49, 78, 92, 10]]);
    }
}