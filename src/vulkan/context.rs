use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use ash::{Entry, vk};

use super::{command::VkCommandPool, debug::VkValidation, device::VkDevice, instance::VkInstance, physical_device::VkPhysicalDevice, render_pass::VkRenderPass, settings::VkSettings, surface::VkSurface, swap_chain::VkSwapChain, swap_chain_sync::VkSwapChainSync};

pub struct VkContext {
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub command_pool: VkCommandPool,
    pub render_pass: VkRenderPass,
    pub swap_chain: VkSwapChain,    
    pub device: Arc<VkDevice>,
    pub physical_device: Arc<VkPhysicalDevice>,
    pub surface: VkSurface,
    pub validation: Option<VkValidation>,
    pub instance: Arc<VkInstance>,
    pub entry: Box<ash::Entry>,
}

impl VkContext {
    pub fn new(window: &Window, settings: &VkSettings) -> VkContext {
        let entry = Box::new(Entry::new().expect("Failed to create Vulkan entry."));
        let instance = Arc::new(VkInstance::new(window, &entry, settings.validation));
        let validation = if settings.validation {
            Some(VkValidation::new(&entry, &instance))
        } else {
            None
        };
        let surface = VkSurface::new(&entry, &instance, window);
        let physical_device = Arc::new(VkPhysicalDevice::new(&instance, &surface));
        let device = Arc::new(VkDevice::new(&physical_device, &surface));

        let window_size = window.inner_size();
        let mut swap_chain = VkSwapChain::new(
            &device,
            &surface,
            &[window_size.width, window_size.height],
        );

        let render_pass = VkRenderPass::new(&device, &swap_chain);

        swap_chain.create_frame_buffers(&device, &render_pass);

        log::info!("Creating swap-chain command pool");
        let command_pool = VkCommandPool::new(&device, device.graphics_queue_family);
        log::info!("Creating swap-chain command buffers");
        let command_buffers = command_pool.create_command_buffers(swap_chain.image_count);        

        VkContext {
            command_buffers,
            command_pool,
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

    pub fn cleanup_swap_chain(&mut self) {
        self.swap_chain.cleanup_framebuffers(&self.device);        
        self.render_pass.cleanup(&self.device);
        self.swap_chain.cleanup(&self.device);
    }

    pub fn recreate_swap_chain(&mut self, size: PhysicalSize<u32>) {
        self.swap_chain = VkSwapChain::new(
            &self.device,
            &self.surface,
            &[size.width, size.height],
        );

        self.render_pass = VkRenderPass::new(&self.device, &self.swap_chain);        
        self.swap_chain
            .create_frame_buffers(&self.device, &self.render_pass);
        self.command_buffers = self.command_pool
            .create_command_buffers(self.swap_chain.image_count);
    }
}

impl Drop for VkContext {
    fn drop(&mut self) {
        self.cleanup_swap_chain();
    }
}
