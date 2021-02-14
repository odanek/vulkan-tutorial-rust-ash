use std::{collections::HashSet, sync::Arc};

use ash::{version::DeviceV1_0, version::InstanceV1_0, vk};

use super::{
    physical_device::VkPhysicalDevice, queue_family::VkQueueFamily, surface::VkSurface, utils,
};

pub struct VkDevice {
    pub physical_device: Arc<VkPhysicalDevice>,
    pub handle: ash::Device,

    // TODO: Remove these from VkDevice
    pub graphics_queue: vk::Queue,
    pub graphics_queue_family: u32,
    pub presentation_queue: vk::Queue,
    pub presentation_queue_family: u32,

    // TODO: Where to put this?
    pub swapchain_image_views: Vec<vk::ImageView>,
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
        for &queue_family in unique_queue_families.iter() {
            let queue_create_info = vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_family)
                .queue_priorities(&queue_priorities)
                .build();
            queue_infos.push(queue_create_info);
        }

        let extensions = VkPhysicalDevice::get_required_device_extensions();
        let extension_names = utils::coerce_extension_names(&extensions);
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
            swapchain_image_views: Vec::new(),
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
