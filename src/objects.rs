use std::mem;

use num::{Float, FromPrimitive};

use crate::vector::Vec4;
use crate::matrix::Mat4;

trait Intersectable<T: Float>
{
    fn intersect(&self, origin: &Vec4<T>, direction: &Vec4<T>) -> (bool, T);
}

trait WorldObject<T: Float>
{
    fn object_matrix(&self) -> &Mat4<T>;
}

#[derive(Debug)]
pub struct Sphere<T: Float> {
    object: Mat4<T>
}

impl<T: Float> WorldObject<T> for Sphere<T> {
    fn object_matrix(&self) -> &Mat4<T> {
        &self.object
    }
}


impl<T> Intersectable<T> for Sphere<T> 
where T: Float + FromPrimitive {
    fn intersect(&self, origin: &Vec4<T>, direction: &Vec4<T>) -> (bool, T) {
        let transformed_origin = self.object_matrix() * origin;
        let transformed_direction = self.object_matrix() * direction;

        let two= FromPrimitive::from_f64(2.0).unwrap();
        let four: T = FromPrimitive::from_f64(4.0).unwrap();
        
        let a = transformed_direction.dot_product(&transformed_direction);
        let b  = transformed_direction.dot_product(&transformed_origin) * two;
        let c = transformed_direction.dot_product(&transformed_direction) - T::one();

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
}

impl<T: Float> Sphere<T> {
    pub fn new(origin: Vec4<T>, radius: T) -> Sphere<T> {
        let o: Mat4<T> = Mat4::translation(&origin);
        let scale_vec = Vec4::new(radius, radius, radius);
        let scale = Mat4::scale(&scale_vec);

        Sphere {
            object: &o * &scale
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_sphere() {
        let o: Vec4<f64> = Vec4::new(0.0, 2.0, 0.0);
        let s: Sphere<f64> = Sphere::new(o, 2.0);

        println!("{}", s.object_matrix())
    }
}
