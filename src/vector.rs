use num::Float;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq)]
pub struct Vec4<T> 
where T: Float {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T>
where T: Float{
    pub fn new(x: T, y: T, z: T) -> Vec4<T> {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: T::one()
        }
    }

    pub fn dot_product(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: T::one()
        }
    }

    pub fn cross_product(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
            w: T::one()
        }
    }

    pub fn mag(&self) -> T {
        Float::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalized(&self) -> Vec4<T> {
        let mag = self.mag();
        Vec4 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w
        }
    }
}

/*
To consider - use the impl_ops crate
*/

impl<T> Add for &Vec4<T> 
where T: Float {
    type Output = Vec4<T>;

    fn add(self, other: &Vec4<T>) -> Vec4<T> {
        Vec4{ x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: self.w }
    }
}

impl<T> Sub for &Vec4<T>
where T: Float {
    type Output = Vec4<T>;

    fn sub(self, other: &Vec4<T>) -> Vec4<T> {
        Vec4 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: self.w }
    }
}

impl<T> Mul<T> for &Vec4<T>
where T: Float {
    type Output = Vec4<T>;

    fn mul(self, other: T) -> Vec4<T> {
        Vec4 { x: self.x * other, y: self.y * other, z: self.z * other, w: self.w }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let _v = Vec4::new(1.0, 1.0, 1.0);
    }

    #[test]
    fn add() {
        let u = Vec4::new(1.0, 2.0, 3.0);
        let v = Vec4::new(-1.0, -2.0, -3.0);
        assert_eq!(Vec4::new(2.0, 4.0, 6.0), &u + &u);
        assert_eq!(Vec4::new(0.0, 0.0, 0.0), &u + &v);
    }

    #[test]
    fn sub() {
        let u = Vec4::new(1.0, 2.0, 3.0);
        let v = Vec4::new(-1.0, -2.0, -3.0);
        assert_eq!(Vec4::new(2.0, 4.0, 6.0), &u - &v);
        assert_eq!(Vec4::new(0.0, 0.0, 0.0), &u - &u);
    }

    #[test]
    fn mul() {
        let u = Vec4::new(1.0, 2.0, 3.0);
        let v = Vec4::new(-1.0, -2.0, -3.0);
        assert_eq!(Vec4::new(2.0, 4.0, 6.0), &u * 2.0);
        assert_eq!(Vec4::new(-3.0, -6.0, -9.0), &v * 3.0);
    }

    #[test]
    fn dot_product() {
        let u = Vec4::new(1.0, 2.0, 3.0);
        let v = Vec4::new(-1.0, -2.0, -3.0);

        assert_eq!(Vec4::new(-1.0, -4.0, -9.0), u.dot_product(&v));
        assert_eq!(Vec4::new(-1.0, -4.0, -9.0), v.dot_product(&u));
        assert_eq!(u.dot_product(&v), v.dot_product(&u));
    }

    #[test]
    fn cross_product_unitvecs() {
        let u = Vec4::new(1.0, 0.0, 0.0);
        let v = Vec4::new(0.0, 1.0, 0.0);
        let w = Vec4::new(0.0, 0.0, 1.0);

        assert_eq!(w, u.cross_product(&v));
        assert_eq!(u, v.cross_product(&w));
        assert_eq!(v, w.cross_product(&u));
    }

    #[test]
    fn mag() {
        // Probably shouldn't use direct equality for floats here

        assert_eq!(1.0, Vec4::new(1.0, 0.0, 0.0).mag());
        assert_eq!(Float::sqrt(2.0), Vec4::new(1.0, 1.0, 0.0).mag());
        assert_eq!(Float::sqrt(12.0), Vec4::new(2.0, 2.0, 2.0).mag());
    }

    #[test]
    fn norm() {
        let u = Vec4::new(1.0, 0.0, 0.0);
        assert_eq!(u, u.normalized());
        assert_eq!(u, (&u * 2.0).normalized());
    }

    #[test]
    fn combinatoric() {
        let u = Vec4::new(2.0, 3.0, 0.0);
        let v = Vec4::new(-3.0, 2.0, 0.0);
        let w = u.cross_product(&v).normalized();
        assert_eq!(Vec4::new(0.0, 0.0, 1.0), w);
    }
}