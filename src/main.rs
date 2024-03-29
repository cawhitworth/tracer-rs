mod engine;
mod light;
mod matrix;
mod object;
mod vector;

use engine::Engine;
use image::{ImageError, Rgb};
use light::{ambientlight::AmbientLight, directionlight::DirectionLight, pointlight::PointLight};
use matrix::Mat4;
use object::sphere::Sphere;
use vector::Vec4;

fn main() -> Result<(), ImageError> {
    let pos = Vec4::position(0.0, 0.0, -10.0);
    let origin = Vec4::position(0.0, 0.0, 0.0);

    let camera = Mat4::look(&pos, &origin);

    let sphere = Sphere::new(Vec4::position(0.0, 0.0, 0.0), 8.0);

    let mut engine = Engine::new(camera);
    engine.add_object(Box::new(sphere));

    let _dlight = DirectionLight::new(Vec4::direction(1.0, -1.0, 0.1).normalized());
//    engine.add_light(Box::new(dlight));

    let alight = AmbientLight::new(Rgb([20, 20, 20]));
    engine.add_light(Box::new(alight));

    let plight = PointLight::new(Vec4::position(-25.0, 25.0, -25.0));
    engine.add_light(Box::new(plight));

    let img = engine.render(640, 480);

    img.save("output.png")
}
