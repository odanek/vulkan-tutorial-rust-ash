use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use ash::{vk, Entry};

use super::{
    command::VkCommandPool, debug::VkValidation, device::VkDevice, image::VkImage,
    instance::VkInstance, physical_device::VkPhysicalDevice, render_pass::VkRenderPass,
    settings::VkSettings, surface::VkSurface, swap_chain::VkSwapChain, VkTexture,
};

pub struct VkContext {
    pub msaa_samples: vk::SampleCountFlags,
    pub color_image: VkTexture,
    pub depth_image: VkTexture,
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

        let msaa_samples = device.get_max_usable_sample_count();
        log::info!("Using {:?} MSAA samples", msaa_samples);

        let window_size = window.inner_size();

        log::info!("Creating swap-chain command pool");
        let command_pool = VkCommandPool::new(&device, device.graphics_queue_family);

        let mut swap_chain =
            VkSwapChain::new(&device, &surface, &[window_size.width, window_size.height]);

        log::info!("Creating color image for MSAA");
        let color_image = VkImage::create_color_image(&device, swap_chain.format.format, swap_chain.extent, msaa_samples);

        let depth_format = VkImage::find_depth_format(&physical_device);

        log::info!("Creating depth image with format {:?}", depth_format);
        let depth_image = VkImage::create_depth_image(
            &device,
            &command_pool,
            device.graphics_queue,
            depth_format,
            swap_chain.extent,
            msaa_samples,
        );

        let render_pass = VkRenderPass::new(&device, &swap_chain, depth_format, msaa_samples);
        swap_chain.create_frame_buffers(&render_pass, &depth_image, &color_image);

        log::info!("Creating swap-chain command buffers");
        let command_buffers = command_pool.create_command_buffers(swap_chain.image_count);

        VkContext {
            color_image,
            msaa_samples,
            depth_image,
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
        self.command_pool
            .clear_command_buffers(&self.command_buffers);
        self.swap_chain.cleanup_framebuffers(&self.device);
        self.render_pass.cleanup(&self.device);
        self.swap_chain.cleanup(&self.device);
    }

    // TODO Move to separate structure and use Drop insted of cleanup
    pub fn recreate_swap_chain(&mut self, size: PhysicalSize<u32>) {
        self.swap_chain = VkSwapChain::new(&self.device, &self.surface, &[size.width, size.height]);

        self.color_image = VkImage::create_color_image(&self.device, self.swap_chain.format.format, self.swap_chain.extent, self.msaa_samples);

        let depth_format = VkImage::find_depth_format(&self.physical_device);
        self.depth_image = VkImage::create_depth_image(
            &self.device,
            &self.command_pool,
            self.device.graphics_queue,
            depth_format,
            self.swap_chain.extent,
            self.msaa_samples,
        );

        self.render_pass = VkRenderPass::new(&self.device, &self.swap_chain, depth_format, self.msaa_samples);
        self.swap_chain
            .create_frame_buffers(&self.render_pass, &self.depth_image, &self.color_image);

        self.command_buffers = self
            .command_pool
            .create_command_buffers(self.swap_chain.image_count);
    }
}

impl Drop for VkContext {
    fn drop(&mut self) {
        log::debug!("Dropping Vulkan context");
        self.cleanup_swap_chain();
    }
}

