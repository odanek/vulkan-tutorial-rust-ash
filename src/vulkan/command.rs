use ash::{version::DeviceV1_0, vk};

use super::{device::VkDevice};

pub struct VkCommandPool {
    pub handle: vk::CommandPool,
    pub buffers: Vec<vk::CommandBuffer>,
}

impl VkCommandPool {
    pub fn new(device: &VkDevice) -> VkCommandPool {
        log::info!("Creating command pool");

        let pool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(device.graphics_queue_family)
            .flags(vk::CommandPoolCreateFlags::empty());

        let handle = unsafe {
            device
                .create_command_pool(&pool_info, None)
                .expect("Unable to create command pool")
        };

        VkCommandPool { handle, buffers: Vec::new() }
    }

    pub fn create_command_buffers(&mut self, device: &VkDevice, count: u32) {
        log::info!("Creating command buffers");

        let buffer_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(self.handle)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(count);

        self.buffers = unsafe {
            device
                .handle
                .allocate_command_buffers(&buffer_info)
                .unwrap()
        };
    }

    pub fn clear_command_buffers(&mut self, device: &VkDevice) {
        unsafe {
            device.free_command_buffers(self.handle, &self.buffers)
        };
        self.buffers.clear();
    }

    pub fn cleanup(&mut self, device: &VkDevice) {
        log::debug!("Dropping command pool");
        unsafe {
            device.handle.destroy_command_pool(self.handle, None);
        }
    }
}
