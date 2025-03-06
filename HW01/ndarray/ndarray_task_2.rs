use ndarray::{array, s, Array1};
use num::Num;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

/*
Напишите функцию, меняющую каждое третье (начиная с 0) значение массива целых чисел на заданное число.
Например, если на вход поступает массив `array([3, 5, 1, 0, -3, 22, 213436])` и число `-111`, 
то на выходе должен получиться массив `array([-111, 5, 1, -111, -3, 22, -111])`.
*/

fn main() {
    // let arr = array![1, 2, 3, 4, 5];
    // solution(arr, 3);
}

fn change_array<T>(mut arr: Array1<T>, num: T) -> Array1<T> 
where T: Num + Clone
{
    let mut a = arr.slice_mut(s![..;3]);
    a.fill(num);
    arr
}

#[cfg(test)]
mod task_2{
    use super::*;

    #[test]
    fn test1() {
        let arr = array![3, 5, 1, 0, -3, 22, 213436];
        let num = -111;

        assert_eq!(change_array(arr, num), array![-111, 5, 1, -111, -3, 22, -111]);
    }

    #[test]
    fn test2() {
        let arr = array![3, 14, 15, 92, 6];
        let num = 8;

        assert_eq!(change_array(arr, num), array![8, 14, 15, 8, 6]);
    }
}
