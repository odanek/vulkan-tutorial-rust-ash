use std::sync::Arc;

use ash::{version::DeviceV1_0, vk};

use super::{device::VkDevice, raw_handle::VkRawHandle};

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
        log::debug!("Dropping fence");
        unsafe {
            self.device.handle.destroy_fence(self.handle, None);
        }
    }
}

impl VkRawHandle for VkFence {
    type Handle = vk::Fence;

    fn raw_handle(&self) -> Self::Handle {
        self.handle
    }
}