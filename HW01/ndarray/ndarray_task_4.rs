use ndarray::{array, concatenate, stack, Array2, Array3, Axis};
use num::Num;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

fn main() -> Result<()> {
    block_matrix(array![[1, 3, 3], [7, 0, 0]]);

    Ok(())
}

fn block_matrix<T>(block: Array2<T>) -> Array2<T>
where T: Num + Clone + std::fmt::Debug
{
    let block = concatenate(Axis(1), &[block.view(), block.view()]).unwrap();
    let block = concatenate(Axis(0), &[block.view(), block.view()]).unwrap();
    block
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test1() {
        let block = array![[1, 3, 3], [7, 0, 0]];
        assert_eq!(block_matrix(block), array![[1, 3, 3, 1, 3, 3],
                                                [7, 0, 0, 7, 0, 0],
                                                [1, 3, 3, 1, 3, 3],
                                                [7, 0, 0, 7, 0, 0]]);
    }
}