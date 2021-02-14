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

    pub fn create_command_buffers(&self, count: u32) -> Vec<vk::CommandBuffer> {
        log::info!("Creating {} command buffers", count);

        let buffer_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(self.handle)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(count);

        unsafe {
            self.device
                .handle
                .allocate_command_buffers(&buffer_info)
                .expect("Unable to allocate command buffers")
        }
    }

    pub fn clear_command_buffers(&self, buffers: &Vec<vk::CommandBuffer>) {
        unsafe {
            self.device
                .handle
                .free_command_buffers(self.handle, buffers)
        };
    }

    pub fn execute_one_time_commands(        
        &self,
        queue: vk::Queue,
        executor: impl FnOnce(&VkDevice, vk::CommandBuffer),
    ) {
        let command_buffers = self.create_command_buffers(1);
        let command_buffer = command_buffers[0];
        let device = &self.device;

        let command_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            device
                .handle
                .begin_command_buffer(command_buffer, &command_begin_info)
                .expect("Unable to begin command buffer")
        };

        // Execute user function
        executor(device, command_buffer);

        unsafe
        {
            device
                .handle
                .end_command_buffer(command_buffer)
                .expect("Unable to end command buffer");

            let submit_info = vk::SubmitInfo::builder().command_buffers(&command_buffers);
            let infos = [submit_info.build()];
            device
                .handle
                .queue_submit(queue, &infos, vk::Fence::null())
                .expect("Unable to submit queue");
            device
                .handle
                .queue_wait_idle(queue)
                .expect("Unable to wait for queue idle state");
        }

        self.clear_command_buffers(&command_buffers);
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
