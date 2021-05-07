use image::Rgb;
use num::Float;
use num::FromPrimitive;

use crate::object::Intersectable;
use crate::vector::Vec4;

use super::Light;

pub struct AmbientLight<T: Float> {
    colour: [T; 3]
}

impl<T> AmbientLight<T>
where T: Float + FromPrimitive {
    pub fn new(colour: Rgb<u8>) -> AmbientLight<T> {
        
        let max_u8: T = FromPrimitive::from_u8(0xff).unwrap();

        let r: T = FromPrimitive::from_u8(colour[0]).unwrap();
        let g: T = FromPrimitive::from_u8(colour[1]).unwrap();
        let b: T = FromPrimitive::from_u8(colour[2]).unwrap();

        AmbientLight {
            colour: [ r / max_u8, g / max_u8, b / max_u8 ]
        }
    }
}

impl<T> Light<T> for AmbientLight<T>
where T: Float {
    fn illuminate(&self, _: &Box<dyn Intersectable<T>>, _: &Vec4<T>, _: &Vec4<T>) -> [T; 3] {
        self.colour
    }
}