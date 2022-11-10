use num::Float;

use crate::object::Intersectable;
use crate::vector::Vec4;

use super::Light;

pub struct PointLight<T: Float> {
    position: Vec4<T>,
}

impl<T> Light<T> for PointLight<T>
where
    T: Float,
{
    fn illuminate(
        &self,
        object: &dyn Intersectable<T>,
        hit_point: &Vec4<T>,
        _: &Vec4<T>,
    ) -> [T; 3] {
        let norm = object.normal(hit_point).normalized();

        let light_vec = &self.position - &hit_point;

        let illum = norm.dot_product(&light_vec.normalized());
        if illum < T::zero() {
            return [T::zero(), T::zero(), T::zero()];
        }

        [illum, illum, illum]
    }
}

impl<T> PointLight<T>
where
    T: Float,
{
    pub fn new(position: Vec4<T>) -> PointLight<T> {
        PointLight { position }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let _ = PointLight::new(Vec4::position(0.0, 0.0, 0.0));
    }
}
