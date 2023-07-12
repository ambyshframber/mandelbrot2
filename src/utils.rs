#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub real: f32, pub imag: f32
}
impl Complex {
    pub const ZERO: Self = Complex { real: 0.0, imag: 0.0 };

    pub fn square(self) -> Complex {
        let real = self.real.powi(2) - self.imag.powi(2);
        let imag = self.real * self.imag * 2.0;
        Complex { real, imag }
    }
    pub fn magnitude(self) -> f32 {
        (self.real.powi(2) + self.imag.powi(2)).sqrt()
    }
    pub fn magnitude_squared(self) -> f32 {
        self.real.powi(2) + self.imag.powi(2)
    }
    pub fn fuzzy_eq(self, other: Self) -> bool {
        float_fuzzy_eq(self.real, other.real) && float_fuzzy_eq(self.imag, other.imag)
    }
}
impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag
        }
    }
}
impl std::ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag
        }
    }
}
impl std::ops::Mul<f32> for Complex {
    type Output = Complex;
    fn mul(self, rhs: f32) -> Self::Output {
        Complex {
            real: self.real * rhs,
            imag: self.imag * rhs
        }
    }
}
impl std::ops::Div<f32> for Complex {
    type Output = Complex;
    fn div(self, rhs: f32) -> Self::Output {
        Complex {
            real: self.real / rhs,
            imag: self.imag / rhs
        }
    }
}

pub fn float_fuzzy_eq(lhs: f32, rhs: f32) -> bool {
    if lhs.is_sign_positive() ^ rhs.is_sign_positive() { // different signs, can't be fuzzy-equal
        return false
    }
    else {
        let lhs_i = lhs.abs().to_bits();
        let rhs_i = rhs.abs().to_bits();
        let ulps = lhs_i.abs_diff(rhs_i);
        ulps <= 3
    }
}

pub fn rotate_vector(x1: f32, y1: f32, angle: f32) -> (f32, f32) {
    let sinth = angle.sin();
    let costh = angle.cos();
    let x2 = (x1 * costh) - (y1 * sinth);
    let y2 = (x1 * sinth) + (y1 * costh);
    (x2, y2)
}
