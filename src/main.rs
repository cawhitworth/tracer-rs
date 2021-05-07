mod vector;
mod matrix;
mod object;
mod engine;
mod light;

use image::{ImageError, Rgb};
use matrix::Mat4;
use vector::Vec4;
use engine::Engine;
use object::sphere::Sphere;
use light::{ambientlight::AmbientLight, directionlight::DirectionLight};

fn main() -> Result<(), ImageError> {
    let fwd = Vec4::direction(0.0, 0.0, 1.0);
    let right = Vec4::direction(1.0, 0.0, 0.0);
    let up = Vec4::direction(0.0, 1.0, 0.0);
    let pos = Vec4::position(0.0, 0.0, -10.0);

    let camera = Mat4::camera(&fwd, &right, &up, &pos);

    let sphere = Sphere::new(Vec4::position(0.0, 0.0, 0.0), 8.0);

    let mut engine = Engine::new(camera);
    engine.add_object(Box::new(sphere));

    let dlight = DirectionLight::new(Vec4::direction(1.0, -1.0, 0.1).normalized());
    engine.add_light(Box::new(dlight));

    let alight = AmbientLight::new(Rgb([20, 20, 20]));
    engine.add_light(Box::new(alight));

    let img = engine.render(640,480);

    img.save("output.png")
}