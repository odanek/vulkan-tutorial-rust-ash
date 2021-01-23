use ash::{extensions::khr::Swapchain, prelude::VkResult, version::DeviceV1_0, vk::{self, SwapchainKHR}};

use super::{
    device::VkDevice, physical_device::VkPhysicalDevice, render_pass::VkRenderPass,
    semaphore::VkSemaphore, surface::VkSurface,
};

pub struct VkSwapChain {
    pub format: vk::SurfaceFormatKHR,
    pub present_mode: vk::PresentModeKHR,
    pub swap_extent: vk::Extent2D,
    pub image_count: u32,
    pub extension: Swapchain,
    pub handle: SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub framebuffers: Vec<vk::Framebuffer>,
}

impl VkSwapChain {
    pub fn new(
        instance: &ash::Instance,
        physical_device: &VkPhysicalDevice,
        device: &VkDevice,
        surface: &VkSurface,
        dimensions: &[u32; 2],
    ) -> VkSwapChain {
        let surface_caps = surface.get_physical_device_surface_capabilities(physical_device);
        let format = choose_swapchain_surface_format(&surface_caps.formats);
        log::info!("Choosing swap-chain image format: {:?}", format);
        let present_mode = choose_swapchain_surface_present_mode(&surface_caps.present_modes);
        log::info!("Choosing swap-chain presentation mode: {:?}", present_mode);
        let swap_extent = choose_swapchain_extent(surface_caps.capabilities, dimensions);
        log::info!("Choosing swap-chain swap extent: {:?} for window size: {:?}", swap_extent, dimensions);
        let image_count = choose_image_count(&surface_caps.capabilities);
        log::info!("Choosing swap-chain image count: {}", image_count);

        let mut create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface.handle)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(swap_extent)
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

        let extension = Swapchain::new(instance, &device.handle);
        let handle = unsafe {
            extension
                .create_swapchain(&create_info, None)
                .expect("Unable to create swap chain")
        };

        log::info!("Creating images and image views");
        let images = unsafe {
            extension
                .get_swapchain_images(handle)
                .expect("Unable to get swap chain images")
        };
        let image_views = create_image_views(device, &images, format.format);

        VkSwapChain {
            format,
            present_mode,
            swap_extent,
            image_count,
            extension,
            handle,
            images,
            image_views,
            framebuffers: Vec::new(),
        }
    }

    pub fn create_frame_buffers(&mut self, device: &VkDevice, render_pass: &VkRenderPass) {
        log::info!("Creating framebuffers");

        self.framebuffers = self
            .image_views
            .iter()
            .map(|view| [*view])
            .map(|attachments| {
                let framebuffer_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(render_pass.handle)
                    .attachments(&attachments)
                    .width(self.swap_extent.width)
                    .height(self.swap_extent.height)
                    .layers(1)
                    .build();
                unsafe {
                    device
                        .handle
                        .create_framebuffer(&framebuffer_info, None)
                        .expect("Unable to create framebuffer")
                }
            })
            .collect::<Vec<_>>();
    }

    pub fn acquire_next_image(&self, semaphore: &VkSemaphore) -> VkResult<(u32, bool)> {
        unsafe {
            self.extension
                .acquire_next_image(
                    self.handle,
                    std::u64::MAX,
                    semaphore.handle,
                    vk::Fence::null(),
                )             
        }
    }

    pub fn cleanup(&self, device: &VkDevice) {
        log::debug!("Dropping swap chain image views");
        for &view in self.image_views.iter() {
            unsafe { device.handle.destroy_image_view(view, None) };
        }
        log::debug!("Dropping swap chain");
        unsafe {
            self.extension.destroy_swapchain(self.handle, None);
        }
    }

    pub fn cleanup_framebuffers(&mut self, device: &VkDevice) {
        log::debug!("Dropping framebuffers");
        for &buffer in self.framebuffers.iter() {
            unsafe { device.handle.destroy_framebuffer(buffer, None) };
        }
        self.framebuffers.clear();
    }
}

fn choose_swapchain_surface_format(
    available_formats: &[vk::SurfaceFormatKHR],
) -> vk::SurfaceFormatKHR {
    *available_formats
        .iter()
        .find(|format| {
            format.format == vk::Format::B8G8R8A8_UNORM
                && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or(&available_formats[0])
}

fn choose_swapchain_surface_present_mode(
    available_present_modes: &[vk::PresentModeKHR],
) -> vk::PresentModeKHR {
    if available_present_modes.contains(&vk::PresentModeKHR::MAILBOX) {
        vk::PresentModeKHR::MAILBOX
    } else if available_present_modes.contains(&vk::PresentModeKHR::FIFO) {
        vk::PresentModeKHR::FIFO
    } else {
        vk::PresentModeKHR::IMMEDIATE
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

fn choose_image_count(capabilities: &vk::SurfaceCapabilitiesKHR) -> u32 {
    let max = capabilities.max_image_count;
    let mut preferred = capabilities.min_image_count + 1;
    if max > 0 && preferred > max {
        preferred = max;
    }
    preferred
}

fn create_image_views(
    device: &VkDevice,
    images: &[vk::Image],
    format: vk::Format,
) -> Vec<vk::ImageView> {
    images
        .iter()
        .map(|&image| create_image_view(device, image, format))
        .collect::<Vec<_>>()
}

fn create_image_view(device: &VkDevice, image: vk::Image, format: vk::Format) -> vk::ImageView {
    let create_info = vk::ImageViewCreateInfo::builder()
        .image(image)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(format)
        .components(vk::ComponentMapping {
            r: vk::ComponentSwizzle::IDENTITY,
            g: vk::ComponentSwizzle::IDENTITY,
            b: vk::ComponentSwizzle::IDENTITY,
            a: vk::ComponentSwizzle::IDENTITY,
        })
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        });
    unsafe {
        device
            .handle
            .create_image_view(&create_info, None)
            .expect("Unable to create image view")
    }
}
