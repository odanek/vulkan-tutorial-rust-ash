use std::sync::Arc;

use ash::vk;

use super::{device::VkDevice, utils::AsRawHandle};

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
            handle,
        }
    }
}

impl Drop for VkSemaphore {
    fn drop(&mut self) {
        log::debug!("Dropping semaphore");
        unsafe {
            self.device.handle.destroy_semaphore(self.handle, None);
        }
    }
}

impl AsRawHandle for &VkSemaphore {
    type Handle = vk::Semaphore;

    fn as_raw_handle(&self) -> Self::Handle {
        self.handle
    }
}
