use euler::{vec3, Vec3};
// use rand::prelude::*;
use rayon::prelude::*;

use crate::color::*;
use crate::primitives::*;
use crate::scene::{construct_camera, SKY_COLOR};
use crate::utils::*;

const ITERATIONS_LIMIT: usize = 32;
const MAX_DISTANCE: f32 = 100.0;
const MIN_DISTANCE: f32 = 0.001;
const RAYS_PER_PIXEL_SQRT: usize = 1;

pub struct Marcher {
    side: usize,
    camera: Camera,
    screen: Vec<Color>,
}

impl Marcher {
    pub fn from(side: usize) -> Self {
        Self {
            side,
            camera: Camera::new(),
            screen: vec![Color::BLACK; side.pow(2)],
        }
    }
    fn set_scene(&mut self, t: f32) {
        self.camera = construct_camera(t);
    }
    pub fn draw(&mut self, t: f32, screen: &mut [u8]) {
        self.set_scene(t);

        let scr = self.side as f32 / 2.0;
        let mul = 1.0 / scr / (RAYS_PER_PIXEL_SQRT + 1) as f32;
        self.screen = (0..self.side.pow(2))
            .into_par_iter()
            .map(|pos| {
                let (x, y) = (pos / self.side, pos % self.side);
                let (x, y) = (x as f32, y as f32);
                let (x, y) = ((y - scr) / scr, (x - scr) / scr);
                let mut color = Color::BLACK;
                for i in 1..=RAYS_PER_PIXEL_SQRT {
                    for j in 1..=RAYS_PER_PIXEL_SQRT {
                        color += self.get_color(x + i as f32 * mul, y + j as f32 * mul);
                    }
                }
                color / RAYS_PER_PIXEL_SQRT.pow(2) as f32
            })
            .collect();

        for (pos, pix) in screen.chunks_exact_mut(4).enumerate() {
            pix.copy_from_slice(&self.screen[pos].into_u8());
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let ray: Ray = self.camera.get_ray(u, v);
        self.cast_ray(ray)
    }
    fn dist(pos: Vec3) -> f32 {
        // const MUL1: f32 = 5.0;
        // const MUL2: f32 = 0.25;
        // let displacement =
        // (MUL1 * pos.x).sin() * (MUL1 * pos.y).sin() * (MUL1 * pos.z).sin() * MUL2;
        let sphere_dist = Sphere::from(vec3![], 1.0, Color::RED).get_dist(pos);
        sphere_dist // + displacement
    }
    fn normal(pos: Vec3) -> Vec3 {
        const STEP: f32 = 0.01;
        let step_x: Vec3 = vec3![STEP, 0.0, 0.0];
        let step_y: Vec3 = vec3![0.0, STEP, 0.0];
        let step_z: Vec3 = vec3![0.0, 0.0, STEP];

        let gradient_x = Self::dist(pos + step_x) - Self::dist(pos - step_x);
        let gradient_y = Self::dist(pos + step_y) - Self::dist(pos - step_y);
        let gradient_z = Self::dist(pos + step_z) - Self::dist(pos - step_z);

        vec3![gradient_x, gradient_y, gradient_z].normalize()
    }
    fn cast_ray(&self, mut ray: Ray) -> Color {
        let light_pos = vec3![2.0, 5.0, 5.0];
        for _ in 0..ITERATIONS_LIMIT {
            let dist = Self::dist(ray.pos);

            if dist < MIN_DISTANCE {
                let norm = Self::normal(ray.pos);
                let dir_to_light: Vec3 = (ray.pos - light_pos).normalize();
                let intersity = norm.dot(dir_to_light).max(0.1);

                return Color::RED * intersity;
            }

            if dist > MAX_DISTANCE {
                break;
            }

            ray.pos += ray.dir * dist;
        }
        Color::BLACK
    }
}

