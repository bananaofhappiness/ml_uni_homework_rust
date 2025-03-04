use ndarray::{array, Array, Array1, ArrayBase, Dim, OwnedRepr};
use num::Num;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

/*
Напишите функцию, выдающую индексы «близких» элементов заданных массивов, а именно тех пар элементов, 
чей модуль разницы не превосходит заданного значения. 
Например, если на вход поступают массив `array([1.5, 0.5, 2, -4.1, -3, 6, -1])`, 
массив `array([1.2, 0.5, 1, -4, 3, 0, -1.2])` и число `0.5`, 
то на выходе должен получиться массив `array([0, 1, 3, 6])` 
(**важно: не `tuple`, а одномерный массив типа `numpy.ndarray` 
(то есть `.ndim` от него равно 1)!**).
*/

fn main(){
    find_close(array![1.1,2.2,3.3], array![1.12,5.4,4.3], 1.1);
}

fn find_close(arr1: Array1<f64>, arr2: Array1<f64>, precision: f64) -> Array1<u32> {
    let res = arr1 - arr2;
    let res = Array::from_iter(
        (0..res.len() as u32)
                .filter(|&x| f64::abs(f64::from(res[x as usize]))  < precision));
    res
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test1() {
        let arr1 = array![1.5, 0.5, 2., -4.1, -3., 6., -1.];
        let arr2 = array![1.2, 0.5, 1., -4.0,  3., 0., -1.2];
        let precision = 0.5;

        let res = find_close(arr1, arr2, precision);
        assert_eq!(res, array![0, 1, 3, 6]);
    }
    
    #[test]
    fn test2() {
        let arr1 = array![3.1415, 2.7182, 1.6180, 6.6261];
        let arr2 = array![6.6730, 1.3807, -1.,    6.0222];
        let precision = 1.7;

        let res = find_close(arr1, arr2, precision);
        assert_eq!(res, array![1, 3]);
    }
}