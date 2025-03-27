use std::ops::Mul;

use sdl2::render::{Canvas, RenderTarget};

use crate::math::*;
type Vert = Vct<4>;
#[derive(Debug, Clone)]
pub struct Poly {
    pub vertices: [Vert; 3],
}

impl Poly {
    pub fn new(mut verts: [Vct<4>; 3]) -> Self {
        verts.iter_mut().for_each(|v| v[3] = 0.0);
        Self { vertices: verts }
    }
    pub fn draw<T: RenderTarget>(
        &self,
        canvas: &mut Canvas<T>,
        transform_matrix: &Mat<4, 4>,
    ) -> Result<(), String> {
        let mut transformed = self.clone();
        transform_matrix * &mut transformed.vertices;
        // since the transformed matrix is meant to be homogenous we are going to divide everything
        // by w in order to get itself back
        transformed.vertices.iter_mut().for_each(Vert::homogenize);
        for i in 0..2 {
            let j = i + 1;
            canvas.draw_line(&transformed.vertices[i], &transformed.vertices[j])?;
        }
        canvas.draw_line(&transformed.vertices[2], &transformed.vertices[0])?;
        Ok(())
    }
}

impl From<[[f32; 4]; 3]> for Poly {
    fn from(val: [[f32; 4]; 3]) -> Self {
        Poly {
            vertices: val.map(|a| a.into()),
        }
    }
}

impl From<[Vct<4>; 3]> for Poly {
    fn from(val: [Vct<4>; 3]) -> Self {
        Poly { vertices: val }
    }
}

impl Scale for Poly {
    fn scale(&mut self, rhs: f32) {
        self.vertices.scale(rhs);
    }
}
