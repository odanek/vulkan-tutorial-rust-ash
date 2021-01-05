use std::ops::Deref;

use ash::vk;

pub struct VkDevice(vk::Device);

impl VkDevice {
    // pub fn new() -> VkDevice {
    //     VkDevice()
    // }
}

impl Deref for VkDevice {
    type Target = vk::Device;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}