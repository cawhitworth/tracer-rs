use std::mem;

use num::{Float, FromPrimitive};

use crate::vector::Vec4;
use crate::matrix::Mat4;
use super::*;

#[derive(Debug)]
pub struct Sphere<T: Float> {
    object: Mat4<T>,
    object_inverse: Mat4<T>
}

impl<T: Float> WorldObject<T> for Sphere<T> {
    fn object_matrix(&self) -> &Mat4<T> {
        &self.object
    }

    fn object_matrix_inv(&self) -> &Mat4<T> {
        &self.object_inverse
    }
}

impl<T> Intersectable<T> for Sphere<T> 
where T: Float + FromPrimitive {
    fn intersect(&self, origin: &Vec4<T>, direction: &Vec4<T>) -> (bool, T) {
        let transformed_origin = self.object_matrix_inv() * origin;
        let transformed_direction = self.object_matrix_inv() * direction;

        let two= FromPrimitive::from_f64(2.0).unwrap();
        let four: T = FromPrimitive::from_f64(4.0).unwrap();
        
        let a = transformed_direction.dot_product(&transformed_direction);
        let b = transformed_direction.dot_product(&transformed_origin) * two;
        let c = transformed_origin.dot_product(&transformed_origin) - T::one();

        let discriminant = (b*b) - (four*a*c);
        if discriminant < T::zero() {
            return (false, T::zero())
        }

        let sqrt_discriminant = T::sqrt(discriminant);

        let mut t0 = (-b - sqrt_discriminant) / (two * a);
        let mut t1 = (-b + sqrt_discriminant) / (two * a);

        if t1 < t0 {
            mem::swap(&mut t0, &mut t1);
        }

        // if t1 is < 0, sphere is in the ray's negative dirction
        if t1 < T::zero() {
            return (false, T::zero())
        }

        // if t0 < 0, intersection is at t1 (and we are inside the sphere)
        if t0 < T::zero() {
            return (true, t1)
        }

        (true, t0)
    }

    fn normal(&self, intersect_point: &Vec4<T>) -> Vec4<T> {
        let v = self.object_matrix_inv() * intersect_point;
        v.normalized() 
    }
}

impl<T> Sphere<T>
where T: Float + FromPrimitive {
    pub fn new(origin: Vec4<T>, radius: T) -> Sphere<T> {
        let o: Mat4<T> = Mat4::translation(&origin);
        let scale_vec = Vec4::direction(radius, radius, radius);
        let scale = Mat4::scale(&scale_vec);

        let object_matrix = &scale * &o;
        let object_matrix_inverse = object_matrix.inverse();

        Sphere {
            object: object_matrix,
            object_inverse: object_matrix_inverse
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_sphere() {
        let o: Vec4<f64> = Vec4::position(0.0, 2.0, 0.0);
        let s: Sphere<f64> = Sphere::new(o, 2.0);

        println!("{}", s.object_matrix())
    }

    #[test]
    fn sphere_intersect() {
        let o = Vec4::position(0.0, 0.0, 0.0);
        let s = Sphere::new(o, 1.0);
        let ray_origin = Vec4::position(0.0, 0.0, -10.0);
        let ray_direction = Vec4::direction(0.0, 0.0, 1.0);

        let (intersected, t) = s.intersect(&ray_origin, &ray_direction);

        assert_eq!(intersected, true);
        assert_eq!(9.0, t);
    }

    #[test]
    fn sphere_intersect_translated() {
        let o = Vec4::position(0.0, 2.0, 0.0);
        let s = Sphere::new(o, 1.0);
        let ray_origin = Vec4::position(0.0, 0.0, -10.0);
        let ray_direction = Vec4::direction(0.0, 0.0, 1.0);

        let (intersected, _) = s.intersect(&ray_origin, &ray_direction);

        assert_eq!(intersected, false);
    }

    #[test]
    fn sphere_norm() {
        let o = Vec4::position(0.0, 0.0, 0.0);
        let s = Sphere::new(o, 1.0);
        // Sphere at origin, normal at any point should be the same as point vector

        let p = Vec4::position(0.0, 1.0, 0.0);
        let n = s.normal(&p);

        assert_eq!(p, n);
    }
}