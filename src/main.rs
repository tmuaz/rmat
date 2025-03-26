mod prelude;
use prelude::*;

fn main() {
    // diagonal line
    let mat = Mat::from_arrays([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
    mat.dot(&mat);

    // println!("{:?}",)
}
