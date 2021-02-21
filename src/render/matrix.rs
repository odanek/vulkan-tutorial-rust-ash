use std::ops;

use super::{Vec3, Vec4};

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    data: [f32; 16]
}

#[cfg_attr(rustfmt, rustfmt::skip)]
pub const IDENT4: Mat4 = Mat4 {
    data: [
        1.0, 0.0, 0.0, 0.0, 
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ]
};

impl Mat4 {
    pub fn scale(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4 {
            #[cfg_attr(rustfmt, rustfmt::skip)]
            data: [
                x, 0.0, 0.0, 0.0,
                0.0, y, 0.0, 0.0,
                0.0, 0.0, z, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn vec_scale(vec: &Vec3) -> Mat4 {
        Self::scale(vec.x(), vec.y(), vec.z())
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4 {
            #[cfg_attr(rustfmt, rustfmt::skip)]
            data: [
                1.0, 0.0, 0.0, 0.0, 
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                x, y, z, 1.0
            ]            
        }
    }

    pub fn vec_translate(vec: &Vec3) -> Mat4 {
        Self::translate(vec.x(), vec.y(), vec.z())
    }

    pub fn rotate(radians: f32, axis: &Vec3) -> Mat4 {
        let c = radians.cos();
        let mc = 1.0 - c;
        let s = radians.sin();

        let x = axis.x();
        let y = axis.y();
        let z = axis.z();

        Mat4 {
            #[cfg_attr(rustfmt, rustfmt::skip)]
            data: [
                x * x * mc + c, x * y * mc + z * s, x * z * mc - y * s, 0.0,
                x * y * mc - z * s, y * y * mc + c, y * z * mc + x * s, 0.0,
                x * z * mc + y * s, y * z * mc - x * s, z * z * mc + c, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn rotate_x(radians: f32) -> Mat4 {
        let s = radians.sin();
        let c = radians.cos();

        Mat4 {
            #[cfg_attr(rustfmt, rustfmt::skip)]
            data:[
                1.0, 0.0, 0.0, 0.0,
                0.0, c, s, 0.0,
                0.0, -s, c, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn rotate_y(radians: f32) -> Mat4 {
        let s = radians.sin();
        let c = radians.cos();

        Mat4 {
            #[cfg_attr(rustfmt, rustfmt::skip)]
            data:[
                c, 0.0, -s, 0.0,
                0.0, 1.0, 0.0, 0.0,
                s, 0.0, c, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn rotate_z(radians: f32) -> Mat4 {
        let s = radians.sin();
        let c = radians.cos();

        Mat4 {
            #[cfg_attr(rustfmt, rustfmt::skip)]
            data:[
                c, s, 0.0, 0.0,
                -s, c, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn perspective(fov: f32, aspect: f32, near_clip: f32, far_clip: f32) -> Mat4 {
        let half_fov = fov / 2.0;
        let f = half_fov.cos() / half_fov.sin();
        let d = far_clip - near_clip;

        Mat4 {
            #[cfg_attr(rustfmt, rustfmt::skip)]
            data: [
                f / aspect, 0.0, 0.0, 0.0,
                0.0, -f, 0.0, 0.0,
                0.0, 0.0, -far_clip / d, -1.0,
                0.0, 0.0, -(far_clip * near_clip) / d, 0.0
            ]
        }
    }

    pub fn ortho(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Mat4 {
        let x = right - left;
        let y= top - bottom;
        let z= far - near;

        Mat4 {
            #[cfg_attr(rustfmt, rustfmt::skip)]
            data: [
                2.0 / x, 0.0, 0.0, 0.0,
                0.0, 2.0 / y, 0.0, 0.0,
                0.0, 0.0, -2.0 / z, 0.0,
                (left + right) / -x, (bottom + top) / -y, (near + far) / -z, 1.0
            ]
        }
    }

    pub fn look_at(eye: &Vec3, center: &Vec3, up: &Vec3) -> Mat4 {
        let f = (center - eye).unit();
        let s = f.cross(&up.unit());
        let u = s.unit().cross(&f);
        let m = Mat4 {
            #[cfg_attr(rustfmt, rustfmt::skip)]
            data: [
                s.x(), u.x(), -f.x(), 0.0,
                s.y(), u.y(), -f.y(), 0.0,
                s.z(), u.z(), -f.z(), 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        };
        m * (-eye).translation_mat()
    }

    pub fn transpose(&self) -> Mat4 {
        let data = &self.data;

        Mat4 {
            #[cfg_attr(rustfmt, rustfmt::skip)]
            data: [
                data[0], data[4], data[8], data[12],
                data[1], data[5], data[9], data[13],
                data[2], data[6], data[10], data[14],
                data[3], data[7], data[11], data[15]
            ]
        }
    }
}

impl ops::Add<Mat4> for Mat4 {
    type Output = Mat4;

    fn add(self, rhs: Mat4) -> Self::Output {
        Mat4 {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
                self.data[3] + rhs.data[3],
                self.data[4] + rhs.data[4],
                self.data[5] + rhs.data[5],
                self.data[6] + rhs.data[6],
                self.data[7] + rhs.data[7],
                self.data[8] + rhs.data[8],
                self.data[9] + rhs.data[9],
                self.data[10] + rhs.data[10],
                self.data[11] + rhs.data[11],
                self.data[12] + rhs.data[12],
                self.data[13] + rhs.data[13],
                self.data[14] + rhs.data[14],
                self.data[15] + rhs.data[15],
            ]
        }
    }
}

impl ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        &self * &rhs
    }
}

impl ops::Mul<&Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: &Mat4) -> Self::Output {
        &self * rhs
    }
}

impl ops::Mul<Mat4> for &Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        self * &rhs
    }
}

impl ops::Mul<&Mat4> for &Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: &Mat4) -> Self::Output {
        let mut data: [f32; 16] = [0f32; 16]; // TODO Not necessary
        let mut index = 0usize;

        for column in 0..4 {
            for row in 0..4 {
                let mut left = row;
                let mut right = column * 4;
                let mut sum = 0f32;

                for _ in 0..4 {
                    sum += self.data[left] * rhs.data[right];
                    left += 4;
                    right += 1;
                }
                data[index] = sum;
                index += 1;
            }
        }

        Mat4 {
            data
        }
    }
}

impl ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        &self * &rhs
    }
}

impl ops::Mul<&Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        &self * rhs
    }
}

impl ops::Mul<Vec4> for &Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self * &rhs
    }
}

impl ops::Mul<&Vec4> for &Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        let m = &self.data;
        let x = rhs.x();
        let y = rhs.y();
        let z = rhs.z();
        let w = rhs.w();

        #[cfg_attr(rustfmt, rustfmt::skip)]
        Vec4::new(
            m[0] * x + m[4] * y + m[8] * z + m[12] * w,
            m[1] * x + m[5] * y + m[9] * z + m[13] * w,
            m[2] * x + m[6] * y + m[10] * z + m[14] * w,
            m[3] * x + m[7] * y + m[11] * z + m[15] * w,
        )
    }
}