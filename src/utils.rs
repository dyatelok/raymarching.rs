use crate::math::{vec3, Vec3};

pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    base1: Vec3,
    base2: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: vec3![],
            dir: vec3![1.0, 0.0, 0.0],
            base1: vec3![0.0, 1.0, 0.0],
            base2: vec3![0.0, 0.0, 1.0],
        }
    }
    pub fn from(pos: Vec3, dir: Vec3, base1: Vec3, base2: Vec3) -> Self {
        Self {
            pos,
            dir,
            base1,
            base2,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            pos: self.pos,
            dir: (self.dir + u * self.base1 + v * self.base2).normalize(),
        }
    }
}

pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray {
    #[allow(dead_code)]
    pub fn from(pos: Vec3, dir: Vec3) -> Self {
        Self { pos, dir }
    }
}

pub fn union(d1: f32, d2: f32) -> f32 {
    d1.min(d2)
}

pub fn substraction(d1: f32, d2: f32) -> f32 {
    (-d2).max(d1)
}

pub fn intersection(d1: f32, d2: f32) -> f32 {
    d1.max(d2)
}

pub fn smooth_union(d1: f32, d2: f32, k: f32) -> f32 {
    let h = clamp(0.5 + 0.5 * (d2 - d1) / k, 0.0, 1.0);
    mix(d2, d1, h) - k * h * (1.0 - h)
}

pub fn smooth_substraction(d1: f32, d2: f32, k: f32) -> f32 {
    let h = clamp(0.5 - 0.5 * (d1 + d2) / k, 0.0, 1.0);
    mix(d1, -d2, h) + k * h * (1.0 - h)
}

pub fn smooth_intersection(d1: f32, d2: f32, k: f32) -> f32 {
    let h = clamp(0.5 - 0.5 * (d2 - d1) / k, 0.0, 1.0);
    mix(d2, d1, h) + k * h * (1.0 - h)
}

pub fn clamp(f: f32, min: f32, max: f32) -> f32 {
    f.max(min).min(max)
}

pub fn mix(a: f32, b: f32, k: f32) -> f32 {
    a + (b - a) * k
}
