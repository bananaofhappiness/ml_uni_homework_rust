use ndarray::{array, Array, Array2};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

fn main() {}

fn diag_prod(matrix: Array2<i32>) -> i32 {
    let diag = matrix.diag();
    diag.iter().filter(|x| **x != 0).product()
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test1() {
        let arr = array![[0, 1, 2, 3],
                        [4, 5, 6, 7],
                        [8, 9, 10, 11],
                        [12, 13, 14, 15]];
        assert_eq!(diag_prod(arr),750);
    }
}


