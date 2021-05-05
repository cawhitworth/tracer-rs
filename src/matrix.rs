use num::Float;
use std::fmt;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, PartialEq)]
pub struct Mat4<T>
where T: Float {
    d: [T; 16]
}

impl<T> Mat4<T>
where T: Float {
    pub fn new() -> Mat4<T> {
        Mat4 {
            d: [ T::zero(); 16 ]
        }
    }

    pub fn i() -> Mat4<T> {
        let one = T::one();
        let zero = T::zero();

        Mat4 {
            d:[ 
                one, zero, zero, zero,
                zero, one, zero, zero,
                zero, zero, one, zero,
                zero, zero, zero, one
            ]
        }
    }
}

impl<T> fmt::Display for Mat4<T>
where T: Float + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..4 {
            f.write_str("[ ")?;

            for col in 0..4 {
                f.write_fmt(format_args!("{:.3} ", self[(col, row)]))?;
            }

            f.write_str("]")?;
        }
        Ok(())
    }
}

impl<T> Index<usize> for Mat4<T>
where T: Float {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.d[index]
    }
}

impl<T> IndexMut<usize> for Mat4<T>
where T: Float {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.d[index]
    }
}

impl<T> Index<(usize, usize)> for Mat4<T>
where T: Float {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        let (x,y) = index;
        if x > 3 || y > 3 {
            core::panic!("Matrix index out of bounds");
        }
        &self.d[x + y*4]
    }
}
    
impl<T> IndexMut<(usize, usize)> for Mat4<T>
where T: Float {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        let (x,y) = index;
        if x > 3 || y > 3 {
            core::panic!("Matrix index out of bounds");
        }
        &mut self.d[x + y*4]
    }
}

impl<T> Mul for &Mat4<T>
where T: Float {
    type Output = Mat4<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Mat4::new();
        for row in 0..4 {
            for col in 0..4 {
                result[(row, col)] =
                self[(row, 0)] * rhs[(0, col)] + 
                self[(row, 1)] * rhs[(1, col)] + 
                self[(row, 2)] * rhs[(2, col)] + 
                self[(row, 3)] * rhs[(3, col)];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let _m: Mat4<f64> = Mat4::i();
    }

    #[test]
    fn access_ro()
    {
        let m = Mat4::i();
        assert_eq!(1.0, m[(0,0)]);
        assert_eq!(1.0, m[(1,1)]);
        assert_eq!(1.0, m[(2,2)]);
        assert_eq!(1.0, m[(3,3)]);
    }

    #[test]
    #[should_panic]
    fn access_ro_oob()
    {
        let m: Mat4<f64> = Mat4::i();
        let _f = m[(5,5)];
    }
    
    #[test]
    fn access_rw()
    {
        let mut m = Mat4::i();
        m[(0,0)] = 2.0;
        m[(1,1)] = 2.0;
        m[(2,2)] = 2.0;
        m[(3,3)] = 2.0;
        m[(0,0)] = 2.0;
    }

    #[test]
    #[should_panic]
    fn access_rw_oob()
    {
        let mut m = Mat4::i();
        m[(5,5)] = 2.0;
    }

    #[test]
    fn multiply_identity()
    {
        let i: Mat4<f64> = Mat4::i();

        let result = &i * &i;
        assert_eq!(i, result);
    }
}