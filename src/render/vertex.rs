#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}