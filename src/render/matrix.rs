use std::ops;

use super::Vec3;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Mat4 {
    data: [f32; 16]
}

pub const IDENTITY4: Mat4 = Mat4 {
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

    pub fn rotate(radians: f32, axis: Vec3) -> Mat4 {
        // TODO
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