use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn to_rgb(&self) -> image::Rgb<u8> {
        image::Rgb([
            (self.r * 256.0) as u8,
            (self.g * 256.0) as u8,
            (self.b * 256.0) as u8,
        ])
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl Add<Color> for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, factor: f32) -> Self {
        Color::new(self.r * factor, self.g * factor, self.b * factor)
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, factor: f32) {
        self.r *= factor;
        self.g *= factor;
        self.b *= factor;
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl MulAssign<Color> for Color {
    fn mul_assign(&mut self, other: Self) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}
impl Div<f32> for Color {
    type Output = Self;
    fn div(self, factor: f32) -> Self {
        Color::new(self.r / factor, self.g / factor, self.b / factor)
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, factor: f32) {
        self.r /= factor;
        self.g /= factor;
        self.b /= factor;
    }
}

impl Sub<f32> for Color {
    type Output = Self;
    fn sub(self, factor: f32) -> Self {
        Color::new(self.r - factor, self.g - factor, self.b - factor)
    }
}

impl SubAssign<f32> for Color {
    fn sub_assign(&mut self, factor: f32) {
        self.r -= factor;
        self.g -= factor;
        self.b -= factor;
    }
}
