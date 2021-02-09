use std::sync::Arc;

use ash::{version::DeviceV1_0, vk};

use super::device::VkDevice;

#[derive(Clone)]
pub struct VkFence {
    device: Arc<VkDevice>,
    pub handle: vk::Fence,
}

impl VkFence {
    pub fn new(device: &Arc<VkDevice>) -> VkFence {
        let create_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);
        let handle = unsafe {
            device
                .handle
                .create_fence(&create_info, None)
                .expect("Unable t ocreate fence")
        };

        VkFence { 
            device: Arc::clone(device),
            handle 
        }
    }
}

impl Drop for VkFence {
    fn drop(&mut self) {
        unsafe {
            self.device.handle.destroy_fence(self.handle, None);
        }
    }
}