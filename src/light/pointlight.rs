use num::Float;

use crate::vector::Vec4;
use crate::object::Intersectable;

use super::Light;

pub struct PointLight<T: Float> {
    position: Vec4<T>
}

impl<T> Light<T> for PointLight<T>
where T: Float {

    fn illuminate(&self, object: &dyn Intersectable<T>, hit_point: &Vec4<T>, eye_pos: &Vec4<T>) -> [T; 3] {
        // Quiet the warnings
        let _ = object;
        let _ = hit_point;
        let _ = eye_pos;
        let _ = self.position;

        [T::zero(), T::zero(), T::zero()]
    }
}

impl<T> PointLight<T>
where T: Float {
    pub fn new(position: Vec4<T>) -> PointLight<T> {
        PointLight {
            position
        }
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