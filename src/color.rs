use std::ops::{Add, AddAssign, Div, Mul};
pub const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn to_rgba8(self) -> [u8; 4] {
        [
            (gamma_encode(self.r) * 255f32) as u8,
            (gamma_encode(self.g) * 255f32) as u8,
            (gamma_encode(self.b) * 255f32) as u8,
            255u8,
        ]
    }

    pub fn from_rgba8(rgba8: [u8; 4]) -> Self {
        Color {
            r: gamma_decode(rgba8[0] as f32 / 255f32),
            g: gamma_decode(rgba8[1] as f32 / 255f32),
            b: gamma_decode(rgba8[2] as f32 / 255f32),
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
        }
    }

    pub fn black() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl Div<f32> for Color {
    type Output = Self;
    fn div(self, other: f32) -> Self::Output {
        Self::Output {
            r: self.r / other,
            b: self.b / other,
            g: self.g / other,
        }
    }
}

impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::default(), Add::add)
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, other: Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}
