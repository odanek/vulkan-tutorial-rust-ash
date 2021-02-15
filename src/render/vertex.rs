use super::{Vec3, vector::Vec2};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
    pub tex_coord: Vec2,
}