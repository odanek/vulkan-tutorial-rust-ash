use winit::{dpi::PhysicalSize, window::Window};

use ash::Entry;

use super::{
    command::VkCommandPool, debug::VkValidation, device::VkDevice, instance::VkInstance,
    physical_device::VkPhysicalDevice, pipeline::VkPipeline, render_pass::VkRenderPass,
    settings::VkSettings, surface::VkSurface, swap_chain::VkSwapChain,
    swap_chain_sync::VkSwapChainSync,
};

pub struct VkContext {
    pub command_pool: VkCommandPool,
    pub pipeline: VkPipeline,
    pub render_pass: VkRenderPass,
    pub swap_chain: VkSwapChain,
    pub swap_chain_sync: VkSwapChainSync,
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
        let mut swap_chain = VkSwapChain::new(
            &instance,
            &physical_device,
            &device,
            &surface,
            &[window_size.width, window_size.height],
        );

        let render_pass = VkRenderPass::new(&device, &swap_chain);
        let pipeline = VkPipeline::new(&device, &swap_chain, &render_pass);

        swap_chain.create_frame_buffers(&device, &render_pass);

        let mut command_pool = VkCommandPool::new(&device);
        command_pool.create_command_buffers(&device, swap_chain.framebuffers.len() as u32);

        let swap_chain_sync = VkSwapChainSync::new(&device, &swap_chain, 2);

        VkContext {
            command_pool,
            pipeline,
            render_pass,
            swap_chain,
            swap_chain_sync,
            device,
            physical_device,
            surface,
            validation,
            instance,
            entry,
        }
    }

    pub fn cleanup_swap_chain(&mut self) {
        self.swap_chain.cleanup_framebuffers(&self.device);
        self.command_pool.clear_command_buffers(&self.device);
        self.pipeline.cleanup(&self.device);
        self.render_pass.cleanup(&self.device);
        self.swap_chain.cleanup(&self.device);
    }

    pub fn recreate_swap_chain(&mut self, size: PhysicalSize<u32>) {
        self.swap_chain = VkSwapChain::new(
            &self.instance,
            &self.physical_device,
            &self.device,
            &self.surface,
            &[size.width, size.height],
        );

        self.render_pass = VkRenderPass::new(&self.device, &self.swap_chain);
        self.pipeline = VkPipeline::new(&self.device, &self.swap_chain, &self.render_pass);
        self.swap_chain
            .create_frame_buffers(&self.device, &self.render_pass);
        self.command_pool
            .create_command_buffers(&self.device, self.swap_chain.framebuffers.len() as u32);
    }
}

impl Drop for VkContext {
    fn drop(&mut self) {
        self.cleanup_swap_chain();

        self.swap_chain_sync.cleanup(&self.device);
        self.command_pool.cleanup(&self.device);
        self.device.cleanup();
        self.surface.cleanup();

        let validation = self.validation.take();
        if let Some(mut validation) = validation {
            validation.cleanup();
        }
        self.instance.cleanup();
    }
}
