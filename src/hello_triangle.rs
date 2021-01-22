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
        let fence = context.in_flight_fences[current_frame];

        unsafe {
            let fences = [fence.handle];
            device
                .wait_for_fences(&fences, true, std::u64::MAX)
                .expect("Waiting for fence failed");
            device.reset_fences(&fences).expect("Fence reset failed");
        }

        let image_index = context
            .swap_chain
            .acquire_next_image(&context.image_available_semaphore[current_frame])
            as usize;

        if let Some(fence) = context.images_in_flight[image_index] {
            unsafe {
                let fences = [fence.handle];
                device
                    .wait_for_fences(&fences, true, std::u64::MAX)
                    .expect("Waiting for fence failed");
            }
        }

        context.images_in_flight[image_index] = Some(fence);

        let wait_semaphores = [context.image_available_semaphore[current_frame].handle];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [context.command_pool.buffers[image_index]];
        let signal_semaphores = [context.render_finished_semaphore[current_frame].handle];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);
        let infos = [submit_info.build()];

        unsafe {
            device
                .queue_submit(
                    context.device.graphics_queue,
                    &infos,
                    context.in_flight_fences[current_frame].handle,
                )
                .expect("Unable to submit queue")
        };

        let swapchains = [context.swap_chain.handle];
        let images_indices = [image_index as u32];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&images_indices)
            .build();

        let _result = unsafe {
            context
                .swap_chain
                .extension
                .queue_present(context.device.presentation_queue, &present_info)
        };

        context.current_frame = (context.current_frame + 1) % context.max_frames_in_flight;
    }
}
