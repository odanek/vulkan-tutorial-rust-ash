use crate::{
    app::App,
    render::Vertex,
    vulkan::{
        read_shader_from_file, VkBuffer, VkCommandPool, VkContext, VkDevice, VkPhysicalDevice,
        VkPipeline, VkSettings,
    },
};
use ash::{version::DeviceV1_0, vk};
use winit::{dpi::PhysicalSize, window::Window};

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [0.0, -0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub struct HelloTriangleApp {
    vertex_buffer: VkBuffer,
    pipeline: VkPipeline,
    vertex_shader_module: vk::ShaderModule,
    fragment_shader_module: vk::ShaderModule,
    vk_context: VkContext,
}

impl HelloTriangleApp {
    pub fn new(window: &Window) -> HelloTriangleApp {
        let vk_settings = VkSettings { validation: true };
        let vk_context = VkContext::new(&window, &vk_settings);

        let vertex_shader_module = read_shader_from_file("shader/vert.spv", &vk_context.device);
        let fragment_shader_module = read_shader_from_file("shader/frag.spv", &vk_context.device);
        let pipeline = VkPipeline::new(
            &vk_context.device,
            &vk_context.swap_chain,
            &vk_context.render_pass,
            vertex_shader_module,
            fragment_shader_module,
        );
        let vertex_buffer = Self::create_vertex_buffer(
            &vk_context.instance,
            &vk_context.physical_device,
            &vk_context.device,
            &vk_context.command_pool,
            vk_context.device.graphics_queue
        );

        let app = HelloTriangleApp {
            vertex_buffer,
            pipeline,
            vertex_shader_module,
            fragment_shader_module,
            vk_context,
        };
        app.record_commands();

        app
    }

    pub fn recreate_swap_chain(&mut self, size: PhysicalSize<u32>) {
        let context = &mut self.vk_context;
        context.device.wait_idle();
        self.pipeline.cleanup(&context.device);
        context.cleanup_swap_chain();
        context.recreate_swap_chain(size);
        self.pipeline = VkPipeline::new(
            &context.device,
            &context.swap_chain,
            &context.render_pass,
            self.vertex_shader_module,
            self.fragment_shader_module,
        );
        self.record_commands();
    }

    fn create_vertex_buffer(
        instance: &ash::Instance,
        physical_device: &VkPhysicalDevice,
        device: &VkDevice,
        command_pool: &VkCommandPool,
        queue: vk::Queue,
    ) -> VkBuffer {
        let size = (VERTICES.len() * std::mem::size_of::<Vertex>()) as u64;
        log::info!("creating vertex buffer of size {}", size);

        let staging_buffer = VkBuffer::new(
            instance,
            physical_device,
            device,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            size,
        );
        staging_buffer.map_memory(device, &VERTICES);

        let vertex_buffer = VkBuffer::new(
            instance,
            physical_device,
            device,
            vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            size,
        );

        log::info!("Copying vertex buffer data");
        VkBuffer::copy(device, command_pool, queue, &staging_buffer, &vertex_buffer);
        staging_buffer.cleanup(device);

        vertex_buffer
    }

    fn record_commands(&self) {
        let context = &self.vk_context;
        let device = &context.device.handle;

        for (index, &buffer) in context.command_buffers.iter().enumerate() {
            let command_begin_info = vk::CommandBufferBeginInfo::builder();
            unsafe {
                device
                    .begin_command_buffer(buffer, &command_begin_info)
                    .expect("Unable to begin command buffer")
            };

            let clear_values = [vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }];

            let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
                .render_pass(context.render_pass.handle)
                .framebuffer(context.swap_chain.framebuffers[index])
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: context.swap_chain.swap_extent,
                })
                .clear_values(&clear_values);

            unsafe {
                device.cmd_begin_render_pass(
                    buffer,
                    &render_pass_begin_info,
                    vk::SubpassContents::INLINE,
                );

                device.cmd_bind_pipeline(
                    buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.pipeline.handle,
                );

                let buffers = [self.vertex_buffer.handle];
                let offsets = [0 as vk::DeviceSize];
                device.cmd_bind_vertex_buffers(buffer, 0, &buffers, &offsets);

                device.cmd_draw(buffer, VERTICES.len() as u32, 1, 0, 0);

                device.cmd_end_render_pass(buffer);

                device
                    .end_command_buffer(buffer)
                    .expect("Failed to record end of command buffer");
            };
        }
    }
}

impl App for HelloTriangleApp {
    fn wait_idle(&self) {
        self.vk_context.device.wait_idle();
    }

    fn update(&mut self) {}

    fn resized(&mut self, _window: &Window, size: PhysicalSize<u32>) {
        self.recreate_swap_chain(size);
    }

    fn minimized(&mut self, _window: &Window) {}

    fn draw_frame(&mut self, window: &Window) {
        log::info!("Drawing");

        let context = &mut self.vk_context;
        let sync = &mut context.swap_chain_sync;
        let current_frame = sync.current_frame;
        let device = &context.device.handle;
        let fence = sync.in_flight_fences[current_frame];

        unsafe {
            let fences = [fence.handle];
            device
                .wait_for_fences(&fences, true, std::u64::MAX)
                .expect("Waiting for fence failed");
        }

        let acquire_result = context
            .swap_chain
            .acquire_next_image(&sync.image_available_semaphore[current_frame]);
        let image_index = match acquire_result {
            Ok((index, _)) => index as usize,
            Err(_) => {
                self.recreate_swap_chain(window.inner_size());
                return;
            }
        };

        if let Some(fence) = sync.images_in_flight[image_index] {
            unsafe {
                let fences = [fence.handle];
                device
                    .wait_for_fences(&fences, true, std::u64::MAX)
                    .expect("Waiting for fence failed");
            }
        }

        sync.images_in_flight[image_index as usize] = Some(fence);

        let wait_semaphores = [sync.image_available_semaphore[current_frame].handle];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [context.command_buffers[image_index as usize]];
        let signal_semaphores = [sync.render_finished_semaphore[current_frame].handle];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);
        let infos = [submit_info.build()];

        unsafe {
            let fences = [fence.handle];
            device.reset_fences(&fences).expect("Fence reset failed");
            device
                .queue_submit(context.device.graphics_queue, &infos, fence.handle)
                .expect("Unable to submit queue")
        };

        let swapchains = [context.swap_chain.handle];
        let images_indices = [image_index as u32];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&images_indices)
            .build();

        let result = unsafe {
            context
                .swap_chain
                .extension
                .queue_present(context.device.presentation_queue, &present_info)
        };

        sync.current_frame = (sync.current_frame + 1) % sync.max_frames_in_flight;

        match result {
            Ok(true) | Err(_) => {
                self.recreate_swap_chain(window.inner_size());
            }
            Ok(false) => (),
        }
    }
}

impl Drop for HelloTriangleApp {
    fn drop(&mut self) {
        let context = &self.vk_context;
        self.vertex_buffer.cleanup(&context.device);
        self.pipeline.cleanup(&context.device);
        unsafe {
            context
                .device
                .handle
                .destroy_shader_module(self.vertex_shader_module, None);
            context
                .device
                .handle
                .destroy_shader_module(self.fragment_shader_module, None);
        }
    }
}
