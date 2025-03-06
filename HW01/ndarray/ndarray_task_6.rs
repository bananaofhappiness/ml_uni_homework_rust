use std::{fmt::Debug};

use ndarray::{array, Array2, Axis};
use num::{signum, Float, FromPrimitive, Num};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

// Нормализовать матрицу

fn main() -> Result<()> {
    // let mut a = array![[1., 4., 4200.], [0., 10., 5000.], [1., 2., 1000.]];

    // normalize(&mut a);
    // println!("->> {a:?}");
    Ok(())
}

fn normalize(array: &mut Array2<f64>) {
    array.axis_iter_mut(Axis(1)).for_each(move |mut x| {
        // get standard deviation and mean. if std == 0, 
        // then make it 1 so we don't divide by 0,
        let std = Some(x.std(0.))
                        .filter(|a| *a != 0. as f64)
                        .unwrap_or_else(|| (x.len() as f64).sqrt() * signum(x[0]));
        let mean = x.mean().unwrap();

        let mut arr = &x - mean;

        arr = arr / std;
        let mut arr = arr.iter();
        x.iter_mut().for_each(|a| {
            *a = *arr.next().unwrap();
        });
    });
}

#[cfg(test)]
mod task_6{
    use super::*;
    
    #[test]
    fn test_1() {
        let mut a = array![[1., 4., 4200.],
                                                        [0., 10., 5000.],
                                                        [1., 2., 1000.]];
        let b = array![[0.7071067811865476, -0.392232270276368, 0.4629100498862757],
                                                    [-1.414213562373095, 1.3728129459672882, 0.9258200997725514],
                                                    [0.7071067811865476, -0.98058067569092, -1.3887301496588271]];
        normalize(&mut a);
        assert_eq!(a, b);
    }

    #[test]
    fn test_2() {
        let mut a = array![[-7., 2., 42.],
                                                        [2., 10., 50.],
                                                        [5., 4., 10.]];
        let b = array![[-1.3728129459672884, -0.98058067569092, 0.4629100498862757],
                                                    [0.3922322702763681, 1.3728129459672882, 0.9258200997725514],
                                                    [0.9805806756909202, -0.392232270276368, -1.3887301496588271]];
        normalize(&mut a);
        assert_eq!(a, b);
    }

    #[test]
    fn test_3() {
        let mut a = array![[-7., 2., 42.],
                                                        [-7., 10., 540.],
                                                        [-7., 2., 10.],
                                                        [-7., -5., 342.]];
        let b = array![[0., -0.04708816093480111, -0.8733032916310017],
                                                    [0., 1.4597329889788344, 1.3977412996600629],
                                                    [0., -0.04708816093480111, -1.0192338677782187],
                                                    [0., -1.3655566671092323, 0.49479585974915763]];
        normalize(&mut a);
        assert_eq!(a, b);
    }
}