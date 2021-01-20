use crate::{
    app::App,
    vulkan::{VkContext, VkSettings},
};
use ash::{version::DeviceV1_0, vk};
use winit::window::Window;

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

    fn update(&mut self) {}

    fn draw_frame(&mut self) {
        log::info!("Drawing");

        let context = &mut self.vk_context;
        let current_frame = context.current_frame;
        let device = &context.device.handle;

        let image_index = context
            .swap_chain
            .acquire_next_image(&context.image_available_semaphore[current_frame]);

        let wait_semaphores = [context.image_available_semaphore[current_frame].handle];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [context.command_pool.buffers[image_index as usize]];
        let signal_semaphores = [context.render_finished_semaphore[current_frame].handle];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);
        let infos = [submit_info.build()];

        unsafe {
            device
                .queue_submit(context.device.graphics_queue, &infos, vk::Fence::null())
                .expect("Unable to submit queue")
        };

        let swapchains = [context.swap_chain.handle];
        let images_indices = [image_index];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&images_indices)
            .build();

        let _result = unsafe {
            context.swap_chain.extension.queue_present(context.device.presentation_queue, &present_info)
        };

        context.current_frame = (context.current_frame + 1) % context.max_frames_in_flight;
    }
}
