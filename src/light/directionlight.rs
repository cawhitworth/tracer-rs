use num::{Float, FromPrimitive};

use crate::vector::Vec4;

use super::Light;

pub struct DirectionLight<T>
where T: Float {
    direction_norm_inv: Vec4<T>
}

impl<T> DirectionLight<T>
where T: Float {
    pub fn new(direction: Vec4<T>) -> DirectionLight<T> {
        DirectionLight {
            direction_norm_inv: direction.normalized().reverse()
        }
    }
}

impl<T> Light<T> for DirectionLight<T>
where T: Float + FromPrimitive {
    fn illuminate(&self, object: &Box<dyn crate::object::Intersectable<T>>, hit_point: &Vec4<T>, eye_pos: &Vec4<T>) -> [T; 3] {
        let norm = object.normal(hit_point).normalized();
        
        let illum = norm.dot_product(&self.direction_norm_inv);
        if illum < T::zero() {
            return [T::zero(), T::zero(), T::zero()]
        }

        [illum, illum, illum]
    }
}