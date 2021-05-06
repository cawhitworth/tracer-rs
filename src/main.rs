mod vector;
mod matrix;
mod object;
mod engine;

use image::ImageError;
use matrix::Mat4;
use vector::Vec4;
use engine::Engine;
use object::sphere::Sphere;

fn main() -> Result<(), ImageError> {
    let fwd = Vec4::direction(0.0, 0.0, 1.0);
    let right = Vec4::direction(1.0, 0.0, 0.0);
    let up = Vec4::direction(0.0, 1.0, 0.0);
    let pos = Vec4::position(0.0, 0.0, -10.0);

    let camera = Mat4::camera(&fwd, &right, &up, &pos);

    let projection = Mat4::perspective(90.0, 1.0, 100.0);
    let sphere = Sphere::new(Vec4::position(0.0, 4.0, 0.0), 8.0);

    let mut engine = Engine::new(camera, projection);

    engine.add(Box::new(sphere));

    let img = engine.render(640,480);

    img.save("output.png")
}