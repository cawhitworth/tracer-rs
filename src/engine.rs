extern crate image;

use image::Rgb;
use num::{Float, FromPrimitive};
use std::vec;

use crate::vector::Vec4;
use crate::matrix::Mat4;
use crate::object::*;
use crate::light::Light;

pub struct Engine<T: Float> {
    view: Mat4<T>,
    objects: Vec<Box<dyn Intersectable<T>>>,
    lights: Vec<Box<dyn Light<T>>>
}

enum TraceResult<'a, T: Float> {
    Miss,
    Hit(Vec4<T>, &'a Box<dyn Intersectable<T>>),
}

impl<T> Engine<T> 
where T: Float + FromPrimitive + std::fmt::Debug {
    pub fn new(view: Mat4<T>) -> Engine<T> {
        Engine { view, objects: vec![], lights: vec![] }
    }

    pub fn add_object(&mut self, object: Box<dyn Intersectable<T>>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Box<dyn Light<T>>) {
        self.lights.push(light);
    }

    fn trace_ray(&self, origin: &Vec4<T>, direction: &Vec4<T>) -> TraceResult<T> {
        for o in self.objects.iter() {
            let result = o.intersect(origin, direction);
            match result {
                IntersectResult::Intersect(t) => {
                    let v = direction * t;
                    let intersect_point = origin + &v;
                    return TraceResult::Hit(intersect_point, o);
                },
                _ => {}
            }
        }
            
        TraceResult::Miss 
    }

    fn illuminate(&self, point: &Vec4<T>, object: &Box<dyn Intersectable<T>>) -> Rgb<u8> {
        let mut illum: [T; 3] = [T::zero(); 3];
       
        for l in self.lights.iter() {
            let illum_result = l.illuminate(object, point, &Vec4::direction(T::zero(), T::zero(), T::zero() ));
            for i in 0..3 {
                illum[i] = illum[i] + illum_result[i];
            }
        }

        let max_u8 = FromPrimitive::from_u8(0xff).unwrap();
        let illum_scaled = illum.iter()
            .map(|channel| T::min(T::one(), T::max(T::zero(), *channel)))
            .map(|channel| channel * max_u8)
            .map(|channel| channel.to_u8().unwrap())
            .collect::<Vec<_>>();

        Rgb([illum_scaled[0], illum_scaled[1], illum_scaled[2]])
    }

    fn trace_and_illuminate(&self, world_origin: Vec4<T>, target: Vec4<T>) -> Rgb<u8> {
        let world_target = &self.view * &target;
        let world_direction = (&world_target - &world_origin).normalized();
        let hit = self.trace_ray(&world_origin, &world_direction);

        match hit {
            TraceResult::Miss => image::Rgb([0,0,0]),
            TraceResult::Hit(point, object) => self.illuminate(&point, object)
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

        let fx_scale: T;
        let fy_scale: T;
        let fx_origin: T;
        let fy_origin: T;

        if fwidth > fheight {
            fx_scale = two / (fheight - T::one());
            fy_scale = two / (fheight - T::one());
            fx_origin = -(fwidth / fheight);
            fy_origin = T::one();
        } else {
            fx_scale = two / (fwidth - T::one());
            fy_scale = two / (fwidth - T::one());
            fx_origin = -T::one();
            fy_origin = fheight / fwidth;
        }
        
        for y in 0..height {
            for x in 0..width {
                let mut fx = FromPrimitive::from_u32(x).unwrap();
                let mut fy = FromPrimitive::from_u32(y).unwrap();
                fx = fx_origin + fx * fx_scale;
                fy = fy_origin - fy * fy_scale;
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