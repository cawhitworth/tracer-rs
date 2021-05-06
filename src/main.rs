use matrix::Mat4;
use vector::Vec4;

mod vector;
mod matrix;
mod objects;

fn main() {
    let fwd = Vec4::new(0.0, 0.0, 1.0);
    let right = Vec4::new(1.0, 0.0, 0.0);
    let up = Vec4::new(0.0, 1.0, 0.0);
    let pos = Vec4::new(0.0, 0.0, -10.0);

    let camera: Mat4<f64> = Mat4::camera(&fwd, &right, &up, &pos);

    println!("{}", &camera);

    let sphere = objects::Sphere::new(Vec4::new(0.0, 2.0, 0.0), 2.0);

    println!("{:?}", &sphere);
}