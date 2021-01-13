use ash::{version::DeviceV1_0, vk};

use super::device::VkDevice;

pub struct VkCommandPool {
    pub handle: vk::CommandPool,
}

impl VkCommandPool {
    pub fn new(device: &VkDevice) -> VkCommandPool {
        log::info!("Creating command pool");
        
        let create_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(device.graphics_queue_family)
            .flags(vk::CommandPoolCreateFlags::empty());

        let handle = unsafe {
            device
                .create_command_pool(&create_info, None)
                .expect("Unable to create command pool")
        };

        VkCommandPool { handle }
    }

    pub fn cleanup(&mut self, device: &VkDevice) {
        log::debug!("Dropping command pool");
        unsafe {
            device.handle.destroy_command_pool(self.handle, None);
        }
    }
}
