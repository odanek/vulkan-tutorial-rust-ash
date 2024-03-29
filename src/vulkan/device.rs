use std::{collections::HashSet, sync::Arc};

use ash::vk;

use super::{
    command::VkCommandBuffer, physical_device::VkPhysicalDevice, queue_family::VkQueueFamily,
    surface::VkSurface, utils, VkBuffer, VkCommandPool, VkFence,
};

pub struct VkDevice {
    pub physical_device: Arc<VkPhysicalDevice>,
    pub handle: ash::Device,

    // TODO: Remove these from VkDevice
    pub graphics_queue: vk::Queue,
    pub graphics_queue_family: u32,
    pub presentation_queue: vk::Queue,
    pub presentation_queue_family: u32,
}

impl VkDevice {
    pub fn new(physical_device: &Arc<VkPhysicalDevice>, surface: &VkSurface) -> VkDevice {
        let graphics_queue_family = find_queue_family(physical_device, |family| {
            family.flags.contains(vk::QueueFlags::GRAPHICS)
        });
        log::info!("Choosing graphics queue family: {}", graphics_queue_family);

        let presentation_queue_family = find_queue_family(physical_device, |family| {
            surface.physical_device_queue_support(physical_device, family.index)
        });
        log::info!(
            "Choosing presentation queue family: {}",
            presentation_queue_family
        );

        let mut unique_queue_families = HashSet::new();
        unique_queue_families.insert(graphics_queue_family);
        unique_queue_families.insert(presentation_queue_family);

        let queue_priorities = [1.0f32];
        let mut queue_infos = Vec::new();
        for &queue_family in &unique_queue_families {
            let queue_create_info = vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_family)
                .queue_priorities(&queue_priorities)
                .build();
            queue_infos.push(queue_create_info);
        }

        let extensions = VkPhysicalDevice::get_required_device_extensions();
        let extension_names = utils::as_raw_handles(&extensions);
        let physical_device_features =
            vk::PhysicalDeviceFeatures::builder().sampler_anisotropy(true);
        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_features(&physical_device_features)
            .enabled_extension_names(&extension_names);
        let handle = unsafe {
            physical_device
                .instance
                .handle
                .create_device(physical_device.handle, &device_create_info, None)
                .expect("Unable to create logical device")
        };

        let graphics_queue = unsafe { handle.get_device_queue(graphics_queue_family, 0) };
        let presentation_queue = unsafe { handle.get_device_queue(presentation_queue_family, 0) };

        VkDevice {
            physical_device: Arc::clone(physical_device),
            handle,
            graphics_queue,
            graphics_queue_family,
            presentation_queue,
            presentation_queue_family,
        }
    }

    pub fn find_memory_type(
        &self,
        requirements: vk::MemoryRequirements,
        required_properties: vk::MemoryPropertyFlags,
    ) -> u32 {
        let mem_properties = self.physical_device.get_mem_properties();
        for i in 0..mem_properties.memory_type_count {
            if requirements.memory_type_bits & (1 << i) != 0
                && mem_properties.memory_types[i as usize]
                    .property_flags
                    .contains(required_properties)
            {
                return i;
            }
        }
        panic!("Failed to find suitable memory type.")
    }

    pub fn wait_idle(&self) {
        log::debug!("Waiting device idle");

        unsafe {
            self.handle
                .device_wait_idle()
                .expect("Failed to wait device idle!")
        };
    }

    pub fn wait_for_fences(&self, fences: &[&VkFence]) {
        let fence_handles = utils::as_raw_handles(fences);
        unsafe {
            self.handle
                .wait_for_fences(&fence_handles, true, std::u64::MAX)
                .expect("Waiting for fence failed");
        }
    }

    pub fn reset_fences(&self, fences: &[&VkFence]) {
        let fence_handles = utils::as_raw_handles(fences);
        unsafe {
            self.handle
                .reset_fences(&fence_handles)
                .expect("Fence reset failed");
        }
    }

    pub fn _get_mem_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        self.physical_device.get_mem_properties()
    }

    pub fn get_format_properties(&self, format: vk::Format) -> vk::FormatProperties {
        self.physical_device.get_format_properties(format)
    }

    pub fn _get_features(&self) -> vk::PhysicalDeviceFeatures {
        self.physical_device.get_features()
    }

    pub fn get_properties(&self) -> vk::PhysicalDeviceProperties {
        self.physical_device.get_properties()
    }

    pub fn get_max_usable_sample_count(&self) -> vk::SampleCountFlags {
        self.physical_device.get_max_usable_sample_count()
    }

    pub fn execute_one_time_commands(
        &self,
        pool: &Arc<VkCommandPool>,
        queue: vk::Queue,
        executor: impl FnOnce(&VkDevice, &VkCommandBuffer),
    ) {
        let command_buffer = VkCommandBuffer::new(pool, true);

        let command_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            self.handle
                .begin_command_buffer(command_buffer.handle, &command_begin_info)
                .expect("Unable to begin command buffer")
        };

        // Execute user function
        executor(self, &command_buffer);

        unsafe {
            self.handle
                .end_command_buffer(command_buffer.handle)
                .expect("Unable to end command buffer");

            let command_buffers = [command_buffer.handle];
            let submit_info = vk::SubmitInfo::builder().command_buffers(&command_buffers);
            let infos = [submit_info.build()];
            self.handle
                .queue_submit(queue, &infos, vk::Fence::null())
                .expect("Unable to submit queue");
            self.handle
                .queue_wait_idle(queue)
                .expect("Unable to wait for queue idle state");
        }
    }

    pub fn copy_buffer(
        &self,
        src: &VkBuffer,
        dst: &VkBuffer,
        command_pool: &Arc<VkCommandPool>,
        queue: vk::Queue,
    ) {
        self.execute_one_time_commands(command_pool, queue, |device, command_buffer| unsafe {
            let regions = [vk::BufferCopy {
                src_offset: 0,
                dst_offset: 0,
                size: src.size,
            }];
            device
                .handle
                .cmd_copy_buffer(command_buffer.handle, src.handle, dst.handle, &regions);
        });
    }
}

impl Drop for VkDevice {
    fn drop(&mut self) {
        log::debug!("Dropping logical device");
        unsafe {
            self.handle.destroy_device(None);
        }
    }
}

fn find_queue_family(
    physical_device: &VkPhysicalDevice,
    predicate: impl Fn(&VkQueueFamily) -> bool,
) -> u32 {
    physical_device
        .queue_families
        .iter()
        .position(|family| family.queue_count > 0 && predicate(family))
        .expect("Unable to find suitable queue family") as u32
}
