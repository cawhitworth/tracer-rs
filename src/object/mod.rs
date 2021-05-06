pub mod sphere;

use num::Float;

use crate::vector::Vec4;
use crate::matrix::Mat4;

trait Intersectable<T: Float>
{
    fn intersect(&self, origin: &Vec4<T>, direction: &Vec4<T>) -> (bool, T);
    fn normal(&self, intersect_point: &Vec4<T>) -> Vec4<T>;
}

trait WorldObject<T: Float>
{
    fn object_matrix(&self) -> &Mat4<T>;
    fn object_matrix_inv(&self) -> &Mat4<T>;
}
