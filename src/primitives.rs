use crate::math::{vec3, Vec3};

pub trait Object3d {
    fn get_dist(&self, pos: Vec3) -> f32;
}

pub struct Sphere {
    pos: Vec3,
    rad: f32,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn from(pos: Vec3, rad: f32) -> Self {
        Self { pos, rad }
    }
}

impl Object3d for Sphere {
    fn get_dist(&self, pos: Vec3) -> f32 {
        (pos - self.pos).length() - self.rad
    }
}

pub struct Rec {
    pos: Vec3,
    sides: Vec3,
}

impl Rec {
    #[allow(dead_code)]
    pub fn from(pos: Vec3, sides: Vec3) -> Self {
        Self { pos, sides }
    }
}

impl Object3d for Rec {
    fn get_dist(&self, pos: Vec3) -> f32 {
        let q = (pos - self.pos).abs() - self.sides / 2.0;
        q.max(0.0).length() + q.x.max(q.y).max(q.z).min(0.0)
    }
}

// impl
