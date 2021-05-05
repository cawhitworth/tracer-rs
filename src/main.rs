use matrix::Mat4;

mod vector;
mod matrix;

fn main() {
    let m: Mat4<f64> = Mat4::i();
    println!("{}", &m * &m);

}
