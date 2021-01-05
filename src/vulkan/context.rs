use winit::window::Window;

use ash::Entry;

use super::{debug::VkValidation, device::VkDevice, instance::VkInstance, physical_device::VkPhysicalDevice, settings::VkSettings};

pub struct VkContext {
    _validation: Option<VkValidation>,
    // _device: VkDevice,
    _physical_device: VkPhysicalDevice,
    _instance: VkInstance,
    _entry: ash::Entry,
}

impl VkContext {
    pub fn new(window: &Window, settings: &VkSettings) -> VkContext {
        let entry = Entry::new().expect("Failed to create Vulkan entry.");
        let instance = VkInstance::new(window, settings, &entry); // Move to instance.rs
        let validation = if settings.validation {
            Some(VkValidation::new(&entry, &instance))
        } else {
            None
        };
        let physical_device = VkPhysicalDevice::new(&instance);
        // let device = VkDevice::new();

        VkContext {
            _validation: validation,
            // _device: device,
            _physical_device: physical_device,
            _instance: instance,
            _entry: entry,
        }
    }

    pub fn wait_device_idle(&self) {
        // TODO method on VkDevice
        // unsafe {
        // self.device
        //     .device_wait_idle()
        //     .expect("Failed to wait device idle!")
        // };
    }
}
