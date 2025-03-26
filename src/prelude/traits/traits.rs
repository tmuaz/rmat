pub trait Dot<T> {
    fn dot(&self, rhs: &T) -> f32;
}
