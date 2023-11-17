use crate::color::*;
use euler::Vec3;

pub trait Object3d {
    fn get_dist(&self, pos: Vec3) -> f32;
    fn get_color(&self) -> Color;
}

pub struct Sphere {
    pos: Vec3,
    rad: f32,
    col: Color,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn from(pos: Vec3, rad: f32, col: Color) -> Self {
        Self { pos, rad, col }
    }
}

impl Object3d for Sphere {
    fn get_dist(&self, pos: Vec3) -> f32 {
        (pos - self.pos).length() - self.rad
    }
    fn get_color(&self) -> Color {
        self.col
    }
}

