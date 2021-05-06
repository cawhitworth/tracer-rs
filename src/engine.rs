extern crate image;

use image::Rgb;
use num::{Float, FromPrimitive};

use crate::matrix::Mat4;
use crate::object::Intersectable;
use std::vec;
use crate::vector::Vec4;

pub struct Engine<T: Float> {
    view: Mat4<T>,
    objects: Vec<Box<dyn Intersectable<T>>>
}

pub enum Hit<'a, T: Float> {
    Miss,
    Hit(
        Vec4<T>,
         &'a Box<dyn Intersectable<T>>
    ),
}

impl<T: Float + FromPrimitive + std::fmt::Debug> Engine<T> {
    pub fn new(view: Mat4<T>) -> Engine<T> {
        Engine {
            view: view,
            objects: vec![]
        }
    }

    pub fn add(&mut self, object: Box<dyn Intersectable<T>>) {
        self.objects.push(object);
    }

    pub fn trace_ray(&self, origin: &Vec4<T>, direction: &Vec4<T>) -> Hit<T> {
        for o in self.objects.iter() {
            let (hit, t) = o.intersect(origin, direction);
            if hit {
                let v = direction * t;
                let intersect_point = origin + &v;
                return Hit::Hit( intersect_point, o)
            }
        }
        Hit::Miss 
    }

    fn trace_and_illuminate(&self, world_origin: Vec4<T>, target: Vec4<T>) -> Rgb<u8> {
        let world_target = &self.view * &target;
        let world_direction = (&world_target - &world_origin).normalized();
        let hit = self.trace_ray(&world_origin, &world_direction);

        match hit {
            Hit::Miss => image::Rgb([0,0,0]),
            Hit::Hit(_, _) => image::Rgb([255, 255, 255])
        }

    }

    pub fn render(&self, width: u32, height: u32) -> image::RgbImage {
        let mut image = image::RgbImage::new(width, height);
        image.put_pixel(0, 0, image::Rgb([255, 255, 255]));

        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        let hfov: T = FromPrimitive::from_f64(90.0).unwrap();

        let distance = T::one() / (T::tan(hfov / two));
        let origin = Vec4::position(T::zero(), T::zero(), -distance);
        let world_origin = &self.view * &origin;

        let fwidth: T = FromPrimitive::from_u32(width).unwrap();
        let fheight: T = FromPrimitive::from_u32(height).unwrap();

        let fx_scale = two / (fwidth - T::one());
        let fy_scale = two / (fheight - T::one());

        for y in 0..height {
            for x in 0..width {
                let mut fx = FromPrimitive::from_u32(x).unwrap();
                let mut fy = FromPrimitive::from_u32(y).unwrap();
                fx = -T::one() + fx * fx_scale;
                fy = -T::one() + fy * fy_scale;
                let target = Vec4::position(fx, fy, T::zero());
                image.put_pixel(x, y, self.trace_and_illuminate(world_origin, target));

            }
        }
        image
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn construct() {
        let view = Mat4::i();
        let _: Engine<f64> = Engine::new(view);
    }
}