use std::sync::Arc;

use ash::{version::DeviceV1_0, vk};

use super::device::VkDevice;

pub struct VkSemaphore {
    device: Arc<VkDevice>,
    pub handle: vk::Semaphore,
}

impl VkSemaphore {
    pub fn new(device: &Arc<VkDevice>) -> VkSemaphore {
        let create_info = vk::SemaphoreCreateInfo::builder();
        let handle = unsafe {
            device
                .handle
                .create_semaphore(&create_info, None)
                .expect("Unable t ocreate a semaphore")
        };

        VkSemaphore { 
            device: Arc::clone(device),
            handle 
        }
    }
}

impl Drop for VkSemaphore {
    fn drop(&mut self) {
        unsafe {
            self.device.handle.destroy_semaphore(self.handle, None);
        }
    }
}
