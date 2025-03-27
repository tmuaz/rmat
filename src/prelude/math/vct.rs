use std::{
    fmt::Display,
    ops::{Add, Div, Index, IndexMut, Mul, MulAssign},
};

use sdl2::rect::Point;

use crate::traits::Dot;

use super::Mat;
#[derive(Debug, Clone)]
pub struct Vct<const L: usize> {
    contents: [f32; L],
}

impl<const L: usize> Vct<L> {
    pub const ZERO: Self = Self { contents: [0.0; L] };

    pub fn generate<F>(generator: F) -> Self
    where
        F: Fn(usize) -> f32,
    {
        let mut output = Self::ZERO;
        for (i, v) in output.contents.iter_mut().enumerate() {
            *v = generator(i);
        }
        output
    }

    #[inline(always)]
    pub fn from_array(array: [f32; L]) -> Self {
        Self { contents: array }
    }
    #[inline(always)]
    pub fn from_array_ref(array: &[f32; L]) -> &Self {
        // this should allow for a perfect cast and also should behave well with the borrow checker
        // The structure of [f32;L] and Vct<L> is the exact same so this is possible
        unsafe { &*((array as *const [f32; L]) as *const Self) }
    }
}

impl<const L: usize> Dot for Vct<L> {
    #[inline(always)]
    fn dot(&self, rhs: &Self) -> f32 {
        let mut output = 0.0;
        for (n1, n2) in self.contents.iter().zip(rhs.contents.iter()) {
            output += n1 * n2;
        }
        output
    }
}

impl<const L: usize> Add for &Vct<L> {
    type Output = Vct<L>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Vct::<L>::ZERO;
        for ((n, l), r) in output
            .contents
            .iter_mut()
            .zip(self.contents.iter())
            .zip(rhs.contents.iter())
        {
            *n += l + r;
        }
        output
    }
}

impl<const L: usize> Mul for &Vct<L> {
    type Output = Vct<L>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::generate(|i| self[i] * rhs[i])
    }
}
impl<const L: usize> Div for &Vct<L> {
    type Output = Vct<L>;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output::generate(|i| self[i] / rhs[i])
    }
}

impl<const L: usize> Mul<f32> for Vct<L> {
    type Output = Vct<L>;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.contents.iter_mut().for_each(|v| *v *= rhs);
        self
    }
}
impl<const L: usize> Div<f32> for Vct<L> {
    type Output = Vct<L>;
    fn div(mut self, rhs: f32) -> Self::Output {
        self.contents.iter_mut().for_each(|v| *v /= rhs);
        self
    }
}

impl<const L: usize> Mul<f32> for &Vct<L> {
    type Output = Vct<L>;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output::generate(|i| self[i] * rhs)
    }
}

impl<const L: usize> MulAssign<f32> for Vct<L> {
    fn mul_assign(&mut self, rhs: f32) {
        self.contents.iter_mut().for_each(|v| *v *= rhs);
    }
}

impl<const L: usize> MulAssign for Vct<L> {
    fn mul_assign(&mut self, rhs: Self) {
        self.contents
            .iter_mut()
            .zip(rhs.contents.iter())
            .for_each(|(l, r)| *l *= r);
    }
}

impl From<&Vct<2>> for Point {
    #[inline(always)]
    fn from(val: &Vct<2>) -> Self {
        val.to_point()
    }
}
impl From<&Vct<3>> for Point {
    #[inline(always)]
    fn from(val: &Vct<3>) -> Self {
        val.to_point()
    }
}
impl From<&Vct<4>> for Point {
    #[inline(always)]
    fn from(val: &Vct<4>) -> Self {
        val.to_point()
    }
}

// lock behind sdl feature
impl Vct<2> {
    #[inline(always)]
    pub fn to_point(&self) -> Point {
        Point::new(self.contents[0] as i32, self.contents[1] as i32)
    }
}
impl Vct<3> {
    #[inline(always)]
    pub fn to_point(&self) -> Point {
        Point::new(self.contents[0] as i32, self.contents[1] as i32)
    }
}
impl Vct<4> {
    #[inline(always)]
    pub fn to_point(&self) -> Point {
        Point::new(self.contents[0] as i32, self.contents[1] as i32)
    }
}

impl<const L: usize> Index<usize> for Vct<L> {
    type Output = f32;
    #[inline(always)]
    fn index(&self, i: usize) -> &Self::Output {
        &self.contents[i]
    }
}

impl<const L: usize> IndexMut<usize> for Vct<L> {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.contents[i]
    }
}

impl<const L: usize> Display for Vct<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.contents)
    }
}

impl<const L: usize> From<[f32; L]> for Vct<L> {
    fn from(val: [f32; L]) -> Self {
        Vct { contents: val }
    }
}

// This is done so as to be able to scale a list of Vct
pub trait Scale<Rhs = f32> {
    fn scale(&mut self, rhs: Rhs);
}

impl<const L: usize, const S: usize> Scale for [Vct<L>; S] {
    fn scale(&mut self, rhs: f32) {
        self.iter_mut().for_each(|v| *v *= rhs);
    }
}
