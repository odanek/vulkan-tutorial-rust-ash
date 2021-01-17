use crate::{app::App, vulkan::{VkContext, VkSettings}};
use winit::{
    window::Window,
};

pub struct HelloTriangleApp {
    vk_context: VkContext,
}

impl HelloTriangleApp {
    pub fn new(window: &Window) -> HelloTriangleApp {
        let vk_settings = VkSettings { validation: true };
        let vk_context = VkContext::new(&window, &vk_settings);
        vk_context.record_commands();

        HelloTriangleApp { vk_context }
    }
}

impl App for HelloTriangleApp {
    fn wait_idle(&self) {
        self.vk_context.device.wait_idle();
    }

    fn update(&mut self) {
    }

    fn draw_frame(&mut self) {
        log::info!("Drawing");

        let context = &self.vk_context;
        context.swap_chain.acquire_next_image(&context.image_available_semaphore);
    }
}