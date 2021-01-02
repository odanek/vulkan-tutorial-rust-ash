use winit::window::Window;

use ash::{
    Entry,
};

use super::debug::*;
use super::instance::VkInstance;
use super::settings::VkSettings;

pub struct VkContext {
    validation: Option<VkValidation>,
    instance: VkInstance,
    entry: ash::Entry,
}

impl VkContext {
    pub fn new(window: &Window, settings: &VkSettings) -> VkContext {
        let entry = Entry::new().expect("Failed to create Vulkan entry.");
        let instance = VkInstance::new(window, settings, &entry); // Move to instance.rs
        let validation = if settings.validation {
            Some(VkValidation::new(&entry, &instance.as_raw()))
        } else {
            None
        };

        VkContext {
            entry,
            instance,
            validation,
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
