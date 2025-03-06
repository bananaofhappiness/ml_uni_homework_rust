use ndarray::{stack, Array1, Array2, Axis};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

fn main() {}

fn construct_matrix(m: i32, a: i32, b: i32) -> Array2<i32> {
    let array = 
        Array2::from_shape_fn((m as usize, (b - a + 1) as usize), 
        |(_, j)| j as i32 + a);
    array
}

#[cfg(test)]
mod task_9{
    use ndarray::array;

    use super::*;

    #[test]
    fn test_1() {
        let a = construct_matrix(5, 3, 10);
        assert_eq!(a, array![[ 3,  4,  5,  6,  7,  8,  9, 10],
                            [ 3,  4,  5,  6,  7,  8,  9, 10],
                            [ 3,  4,  5,  6,  7,  8,  9, 10],
                            [ 3,  4,  5,  6,  7,  8,  9, 10],
                            [ 3,  4,  5,  6,  7,  8,  9, 10]])
    }

    #[test]
    fn test_2() {
        let a = construct_matrix(3, 2, 6);
        assert_eq!(a, array![[2, 3, 4, 5, 6],
                            [2, 3, 4, 5, 6],
                            [2, 3, 4, 5, 6]])
    }
}