use std::{collections::HashSet, ops::Deref};

use ash::{version::DeviceV1_0, version::InstanceV1_0, vk};

use super::{
    physical_device::VkPhysicalDevice, queue_family::VkQueueFamily, surface::VkSurface, utils,
};

pub struct VkDevice {
    pub handle: ash::Device,
    pub graphics_queue: vk::Queue,
    pub graphics_queue_family: u32,
    pub presentation_queue: vk::Queue,
    pub presentation_queue_family: u32,
}

impl VkDevice {
    pub fn new(
        instance: &ash::Instance,
        physical_device: &VkPhysicalDevice,
        surface: &VkSurface,
    ) -> VkDevice {
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
        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&extension_names);
        let handle = unsafe {
            instance
                .create_device(physical_device.handle, &device_create_info, None)
                .expect("Unable to create logical device")
        };

        let graphics_queue = unsafe { handle.get_device_queue(graphics_queue_family, 0) };
        let presentation_queue = unsafe { handle.get_device_queue(presentation_queue_family, 0) };

        VkDevice {
            handle,
            graphics_queue,
            graphics_queue_family,
            presentation_queue,
            presentation_queue_family,
        }
    }

    pub fn wait_idle(&self) {
        unsafe {
            self.handle.device_wait_idle().expect("Failed to wait device idle!")
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

impl Deref for VkDevice {
    type Target = ash::Device;

    fn deref(&self) -> &Self::Target {
        &self.handle
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
