use ash::{extensions::khr::Surface, vk};

use super::physical_device::VkPhysicalDevice;

pub struct VkSurface {
    pub extension: Surface,
    pub surface: vk::SurfaceKHR,
}

pub struct VkSurfaceCapabilities {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl VkSurface {
    pub fn new(
        entry: &ash::Entry,
        instance: &ash::Instance,
        window: &winit::window::Window,
    ) -> VkSurface {
        let extension = Surface::new(entry, instance);
        let surface = unsafe { ash_window::create_surface(entry, instance, window, None).unwrap() };

        VkSurface { extension, surface }
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
                    self.surface,
                )
                .expect("Unable to query surface support")
        }
    }

    pub fn get_physical_device_surface_capabilties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkSurfaceCapabilities {
        unsafe {
            VkSurfaceCapabilities {
                capabilities: self
                    .extension
                    .get_physical_device_surface_capabilities(physical_device, self.surface)
                    .expect("Unable to query surface capabilities"),
                formats: self
                    .extension
                    .get_physical_device_surface_formats(physical_device, self.surface)
                    .expect("Unable to query surface formats"),
                present_modes: self
                    .extension
                    .get_physical_device_surface_present_modes(physical_device, self.surface)
                    .expect("Unable to query surface presentation modes"),
            }
        }
    }
}

impl Drop for VkSurface {
    fn drop(&mut self) {
        log::debug!("Dropping surface");
        unsafe {
            self.extension.destroy_surface(self.surface, None);
        }
    }
}
