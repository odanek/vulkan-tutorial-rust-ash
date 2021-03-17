use std::sync::Arc;

use winit::window::Window;

use ash::Entry;

use super::{
    debug::VkValidation, device::VkDevice, instance::VkInstance, physical_device::VkPhysicalDevice,
    settings::VkSettings, surface::VkSurface,
};

pub struct VkContext {
    pub device: Arc<VkDevice>,
    pub physical_device: Arc<VkPhysicalDevice>,
    pub surface: VkSurface,
    pub validation: Option<VkValidation>,
    pub instance: Arc<VkInstance>,
    pub entry: Box<ash::Entry>,
}

impl VkContext {
    pub fn new(window: &Window, settings: &VkSettings) -> VkContext {
        let entry = Box::new(unsafe { Entry::new().expect("Failed to create Vulkan entry.") });
        let instance = Arc::new(VkInstance::new(window, &entry, settings.validation));
        let validation = if settings.validation {
            Some(VkValidation::new(&entry, &instance))
        } else {
            None
        };
        let surface = VkSurface::new(&entry, &instance, window);
        let physical_device = Arc::new(VkPhysicalDevice::new(&instance, &surface));
        let device = Arc::new(VkDevice::new(&physical_device, &surface));

        VkContext {
            device,
            physical_device,
            surface,
            validation,
            instance,
            entry,
        }
    }
}
