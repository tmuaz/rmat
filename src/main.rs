#![feature(generic_const_exprs)]
mod mat;
mod traits;
use mat::Mat;
use traits::Dot;

fn main() {
    // diagonal line
    let mat = Mat::from_arrays([[1.0, -1.0, -1.0], [-1.0, 1.0, -1.0], [-1.0, -1.0, 1.0]]);
    println!("{}", mat);
    let comp_mat = Mat::from_arrays([[-1.0, -1.0, -1.0], [-1.0, 1.0, -1.0], [-1.0, -1.0, 1.0]]);
    println!("{}", comp_mat);

    println!("{}", mat.normalized_dot(&comp_mat));
}
