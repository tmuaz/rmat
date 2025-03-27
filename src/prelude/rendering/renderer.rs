use sdl2::{
    pixels::Color,
    render::{Canvas, RenderTarget},
};

use super::{super::math::*, polygon::Poly};

pub struct Renderer {
    polys: Vec<Poly>,
    transform_matrix: Mat<4, 4>,
}

const IDENTITY_MAT: Mat<4, 4> = Mat::<4, 4>::identity();

impl Renderer {
    pub fn new(polys: Vec<Poly>, center: Vct<2>) -> Self {
        Self {
            polys,
            transform_matrix: Self::compute_transform_matrix(center),
        }
    }

    pub fn modify_polys<F>(&mut self, f: F)
    where
        F: Fn(&mut Poly),
    {
        self.polys.iter_mut().for_each(f);
    }

    fn compute_transform_matrix(center: Vct<2>) -> Mat<4, 4> {
        let mut transform_matrix = IDENTITY_MAT;
        // we want to flip the Y because SDL draws top to bottom
        transform_matrix[1][1] = -1.0;
        // we want to shift to the center
        transform_matrix[0][3] = center[0];
        transform_matrix[1][3] = center[1];
        transform_matrix
    }

    pub fn set_center(&mut self, center: Vct<2>) {
        self.transform_matrix = Self::compute_transform_matrix(center);
    }

    pub fn draw<T: RenderTarget>(&self, canvas: &mut Canvas<T>) -> Result<(), String> {
        // we have to transform the points to use
        for poly in &self.polys {
            poly.draw(canvas, &self.transform_matrix)?;
        }
        Ok(())
    }
}
