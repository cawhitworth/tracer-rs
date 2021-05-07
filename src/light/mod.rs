use num::Float;

use crate::object::Intersectable;
use crate::vector::Vec4;

pub mod pointlight;
pub mod directionlight;
pub mod ambientlight;

pub trait Light<T: Float> {
    fn illuminate(&self, object: &Box<dyn Intersectable<T>>, hit_point: &Vec4<T>, eye_pos: &Vec4<T>) -> [T; 3];
}
