use num::Float;

use crate::object::Intersectable;
use crate::vector::Vec4;

pub mod pointlight;
pub mod directionlight;
pub mod ambientlight;

pub trait Light<T: Float> {
    fn illuminate(&self, object: &dyn Intersectable<T>, hit_point: &Vec4<T>, eye_pos: &Vec4<T>) -> [T; 3];
}

impl<T> Light<T> for Box<dyn Light<T>>
where T: Float
{
    fn illuminate(&self, object: &dyn Intersectable<T>, hit_point: &Vec4<T>, eye_pos: &Vec4<T>) -> [T; 3] {
        self.as_ref().illuminate(object, hit_point, eye_pos)
    }
}