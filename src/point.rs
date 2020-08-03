use crate::vec3::Vec3;

pub type Point = Vec3;

impl Point {
    pub fn origin() -> Self {
        Self::zero()
    }
}
