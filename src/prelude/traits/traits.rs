pub trait Dot<T = Self> {
    fn dot(&self, rhs: &T) -> f32;
}
