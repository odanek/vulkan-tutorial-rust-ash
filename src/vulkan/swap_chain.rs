use std::sync::Arc;

use ash::{
    extensions::khr::Swapchain,
    prelude::VkResult,
    version::DeviceV1_0,
    vk,
};

use super::{
    device::VkDevice, render_pass::VkRenderPass, semaphore::VkSemaphore, surface::VkSurface,
    VkCommandPool, VkFence, VkImage, VkTexture,
};

pub struct VkSwapChainImage {
    device: Arc<VkDevice>,
    pub image: vk::Image,
    pub view: vk::ImageView,
    pub color_image: VkTexture,
    pub depth_image: VkTexture,
    pub framebuffer: vk::Framebuffer,
    pub frame: Option<usize>,
    pub command_buffer: vk::CommandBuffer,
}

pub struct VkFrame {
    pub available: VkSemaphore,
    pub finished: VkSemaphore,
    pub in_flight: VkFence,
}

pub struct VkSwapChain {
    device: Arc<VkDevice>,
    pub handle: vk::SwapchainKHR,
    pub extension: Swapchain,
    pub format: vk::SurfaceFormatKHR,
    pub present_mode: vk::PresentModeKHR,
    pub extent: vk::Extent2D,
    pub images: Vec<VkSwapChainImage>,
    pub frames: Vec<VkFrame>,
}

impl VkSwapChain {
    pub fn new(
        device: &Arc<VkDevice>,
        surface: &VkSurface,
        format: vk::SurfaceFormatKHR,
        present_mode: vk::PresentModeKHR,
        image_count: u32,
        dimensions: &[u32; 2],
    ) -> VkSwapChain {
        let surface_caps =
            surface.get_physical_device_surface_capabilities(&device.physical_device);

        let extent = choose_swapchain_extent(surface_caps.capabilities, dimensions);
        log::info!(
            "Choosing swap-chain swap extent: {:?} for window size: {:?}",
            extent,
            dimensions
        );

        // TODO: This changes with resolution
        let mut create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface.handle)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(surface_caps.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        let queue_family_indices = [
            device.graphics_queue_family,
            device.presentation_queue_family,
        ];
        if device.graphics_queue_family != device.presentation_queue_family {
            create_info = create_info
                .image_sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(&queue_family_indices);
        } else {
            create_info = create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE);
        }

        let extension = Swapchain::new(&device.physical_device.instance.handle, &device.handle);
        let handle = unsafe {
            extension
                .create_swapchain(&create_info, None)
                .expect("Unable to create swap chain")
        };

        VkSwapChain {
            device: Arc::clone(device),
            format,
            present_mode,
            extent,
            extension,
            handle,
            images: Vec::new(),
            frames: Vec::new(),
        }
    }

    pub fn image_count(&self) -> usize {
        return self.images.len();
    }

    pub fn frame_count(&self) -> usize {
        return self.frames.len();
    }

    pub fn advance_frame(&self, frame: usize) -> usize {
        (frame + 1) % self.frame_count()
    }

    pub fn initialize_images(
        &mut self,
        max_frames: usize,
        render_pass: &VkRenderPass,
        depth_format: vk::Format,
        msaa_samples: vk::SampleCountFlags,
        command_pool: &VkCommandPool,
        transfer_queue: vk::Queue,
    ) {
        log::info!("Creating swap-chain images");
        let images = unsafe {
            self.extension
                .get_swapchain_images(self.handle)
                .expect("Unable to get swap chain images")
        };

        for &image in images.iter() {
            let color_format = self.format.format;

            let view = VkImage::create_image_view(
                &self.device,
                image,
                1,
                color_format,
                vk::ImageAspectFlags::COLOR,
            );

            let color_image =
                VkImage::create_color_image(&self.device, color_format, self.extent, msaa_samples);

            let depth_image = VkImage::create_depth_image(
                &self.device,
                &command_pool,
                transfer_queue,
                depth_format,
                self.extent,
                msaa_samples,
            );

            let framebuffer =
                self.create_frame_buffer(view, render_pass, &depth_image, &color_image);
            let command_buffer = command_pool.create_command_buffers(1)[0]; // TODO: Release in Drop

            let swap_image = VkSwapChainImage {
                device: Arc::clone(&self.device),
                image,
                view,
                color_image,
                depth_image,
                framebuffer,
                frame: None,
                command_buffer,
            };

            self.images.push(swap_image);
        }

        let frame_count = max_frames.min(images.len());
        for index in 0..frame_count {
            let available = VkSemaphore::new(&self.device);
            let finished = VkSemaphore::new(&self.device);
            let in_flight = VkFence::new(&self.device);
    
            let frame = VkFrame {
                available,
                finished,
                in_flight
            };

            self.frames.push(frame);
        }
    }

    pub fn create_frame_buffer(
        &self,
        view: vk::ImageView,
        render_pass: &VkRenderPass,
        depth_image: &VkTexture,
        color_image: &VkTexture,
    ) -> vk::Framebuffer {
        let attachments = [color_image.view, depth_image.view, view];
        let framebuffer_info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass.handle)
            .attachments(&attachments)
            .width(self.extent.width)
            .height(self.extent.height)
            .layers(1)
            .build();
        unsafe {
            self.device
                .handle
                .create_framebuffer(&framebuffer_info, None)
                .expect("Unable to create framebuffer")
        }
    }

    pub fn acquire_next_image(&self, semaphore: &VkSemaphore) -> VkResult<(u32, bool)> {
        unsafe {
            self.extension.acquire_next_image(
                self.handle,
                std::u64::MAX,
                semaphore.handle,
                vk::Fence::null(),
            )
        }
    }

    pub fn cleanup_images(&mut self) {
        log::debug!("Dropping swap chain images");
        self.images.clear();
    }

    // TODO: Resize method
}

impl Drop for VkSwapChainImage {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_image_view(self.view, None) };
        unsafe {
            self.device
                .handle
                .destroy_framebuffer(self.framebuffer, None)
        };
    }
}

impl Drop for VkSwapChain {
    fn drop(&mut self) {
        log::debug!("Dropping swap chain");
        self.images.clear();
        self.frames.clear();
        unsafe {
            self.extension.destroy_swapchain(self.handle, None);
        }
    }
}

fn choose_swapchain_extent(
    capabilities: vk::SurfaceCapabilitiesKHR,
    dimensions: &[u32; 2],
) -> vk::Extent2D {
    if capabilities.current_extent.width != std::u32::MAX {
        return capabilities.current_extent;
    }

    let min = capabilities.min_image_extent;
    let max = capabilities.max_image_extent;
    let width = dimensions[0].min(max.width).max(min.width);
    let height = dimensions[1].min(max.height).max(min.height);
    vk::Extent2D { width, height }
}
