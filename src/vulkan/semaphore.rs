use ash::{version::DeviceV1_0, vk};

use super::device::VkDevice;

pub struct VkSemaphore {
    pub handle: vk::Semaphore,
}

impl VkSemaphore {
    pub fn new(device: &VkDevice) -> VkSemaphore {
        let create_info = vk::SemaphoreCreateInfo::builder();
        let handle = unsafe { device.handle.create_semaphore(&create_info, None).expect("Unable t ocreate a semaphore") };

        VkSemaphore {
            handle,
        }
    }

    pub fn cleanup(&self, device: &VkDevice) {
        unsafe {
            device.handle.destroy_semaphore(self.handle, None);
        }
    }
}