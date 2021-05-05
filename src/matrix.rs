use num::Float;
use std::fmt;
use std::ops::{Index, IndexMut, Mul};

use crate::vector::Vec4;

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

    pub fn camera(fwd: &Vec4<T>, right: &Vec4<T>, up: &Vec4<T>, pos: &Vec4<T>) -> Mat4<T> {
        let zero = T::zero();
        let one = T::one();
        Mat4 {
            d: [
                right.x, up.x, fwd.x, pos.x,
                right.y, up.y, fwd.y, pos.y,
                right.z, up.z, fwd.z, pos.z,
                zero,    zero, zero,  one
            ]
        }
    }

    pub fn look(position: &Vec4<T>, look_at: &Vec4<T>) -> Mat4<T> {
        let direction = (look_at - position).normalized();
        let temp_up = Vec4::new(T::zero(), T::one(), T::zero());
        let right = temp_up.cross_product(&direction).normalized();
        let up = direction.cross_product(&right).normalized();

        Mat4::camera(&direction, &right, &up, &position)
    }

    pub fn translation(translation: &Vec4<T>) -> Mat4<T> {
        let mut t: Mat4<T> = Mat4::i();
        t[(3,0)] = translation.x;
        t[(3,1)] = translation.y;
        t[(3,2)] = translation.z;

        t
    }

    pub fn scale(factor: &Vec4<T>) -> Mat4<T> {
        let mut s: Mat4<T> = Mat4::i();
        s[(0,0)] = factor.x;
        s[(1,1)] = factor.y;
        s[(2,2)] = factor.z;

        s
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

impl<T> Mul<&Vec4<T>> for &Mat4<T>
where T: Float {
    type Output = Vec4<T>;

    fn mul(self, rhs: &Vec4<T>) -> Self::Output {
        let mut r = Vec4::new(T::zero(), T::zero(), T::zero());

        for row in 0..4 {
            r[row] = self[(row, 0)] * rhs[0] +
                self[(row, 1)] * rhs[1] +
                self[(row, 2)] * rhs[2] +
                self[(row, 3)] * rhs[3];
        }
        r

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

    #[test]
    fn look_at()
    {
        let pos = Vec4::new(0.0, 0.0, -10.0);
        let origin = Vec4::new(0.0, 0.0, 0.0);
        let camera = Mat4::look(&pos, &origin);

        println!("{}", camera)
    }
}