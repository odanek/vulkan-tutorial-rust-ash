use winit::window::Window;

use ash::Entry;

use super::{
    debug::VkValidation, device::VkDevice, instance::VkInstance, physical_device::VkPhysicalDevice,
    pipeline::VkPipeline, render_pass::VkRenderPass, settings::VkSettings, surface::VkSurface,
    swap_chain::VkSwapChain,
};

pub struct VkContext {
    pub pipeline: VkPipeline,
    pub render_pass: VkRenderPass,
    pub swap_chain: VkSwapChain,
    pub device: VkDevice,
    pub physical_device: VkPhysicalDevice,
    pub surface: VkSurface,
    pub validation: Option<VkValidation>,
    pub instance: VkInstance,
    pub entry: ash::Entry,
}

impl VkContext {
    pub fn new(window: &Window, settings: &VkSettings) -> VkContext {
        let entry = Entry::new().expect("Failed to create Vulkan entry.");
        let instance = VkInstance::new(window, &entry, settings.validation);
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

        let render_pass = VkRenderPass::new(&device, &swap_chain);
        let pipeline = VkPipeline::new(&device, &swap_chain, &render_pass);

        VkContext {
            pipeline,
            render_pass,
            swap_chain,
            device,
            physical_device,
            surface,
            validation,
            instance,
            entry,
        }
    }
}

impl Drop for VkContext {
    fn drop(&mut self) {
        self.pipeline.cleanup(&self.device);
        self.render_pass.cleanup(&self.device);
        self.swap_chain.cleanup(&self.device);
        self.device.cleanup();
        self.surface.cleanup();

        let validation = self.validation.take();
        if let Some(mut validation) = validation {
            validation.cleanup();
        }
        self.instance.cleanup();
    }
}
