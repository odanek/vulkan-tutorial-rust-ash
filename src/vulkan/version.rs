use std::fmt;

use ash::vk::{version_major, version_minor, version_patch};

pub struct VkVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl VkVersion {
    pub fn parse(value: u32) -> VkVersion {
        let major = version_major(value);
        let minor = version_minor(value);
        let patch = version_patch(value);
        VkVersion {
            major,
            minor,
            patch,
        }
    }
}

impl fmt::Display for VkVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
