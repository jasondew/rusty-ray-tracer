use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn zero() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, factor: f32) -> Self {
        Vec3::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, factor: f32) -> Self {
        Vec3::new(self.x / factor, self.y / factor, self.z / factor)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, factor: f32) {
        self.x /= factor;
        self.y /= factor;
        self.z /= factor;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3::new(self.x * -1.0, self.y * -1.0, self.z * -1.0)
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;
    fn sub(self, factor: f32) -> Self {
        Vec3::new(self.x - factor, self.y - factor, self.z - factor)
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, factor: f32) {
        self.x -= factor;
        self.y -= factor;
        self.z -= factor;
    }
}
