mod mat;
use mat::Mat;

fn main() {
    let vec1 = Mat::<1, 2>::row_mat([1.0, 1.0]);
    let vec2 = Mat::<2, 1>::col_mat([-1.0, -1.0]);
    let dot = &vec1 * &vec2;

    println!("{}", dot)
}
