use std::ops::Deref;

use ash::{version::InstanceV1_0, version::DeviceV1_0, vk};

use super::physical_device::VkPhysicalDevice;

pub struct VkDevice {
    pub handle: ash::Device,
    pub graphics_queue: vk::Queue,
}

impl VkDevice {
    pub fn new(instance: &ash::Instance, physical_device: &VkPhysicalDevice) -> VkDevice {
        let graphics_queue_family = select_graphics_queue_family(physical_device);        
        let queue_priorities = [1.0f32];
        log::info!("Choosing graphics queue family: {}", graphics_queue_family);

        let queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(graphics_queue_family)
            .queue_priorities(&queue_priorities)
            .build();
        let queue_infos = [queue_create_info];

        let device_create_info = vk::DeviceCreateInfo::builder().queue_create_infos(&queue_infos);
        let handle = unsafe {
            instance
                .create_device(physical_device.handle, &device_create_info, None)
                .expect("Unable to create logical device")
        };
        
        let graphics_queue = unsafe { handle.get_device_queue(graphics_queue_family, 0) };

        VkDevice { 
            handle,
            graphics_queue
        }
    }
}

impl Drop for VkDevice {
    fn drop(&mut self) {
        println!("Dropping logical device");
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

fn select_graphics_queue_family(physical_device: &VkPhysicalDevice) -> u32 {
    physical_device
        .queue_families
        .iter()
        .position(|family| family.graphics)
        .unwrap() as u32
}
