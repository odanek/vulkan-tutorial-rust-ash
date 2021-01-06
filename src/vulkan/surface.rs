use ash::{extensions::khr::Surface, vk};

use super::physical_device::VkPhysicalDevice;

pub struct VkSurface {
    extension: Surface,
    pub surface: vk::SurfaceKHR,
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
}

impl Drop for VkSurface {
    fn drop(&mut self) {
        log::debug!("Dropping surface");
        unsafe {
            self.extension.destroy_surface(self.surface, None);
        }
    }
}
