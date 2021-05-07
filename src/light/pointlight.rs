use image::Rgb;
use num::Float;

use crate::vector::Vec4;
use crate::object::Intersectable;

use super::Light;

pub struct PointLight<T: Float> {
    position: Vec4<T>
}

impl<T> Light<T> for PointLight<T>
where T: Float {

    fn illuminate(&self, object: &Box<dyn Intersectable<T>>, hit_point: &Vec4<T>, eye_pos: &Vec4<T>) -> [T; 3] {
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
        let l = PointLight::new(Vec4::position(0.0, 0.0, 0.0));
    }
}