use crate::utils::*;

#[derive(Debug, Copy, Clone)]
pub struct PixelMapper {
    topleft: Complex, // complex number at image 0,0
    x_px_dist: Complex, // offset represented by 1 pixel in the x direction
    y_px_dist: Complex, // as above for y direction
}
impl PixelMapper {
    pub fn map(&self, x: usize, y: usize) -> Complex {
        let offset = (self.x_px_dist * x as f32) - (self.y_px_dist * y as f32);
        self.topleft + offset
    }

    pub fn new_radx(centre: Complex, radius: f32, angle: f32, wi: u32, hi: u32) -> Self {
        let r = radius;

        let k = (hi as f32) / (wi as f32);
        let s = radius * k;

        let r = rotate_vector(r, 0.0, angle);
        let s = rotate_vector(0.0, s, angle);

        let dx = centre.real + s.0 - r.0;
        let dy = centre.imag + s.1 - r.1;
        let topleft = Complex { real: dx, imag: dy };

        let xr = 2.0 * (r.0 / wi as f32);
        let xi = 2.0 * (r.1 / wi as f32);
        let x_px_dist = Complex { real: xr, imag: xi };
        let yr = 2.0 * (s.0 / hi as f32);
        let yi = 2.0 * (s.1 / hi as f32);
        let y_px_dist = Complex { real: yr, imag: yi };

        Self {
            topleft, x_px_dist, y_px_dist
        }
    }
    /// scale > 1 means increase resolution
    pub fn scale(&self, scale: f32) -> Self {
        Self {
            x_px_dist: self.x_px_dist / scale,
            y_px_dist: self.y_px_dist / scale,
            .. *self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn radx_sanity() {
        let pm = PixelMapper::new_radx(Complex::ZERO, 2.0, 0.0, 4, 4);
        assert_eq!(pm.topleft, Complex { real: -2.0, imag: 2.0 });
        assert_eq!(pm.x_px_dist, Complex { real: 1.0, imag: 0.0 });
        assert_eq!(pm.y_px_dist, Complex { real: 0.0, imag: 1.0 });
        assert_eq!(pm.map(0, 0), Complex { real: -2.0, imag: 2.0 });
        assert_eq!(pm.map(2, 2), Complex::ZERO);

        let pm = PixelMapper::new_radx(Complex::ZERO, 2.0, std::f32::consts::PI * 1.5, 4, 4);
        assert_eq!(pm.topleft, Complex { real: 2.0, imag: 2.0 });
    }
}

