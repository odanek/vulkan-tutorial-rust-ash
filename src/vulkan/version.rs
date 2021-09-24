use std::fmt;

use ash::vk::{api_version_major, api_version_minor, api_version_patch};

pub struct VkVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl VkVersion {
    pub fn parse(value: u32) -> VkVersion {
        let major = api_version_major(value);
        let minor = api_version_minor(value);
        let patch = api_version_patch(value);
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
