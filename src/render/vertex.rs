use super::Vec3;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
}