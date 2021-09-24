use std::sync::Arc;

use ash::vk;

use super::device::VkDevice;

pub struct VkCommandPool {
    device: Arc<VkDevice>,
    pub handle: vk::CommandPool,
    pub queue_family_index: u32,
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
            queue_family_index,
        }
    }

    pub fn allocate_command_buffer(&self) -> vk::CommandBuffer {
        let buffer_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(self.handle)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let buffers = unsafe {
            self.device
                .handle
                .allocate_command_buffers(&buffer_info)
                .expect("Unable to allocate command buffers")
        };

        buffers[0]
    }

    pub fn free_command_buffer(&self, buffer: vk::CommandBuffer) {
        unsafe {
            self.device
                .handle
                .free_command_buffers(self.handle, &[buffer]);
        }
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

pub struct VkCommandBuffer {
    pool: Arc<VkCommandPool>,
    pub handle: vk::CommandBuffer,
    auto_release: bool,
}

impl VkCommandBuffer {
    pub fn new(pool: &Arc<VkCommandPool>, auto_release: bool) -> VkCommandBuffer {
        log::info!("Creating command buffer");
        VkCommandBuffer {
            pool: Arc::clone(pool),
            handle: pool.allocate_command_buffer(),
            auto_release,
        }
    }
}

impl Drop for VkCommandBuffer {
    fn drop(&mut self) {
        if self.auto_release {
            self.pool.free_command_buffer(self.handle);
        }
    }
}
