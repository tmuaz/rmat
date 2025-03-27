use crate::{prelude::traits::*, rendering::Poly, Vct};
use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut, Mul},
};

#[derive(Debug, Clone)]
pub struct Mat<const R: usize, const C: usize> {
    pub contents: [[f32; C]; R],
}

// What you are actually indexing is the row
// so how you *actually* index this is by doing mat[r][c] which is a pretty cool quirk
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

    pub fn generate<F>(generator: F) -> Self
    where
        F: Fn(usize, usize) -> f32,
    {
        let mut mat = Self::ZERO;
        // TODO: parallelize and somehow do it with uninitialized memory
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

    pub fn inverse(&self) -> Self {
        todo!();
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

    /* // wohoo const generics!!!
    pub fn flatten(&self) -> [f32; Self::ELEMENT_COUNT] {
        // TODO: get this uninitialized
        let mut output = [0.0; Self::ELEMENT_COUNT];

        // skip the bound checking, we KNOW the bounds
        // yes the performance benefit is "negligible" but this something you can only do with
        // const generics
        unsafe {
            let mut i = 0;
            for row in self.contents {
                // move C * sizeof<T> bytes forward
                std::ptr::copy_nonoverlapping(row.as_ptr(), output.as_mut_ptr().add(i), C);
                // now that we've copied C bytes, we move the pointer C bytes forward
                i += C;
            }
        }

        // safe way in-case paranoia
        /* let mut i = 0;
        for row in self.contents {
            output[i..i + C].copy_from_slice(&row);
            i += C;
        } */
        output
    } */

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

impl<const L: usize> Mat<L, L> {
    pub const fn identity() -> Self {
        let mut contents = [[0.0; L]; L];

        let mut i = 0;
        while i < L {
            contents[i][i] = 1.0;
            i += 1;
        }

        Self { contents }
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
impl<const R: usize> Mul<&Mat<R, R>> for Mat<R, R> {
    type Output = Self;
    fn mul(mut self, rhs: &Mat<R, R>) -> Self::Output {
        for r in 0..R {
            for c in 0..R {
                self[r][c] *= rhs[r][c]
            }
        }
        self
    }
}

impl<const R: usize, const C: usize> Mul<&Vct<C>> for &Mat<R, C> {
    type Output = Vct<R>;
    fn mul(self, rhs: &Vct<C>) -> Self::Output {
        let mut output = Self::Output::ZERO;
        for (i, row) in self.contents.iter().enumerate() {
            output[i] = Vct::from_array_ref(row).dot(rhs);
        }
        output
    }
}

impl<const R: usize> Mul<&mut Vct<R>> for &Mat<R, R> {
    type Output = ();
    fn mul(self, rhs: &mut Vct<R>) -> Self::Output {
        for (i, row) in self.contents.iter().enumerate() {
            rhs[i] = Vct::from_array_ref(row).dot(rhs);
        }
    }
}

impl<const R: usize, const S: usize> Mul<&mut [Vct<R>; S]> for &Mat<R, R> {
    type Output = ();
    fn mul(self, rhs: &mut [Vct<R>; S]) -> Self::Output {
        for vct in rhs.iter_mut() {
            self * vct;
        }
    }
}
impl Mul<&mut Poly> for &Mat<4, 4> {
    type Output = ();
    fn mul(self, rhs: &mut Poly) -> Self::Output {
        self * &mut rhs.vertices;
    }
}

impl<const L: usize> Mul<Vct<L>> for &Mat<L, L> {
    type Output = Vct<L>;
    fn mul(self, mut rhs: Vct<L>) -> Self::Output {
        for (i, row) in self.contents.iter().enumerate() {
            rhs[i] = Vct::from_array_ref(row).dot(&rhs);
        }
        rhs
    }
}

impl<const R: usize, const C: usize> Into<Mat<R, C>> for [[f32; C]; R] {
    fn into(self) -> Mat<R, C> {
        Mat::from_arrays(self)
    }
}
