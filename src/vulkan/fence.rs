use ash::{version::DeviceV1_0, vk};

use super::device::VkDevice;

#[derive(Copy, Clone)]
pub struct VkFence {
    pub handle: vk::Fence,
}

impl VkFence {
    pub fn new(device: &VkDevice) -> VkFence {
        let create_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);
        let handle = unsafe {
            device
                .handle
                .create_fence(&create_info, None)
                .expect("Unable t ocreate fence")
        };

        VkFence { handle }
    }

    pub fn cleanup(&self, device: &VkDevice) {
        unsafe {
            device.handle.destroy_fence(self.handle, None);
        }
    }
}
