use std::sync::Arc;

use ash::{version::DeviceV1_0, vk};

use super::device::VkDevice;

pub struct VkCommandPool {
    device: Arc<VkDevice>,
    pub handle: vk::CommandPool,
}

impl VkCommandPool {
    pub fn new(device: &Arc<VkDevice>, queue_family_index: u32) -> VkCommandPool {
        let pool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .flags(vk::CommandPoolCreateFlags::empty());

        let handle = unsafe {
            device
                .handle
                .create_command_pool(&pool_info, None)
                .expect("Unable to create command pool")
        };

        VkCommandPool {
            device: Arc::clone(device),
            handle,
        }
    }

    pub fn create_command_buffers(&self, device: &VkDevice, count: u32) -> Vec<vk::CommandBuffer> {
        log::info!("Creating {} command buffers", count);

        let buffer_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(self.handle)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(count);

        unsafe {
            device
                .handle
                .allocate_command_buffers(&buffer_info)
                .expect("Unable to allocate command buffers")
        }
    }

    pub fn create_command_buffer(&self) -> vk::CommandBuffer {
        self.create_command_buffers(&self.device, 1)[0]
    }

    pub fn clear_command_buffer(&self, buffer: vk::CommandBuffer) {
        let buffers = [buffer];
        unsafe {
            self.device
                .handle
                .free_command_buffers(self.handle, &buffers)
        };
    }

    pub fn clear_command_buffers(&self, buffers: &mut Vec<vk::CommandBuffer>) {
        unsafe {
            self.device
                .handle
                .free_command_buffers(self.handle, buffers)
        };
        buffers.clear();
    }
}

impl Drop for VkCommandPool {
    fn drop(&mut self) {
        log::debug!("Dropping command pool");
        unsafe {
            self.device.handle.destroy_command_pool(self.handle, None);
        }
    }
}
