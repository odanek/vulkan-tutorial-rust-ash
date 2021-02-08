use std::ops;

use super::Mat4;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    data: [f32; 3],
}

pub const ZERO3: Vec3 = Vec3 {
    data: [0.0, 0.0, 0.0],
};
pub const X3: Vec3 = Vec3 {
    data: [1.0, 0.0, 0.0],
};
pub const Y3: Vec3 = Vec3 {
    data: [0.0, 1.0, 0.0],
};
pub const Z3: Vec3 = Vec3 {
    data: [0.0, 0.0, 1.0],
};

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { data: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn length(&self) -> f32 {
        let x = self.data[0];
        let y = self.data[1];
        let z = self.data[2];
        (x * x + y * y + z * z).sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.data[0] * rhs.data[0] + self.data[1] * rhs.data[1] + self.data[2] * rhs.data[2]
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        let x = self.data[0];
        let y = self.data[1];
        let z = self.data[2];
        let rx = rhs.data[0];
        let ry = rhs.data[1];
        let rz = rhs.data[2];

        Vec3 {
            data: [y * rz - z * ry, z * rx - x * rz, x * ry - y * rx],
        }
    }

    pub fn unit(&self) -> Vec3 {
        self / self.length()
    }

    pub fn translation_mat(&self) -> Mat4 {
        Mat4::vec_translate(self)
    }

    pub fn scale_mat(&self) -> Mat4 {
        Mat4::vec_scale(self)
    }

    pub fn rotation_mat(&self, radians: f32) -> Mat4 {
        Mat4::rotate(radians, self)
    }

    pub fn homogenous(&self) -> Vec4 {
        Vec4 {
            data: [self.data[0], self.data[1], self.data[2], 1.0]
        }
    }

    // pub fn angle(&self) -> f32 {
    // self.data[1].atan2(self.data[0])
    // }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        &self + &rhs
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        &self + rhs
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
            ],
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        &self - &rhs
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        &self - rhs
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            data: [
                self.data[0] - rhs.data[0],
                self.data[1] - rhs.data[1],
                self.data[2] - rhs.data[2],
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        &self * rhs
    }
}

impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            data: [self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs],
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            data: [self * rhs.data[0], self * rhs.data[1], self * rhs.data[2]],
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        &self / rhs
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            data: [self.data[0] / rhs, self.data[1] / rhs, self.data[2] / rhs],
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -(&self)
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            data: [-self.data[0], -self.data[1], -self.data[2]],
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    data: [f32; 4],
}

pub const ZERO4H: Vec4 = Vec4 {
    data: [0.0, 0.0, 0.0, 1.0],
};
pub const X4H: Vec4 = Vec4 {
    data: [1.0, 0.0, 0.0, 1.0],
};
pub const Y4H: Vec4 = Vec4 {
    data: [0.0, 1.0, 0.0, 1.0],
};
pub const Z4H: Vec4 = Vec4 {
    data: [0.0, 0.0, 1.0, 1.0],
};

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { data: [x, y, z, w] }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn w(&self) -> f32 {
        self.data[3]
    }
}
