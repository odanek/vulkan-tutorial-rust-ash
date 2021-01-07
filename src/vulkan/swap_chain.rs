use ash::vk;

use super::physical_device::VkPhysicalDevice;

pub struct VkSwapChain {
    pub format: vk::SurfaceFormatKHR,
    pub present_mode: vk::PresentModeKHR,
    pub swap_extent: vk::Extent2D,
}

impl VkSwapChain {
    pub fn new(physical_device: &VkPhysicalDevice, dimensions: &[u32; 2]) -> VkSwapChain {
        let surface_caps = &physical_device.surface_caps;
        let format = choose_swapchain_surface_format(&surface_caps.formats);
        let present_mode = choose_swapchain_surface_present_mode(&surface_caps.present_modes);
        let swap_extent = choose_swapchain_extent(surface_caps.capabilities, dimensions);

        VkSwapChain {
            format,
            present_mode,
            swap_extent,
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
