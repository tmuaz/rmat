use super::traits::*;
use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut, Mul},
};

#[derive(Debug, Clone)]
pub struct Mat<const R: usize, const C: usize> {
    contents: [[f32; C]; R],
}

// so how you *actually* index this is by doing mat[r][c] which is a pretty cool quirk
// What you are actually indexing is the row
impl<const R: usize, const C: usize> Index<usize> for Mat<R, C> {
    type Output = [f32; C];
    fn index(&self, r: usize) -> &Self::Output {
        &self.contents[r]
    }
}

impl<const R: usize, const C: usize> IndexMut<usize> for Mat<R, C> {
    fn index_mut(&mut self, r: usize) -> &mut Self::Output {
        &mut self.contents[r]
    }
}

// TODO: probably optimize? but it's just a display function it shouldn't matter

impl<const R: usize, const C: usize> Display for Mat<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PADDING: usize = 8;
        let mut array_iter = self.contents.iter();
        let first_row = array_iter.next();
        if let Some(row) = first_row {
            writeln!(f, "{:-<26}", "")?;
            write!(f, "│")?;
            for v in row {
                write!(f, "{:>width$}│", v, width = PADDING)?;
            }
        }
        for rows in array_iter {
            writeln!(f)?;
            write!(f, "│")?;
            for v in rows {
                write!(f, "{:>width$}│", v, width = PADDING)?;
            }
        }
        writeln!(f)?;
        write!(f, "└")?;
        write!(f, "{:-}", "")?;

        Ok(())
    }
}

impl<const R: usize, const C: usize> Mat<R, C> {
    pub const ELEMENT_COUNT: usize = R * C;
    pub const ZERO: Self = Self {
        contents: [[0.0; C]; R],
    };

    pub const fn fill(val: f32) -> Self {
        Self {
            contents: [[val; C]; R],
        }
    }

    /// Given a function that takes in a row and column and outputs an f32, generate a matrix
    /// based on the generator
    pub fn generate(generator: fn(usize, usize) -> f32) -> Self {
        let mut mat = Self::ZERO;
        // TODO: parallelize
        mat.contents.iter_mut().enumerate().for_each(|(r, row)| {
            row.iter_mut()
                .enumerate()
                .for_each(|(c, v)| *v = generator(r, c))
        });
        mat
    }

    #[inline(always)]
    pub fn from_arrays(arrays: [[f32; C]; R]) -> Self {
        Self { contents: arrays }
    }

    pub fn transpose(&self) -> Mat<C, R> {
        // initialize new matrix
        let mut new: Mat<C, R> = Mat::ZERO;
        for (r, arr) in self.contents.iter().enumerate() {
            unsafe {
                for (c, &value) in arr.iter().enumerate() {
                    let new_val = new.contents.get_unchecked_mut(c).get_unchecked_mut(r);
                    *new_val = value;
                }
            }
        }
        new
    }

    #[inline(always)]
    pub fn row_mat<const L: usize>(arr: [f32; L]) -> Mat<1, L> {
        Mat { contents: [arr] }
    }

    // wohoo const generics!!!
    pub fn flatten(&self) -> [f32; Self::ELEMENT_COUNT] {
        let mut output = [0.0; Self::ELEMENT_COUNT];
        self.contents.iter().enumerate().for_each(|(r, row)| {
            row.iter()
                .enumerate()
                .for_each(|(c, v)| output[r * 3 + c] = *v)
        });
        output
    }

    #[inline(always)]
    pub fn col_mat<const L: usize>(arr: [f32; L]) -> Mat<L, 1> {
        let mut contents = [[0.0; 1]; L];
        for (idx, row) in contents.iter_mut().enumerate() {
            row[0] = arr[idx];
        }
        Mat { contents }
    }

    pub fn col(&self, c: usize) -> [f32; C] {
        let mut output = [0.0; C];
        for (i, row) in self.contents.iter().enumerate() {
            output[i] = row[c];
        }
        output
    }

    pub fn sum(&self) -> f32 {
        self.contents.map(|arr| arr.iter().sum()).iter().sum()
    }

    pub fn normalized_sum(&self) -> f32 {
        self.sum() / Self::ELEMENT_COUNT as f32
    }

    pub fn normalized_dot(&self, rhs: &Self) -> f32 {
        self.dot(rhs) / Self::ELEMENT_COUNT as f32
    }
}

impl<const R: usize, const C: usize> Dot<Mat<R, C>> for Mat<R, C> {
    fn dot(&self, rhs: &Self) -> f32 {
        let mut output = 0.0;
        for (l, r) in self.contents.iter().zip(rhs.contents.iter()) {
            for (le, re) in l.iter().zip(r.iter()) {
                output += le * re;
            }
        }
        output
    }
}

impl<const R: usize, const C: usize> Add for Mat<R, C> {
    type Output = Mat<R, C>;
    fn add(mut self, rhs: Self) -> Self::Output {
        for (arr, rhs_array) in self.contents.iter_mut().zip(rhs.contents.iter()) {
            for (v, rhs_v) in arr.iter_mut().zip(rhs_array.iter()) {
                *v += rhs_v;
            }
        }
        self
    }
}

// ok so what this means is that there are 3 const generics, R C and K.
// We are multiplying with another matrix with C rows and K columns so the trait generic is
// Mul<C,K>
// we are using &s because I want to borrow the value of matrix instead of taking direct ownership
// which sounds very awful in general
impl<const R: usize, const C: usize, const K: usize> Mul<&Mat<C, K>> for &Mat<R, C> {
    type Output = Mat<R, K>;
    fn mul(self, rhs: &Mat<C, K>) -> Self::Output {
        let mut output: Mat<R, K> = Mat::ZERO;
        for r in 0..R {
            for k in 0..K {
                for c in 0..C {
                    output[r][k] = self[r][c] * rhs[c][k]
                }
            }
        }
        output
    }
}

fn dot<const L: usize>(arr1: &[f32; L], arr2: &[f32; L]) -> f32 {
    arr1.iter().zip(arr2.iter()).map(|(n1, n2)| n1 * n2).sum()
}
