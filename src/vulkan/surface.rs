use ash::{extensions::khr::Surface, vk};

use super::{instance::VkInstance, physical_device::VkPhysicalDevice};

pub struct VkSurface {
    pub extension: Surface,
    pub handle: vk::SurfaceKHR,
}

pub struct VkSurfaceCapabilities {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl VkSurface {
    pub fn new(
        entry: &ash::Entry,
        instance: &VkInstance,
        window: &winit::window::Window,
    ) -> VkSurface {
        let extension = Surface::new(entry, &instance.handle);
        let handle = unsafe {
            ash_window::create_surface(entry, &instance.handle, window, None)
                .expect("Unable to create surface")
        };

        VkSurface { extension, handle }
    }

    pub fn physical_device_queue_support(
        &self,
        physical_device: &VkPhysicalDevice,
        queue_index: u32,
    ) -> bool {
        unsafe {
            self.extension
                .get_physical_device_surface_support(
                    physical_device.handle,
                    queue_index,
                    self.handle,
                )
                .expect("Unable to query surface support")
        }
    }

    pub fn get_physical_device_surface_capabilities(
        &self,
        physical_device: &VkPhysicalDevice,
    ) -> VkSurfaceCapabilities {
        unsafe {
            VkSurfaceCapabilities {
                capabilities: self
                    .extension
                    .get_physical_device_surface_capabilities(physical_device.handle, self.handle)
                    .expect("Unable to query surface capabilities"),
                formats: self
                    .extension
                    .get_physical_device_surface_formats(physical_device.handle, self.handle)
                    .expect("Unable to query surface formats"),
                present_modes: self
                    .extension
                    .get_physical_device_surface_present_modes(physical_device.handle, self.handle)
                    .expect("Unable to query surface presentation modes"),
            }
        }
    }
}

impl Drop for VkSurface {
    fn drop(&mut self) {
        log::debug!("Dropping surface");
        unsafe {
            self.extension.destroy_surface(self.handle, None);
        }
    }
}
