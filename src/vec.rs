use num::Float;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, PartialEq)]
pub struct Vec3<T> 
where T: Float {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where T: Float{
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn dot_product(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z 
        }
    }

    pub fn cross_product(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x)
        }
    }

    pub fn mag(&self) -> T {
        Float::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalized(&self) -> Vec3<T> {
        let mag = self.mag();
        Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag
        }
    }
}

// To consider - use the impl_ops crate

impl<T> Add for &Vec3<T> 
where T: Float {
    type Output = Vec3<T>;

    fn add(self, other: &Vec3<T>) -> Vec3<T> {
        Vec3{ x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<T> Sub for &Vec3<T>
where T: Float {
    type Output = Vec3<T>;

    fn sub(self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl<T> Mul<T> for &Vec3<T>
where T: Float {
    type Output = Vec3<T>;

    fn mul(self, other: T) -> Vec3<T> {
        Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        let u = &v;
        println!("{:?} {:?}", &u, &v);
    }

    #[test]
    fn add() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), &u + &u);
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), &u + &v);
    }

    #[test]
    fn sub() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), &u - &v);
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), &u - &u);
    }

    #[test]
    fn mul() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), &u * 2.0);
        assert_eq!(Vec3::new(-3.0, -6.0, -9.0), &v * 3.0);
    }

    #[test]
    fn dot_product() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(-1.0, -2.0, -3.0);

        assert_eq!(Vec3::new(-1.0, -4.0, -9.0), u.dot_product(&v));
        assert_eq!(Vec3::new(-1.0, -4.0, -9.0), v.dot_product(&u));
        assert_eq!(u.dot_product(&v), v.dot_product(&u));
    }

    #[test]
    fn cross_product_unitvecs() {
        let u = Vec3::new(1.0, 0.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let w = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(w, u.cross_product(&v));
        assert_eq!(u, v.cross_product(&w));
        assert_eq!(v, w.cross_product(&u));
    }

    #[test]
    fn mag() {
        // Probably shouldn't use direct equality for floats here

        assert_eq!(1.0, Vec3::new(1.0, 0.0, 0.0).mag());
        assert_eq!(Float::sqrt(2.0), Vec3::new(1.0, 1.0, 0.0).mag());
        assert_eq!(Float::sqrt(12.0), Vec3::new(2.0, 2.0, 2.0).mag());
    }

    #[test]
    fn norm() {
        let u = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(u, u.normalized());
        assert_eq!(u, (&u * 2.0).normalized());
    }

    #[test]
    fn combinatoric() {
        let u = Vec3::new(2.0, 3.0, 0.0);
        let v = Vec3::new(-3.0, 2.0, 0.0);
        let w = u.cross_product(&v).normalized();
        assert_eq!(Vec3::new(0.0, 0.0, 1.0), w);
    }
}