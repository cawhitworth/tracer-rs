extern crate image;

use num::Float;

use crate::object::Intersectable;
use std::vec;
use crate::vector::Vec4;

pub struct Engine<T> {

    objects: Vec<Box<dyn Intersectable<T>>>
}

pub enum Hit<'a, T: Float> {
    Miss,
    Hit(
        (Vec4<T>,
         &'a Box<dyn Intersectable<T>>)
    ),
}

impl<T: Float> Engine<T> {
    pub fn new() -> Engine<T> {
        Engine {
            objects: vec![]
        }
    }

    pub fn add(&mut self, object: Box<dyn Intersectable<T>>) {
        self.objects.push(object);
    }

    pub fn trace_ray(&self, origin: &Vec4<T>, direction: &Vec4<T>)
        -> Hit<T> {
        for o in self.objects.iter() {
            let (hit, t) = o.intersect(origin, direction);
            if hit {
                let v = direction * t;
                let intersect_point = origin + &v;
                return Hit::Hit( (intersect_point, o))
            }
        }

       Hit::Miss 
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn construct() {
        let _engine: Engine<f64> = Engine::new();
    }
}