#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            data: [x, y, z]
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec4 {
    data: [f32; 4],
}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {
            data: [x, y, z, w]
        }
    }
}