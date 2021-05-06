mod vector;
mod matrix;
mod object;
mod engine;

use matrix::Mat4;
use vector::Vec4;
use engine::Engine;
use object::sphere::Sphere;

fn main() {
    let fwd = Vec4::direction(0.0, 0.0, 1.0);
    let right = Vec4::direction(1.0, 0.0, 0.0);
    let up = Vec4::direction(0.0, 1.0, 0.0);
    let pos = Vec4::position(0.0, 0.0, -10.0);

    let camera = Mat4::camera(&fwd, &right, &up, &pos);

    println!("{}", &camera);

    let sphere = Sphere::new(Vec4::position(0.0, 4.0, 0.0), 8.0);

    println!("{:?}", &sphere);

    let mut engine = Engine::new();
    engine.add(Box::new(sphere));

    let o = Vec4::position(0.0, 0.0, -10.0);
    let d = Vec4::direction(0.0, 0.0, 1.0);

    let hit = engine.trace_ray(&o, &d);

    match hit {
        engine::Hit::Miss => println!("Miss!"),
        engine::Hit::Hit((p, _)) =>
        {
            println!("Hit object at {:?}", p);
        }

    }
}