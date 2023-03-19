use rand::Rng;

/// A `Size` represents a region in space
#[derive(Clone, Copy, Default, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    /// Returns a new `Size` of the given dimensions
    pub fn new(width: f32, height: f32) -> Size {
        Size {
            width,
            height,
        }
    }

    /// Returns a random x coordinate within the bounds of this `Size`
    pub fn random_x<R: Rng>(&self, rng: &mut R) -> f32 {
        rng.gen_range(0.0..self.width)
    }

    /// Returns a random y coordinate within the bounds of this `Size`
    pub fn random_y<R: Rng>(&self, rng: &mut R) -> f32 {
        rng.gen_range(0.0..self.height)
    }

    /// Applies the given scale to this `Size`
    pub fn scale(&self, scale: f32) -> Size {
        Size {
            width: self.width * scale,
            height: self.height * scale,
        }
    }
}
