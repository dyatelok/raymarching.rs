use crate::color::Color;
// use crate::primitives::*;
use crate::utils::*;
use euler::vec3;

pub const SKY_COLOR: Color = Color::SKYBLUE;

pub fn construct_camera(t: f32) -> Camera {
    let pos = vec3!(t.cos(), t.sin(), 0.5) * 3.0;
    let dir = vec3![] - pos.normalize() * 1.0;
    let base0 = vec3![0.0, 0.0, 1.0];
    let base1 = vec3![] - base0.cross(dir).normalize();
    let base2 = dir.cross(base1).normalize();
    Camera::from(pos, dir, base1, base2)
}

// fn construct_objects(_t: f32) -> Vec<Box<dyn Object3d + Sync>> {
//     vec![Box::new(Sphere::from(
//         vec3![0.0, 0.0, 0.0],
//         1.0,
//         Color::RED,
//     ))]
// }

// pub fn construct_scene(t: f32) -> (Camera, Vec<Box<dyn Object3d + Sync>>) {
//     (construct_camera(), construct_objects(t))
// }

