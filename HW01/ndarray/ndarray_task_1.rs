use ndarray::{array, Array, Array1};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

/*
Напишите функцию, возвращающую округленную взвешенную сумму оценок по данным оценкам и весам. 
Предположим, вес экзамена равен 0.3, вес домашек - 0.4, вес контрольной - 0.2, вес самостоятельных - 0.1. 
Верхняя оценка - 10. Например, если за экзамен у вас 7, за домашки 10, за контрольную 8, а за самостоятельные 6, 
то вы получите отличную оценку 8!
*/

fn main() {}

fn result_mark(weights: Array1<f32>, marks: Array1<i32>) -> i32 {
    let marks = marks.mapv(|x| x as f32);
    let res = weights * marks;

    res.sum().round() as i32
}



#[cfg(test)]
mod task_1{
    use super::*;
    
    #[test]
    fn test1() -> Result<()> {
        let weights = array![0.3, 0.4, 0.2, 0.1];
        let marks = array![7, 10, 8, 6];

        assert_eq!(result_mark(weights, marks), 8);
        Ok(())
    }

    #[test]
    fn test2() -> Result<()> {
        let weights = array![0.3, 0.4, 0.2, 0.1];
        let marks = array![7, 0, 8, 6];

        assert_eq!(result_mark(weights, marks), 4);
        Ok(())
    }
}