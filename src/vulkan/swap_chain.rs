use ash::{extensions::khr::Swapchain, vk::{self, SwapchainKHR}};

use super::{device::VkDevice, physical_device::VkPhysicalDevice, surface::VkSurface};

pub struct VkSwapChain {
    pub format: vk::SurfaceFormatKHR,
    pub present_mode: vk::PresentModeKHR,
    pub swap_extent: vk::Extent2D,
    pub image_count: u32,
    pub extension: Swapchain,
    pub handle: SwapchainKHR,
    pub images: Vec<vk::Image>,
}

impl VkSwapChain {
    pub fn new(
        instance: &ash::Instance,
        physical_device: &VkPhysicalDevice,
        device: &VkDevice,
        surface: &VkSurface,
        dimensions: &[u32; 2],
    ) -> VkSwapChain {
        let surface_caps = &physical_device.surface_caps;
        let format = choose_swapchain_surface_format(&surface_caps.formats);
        log::info!("Choosing swap-chain image format: {:?}", format);
        let present_mode = choose_swapchain_surface_present_mode(&surface_caps.present_modes);
        log::info!("Choosing swap-chain presentation mode: {:?}", present_mode);
        let swap_extent = choose_swapchain_extent(surface_caps.capabilities, dimensions);
        log::info!("Choosing swap-chain swap extent: {:?}", swap_extent);
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

        let queue_family_indices = [device.graphics_queue_family, device.presentation_queue_family];
        if device.graphics_queue_family != device.presentation_queue_family {
            create_info = create_info.image_sharing_mode(vk::SharingMode::CONCURRENT).queue_family_indices(&queue_family_indices);
        } else {
            create_info = create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE);
        }

        let extension = Swapchain::new(instance, &device.handle);
        let handle = unsafe { extension.create_swapchain(&create_info, None).unwrap() };
        let images = unsafe { extension.get_swapchain_images(handle).unwrap() };

        VkSwapChain {
            format,
            present_mode,
            swap_extent,
            image_count,
            extension,
            handle,
            images
        }
    }
}

impl Drop for VkSwapChain {
    fn drop(&mut self) {
        log::debug!("Dropping swap chain");
        unsafe {
            self.extension.destroy_swapchain(self.handle, None);
        }
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
