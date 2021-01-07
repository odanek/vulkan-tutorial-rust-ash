use winit::window::Window;

use ash::Entry;

use super::{
    debug::VkValidation, device::VkDevice, instance::VkInstance, physical_device::VkPhysicalDevice,
    settings::VkSettings, surface::VkSurface, swap_chain::VkSwapChain,
};

pub struct VkContext {
    _swap_chain: VkSwapChain,
    _device: VkDevice,
    _physical_device: VkPhysicalDevice,
    _surface: VkSurface,
    _validation: Option<VkValidation>,
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
        let surface = VkSurface::new(&entry, &instance, window);
        let physical_device = VkPhysicalDevice::new(&instance, &surface);
        let device = VkDevice::new(&instance, &physical_device, &surface);

        let window_size = window.inner_size();
        let swap_chain = VkSwapChain::new(
            &instance,
            &physical_device,
            &device,
            &surface,
            &[window_size.width, window_size.height],
        );

        VkContext {
            _swap_chain: swap_chain,
            _device: device,
            _physical_device: physical_device,
            _surface: surface,
            _validation: validation,
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
