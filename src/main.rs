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
    println!("{}", &mat * &comp_mat);

    let mat_fill = Mat::<10, 10>::fill(1000000000000000.0);
    println!("{}", mat_fill);

    println!("{}", mat.dot(&mat));
}
