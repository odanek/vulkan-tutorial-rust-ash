use std::ops;

use super::Vec3;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Mat4 {
    data: [f32; 16]
}

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
            data: [
                1.0, 0.0, 0.0, x, 
                0.0, 1.0, 0.0, y,
                0.0, 0.0, 1.0, z,
                0.0, 0.0, 0.0, 1.0
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
        let d = near_clip - far_clip;

        Mat4 {
            data: [
                f / aspect, 0.0, 0.0, 0.0,
                0.0, f, 0.0, 0.0,
                0.0, 0.0, (near_clip + far_clip) / d, -1.0,
                0.0, 0.0, (2.0 * near_clip * far_clip) / d, 0.0
            ]
        }
    }

    pub fn ortho(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Mat4 {
        let x = right - left;
        let y= top - bottom;
        let z= far - near;

        Mat4 {
            data: [
                2.0 / x, 0.0, 0.0, 0.0,
                0.0, 2.0 / y, 0.0, 0.0,
                0.0, 0.0, -2.0 / z, 0.0,
                (left + right) / -x, (bottom + top) / -y, (near + far) / -z, 1.0
            ]
        }
    }

    pub fn look_at(eye: &Vec3, front: &Vec3, up: &Vec3) -> Mat4 {
        let s = front.cross(up);
        let u = s.cross(front);
        let m = Mat4 {
            data: [
                s.x(), u.x(), -front.x(), 0.0,
                s.y(), u.y(), -front.y(), 0.0,
                s.z(), u.z(), -front.z(), 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        };
        m * (-eye).translation_mat()
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