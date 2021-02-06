use crate::{
    app::App,
    render::{Mat4, Vec3, Vertex},
    vulkan::{
        VkBuffer, VkCommandPool, VkContext, VkDevice, VkPhysicalDevice, VkPipeline, VkSettings,
        VkShaderModule,
    },
};
use ash::{version::DeviceV1_0, vk};
use winit::{dpi::PhysicalSize, window::Window};

const VERTICES: [Vertex; 4] = [
    Vertex {
        position: Vec3::new(-0.5, -0.5, 0.0),
        color: Vec3::new(1.0, 0.0, 0.0),
    },
    Vertex {
        position: Vec3::new(0.5, -0.5, 0.0),
        color: Vec3::new(0.0, 1.0, 0.0),
    },
    Vertex {
        position: Vec3::new(0.5, 0.5, 0.0),
        color: Vec3::new(0.0, 0.0, 1.0),
    },
    Vertex {
        position: Vec3::new(-0.5, 0.5, 0.0),
        color: Vec3::new(1.0, 0.0, 1.0),
    },
];

const INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

struct UniformBufferObject {
    model: Mat4,
    view: Mat4,
    proj: Mat4,
}

pub struct TutorialApp {
    index_buffer: VkBuffer,
    vertex_buffer: VkBuffer,
    pipeline: VkPipeline,
    descriptor_set_layout: vk::DescriptorSetLayout,
    vertex_shader_module: VkShaderModule,
    fragment_shader_module: VkShaderModule,
    vk_context: VkContext,
}

impl TutorialApp {
    pub fn new(window: &Window) -> TutorialApp {
        let vk_settings = VkSettings { validation: true };
        let vk_context = VkContext::new(&window, &vk_settings);
        let VkContext {
            ref instance,
            ref physical_device,
            ref device,
            ref command_pool,
            ref swap_chain,
            ref render_pass,
            ..
        } = vk_context;

        let vertex_shader_module = VkShaderModule::new_from_file(
            device,
            vk::ShaderStageFlags::VERTEX,
            "shader/vert.spv",
            "main",
        );
        let fragment_shader_module = VkShaderModule::new_from_file(
            device,
            vk::ShaderStageFlags::FRAGMENT,
            "shader/frag.spv",
            "main",
        );
        let descriptor_set_layout = Self::create_descriptor_set_layout(device);
        let pipeline = VkPipeline::new(
            device,
            swap_chain,
            render_pass,
            &vertex_shader_module,
            &fragment_shader_module,
            &[descriptor_set_layout]
        );
        let vertex_buffer = Self::create_buffer(
            instance,
            physical_device,
            device,
            command_pool,
            device.graphics_queue,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            &VERTICES,
        );
        let index_buffer = Self::create_buffer(
            instance,
            physical_device,
            device,
            command_pool,
            device.graphics_queue,
            vk::BufferUsageFlags::INDEX_BUFFER,
            &INDICES,
        );

        let app = TutorialApp {
            index_buffer,
            vertex_buffer,
            pipeline,
            descriptor_set_layout,
            vertex_shader_module,
            fragment_shader_module,
            vk_context,
        };
        app.record_commands();

        app
    }

    fn recreate_swap_chain(&mut self, size: PhysicalSize<u32>) {
        let context = &mut self.vk_context;
        context.device.wait_idle();
        self.pipeline.cleanup(&context.device);
        context.cleanup_swap_chain();
        context.recreate_swap_chain(size);
        self.pipeline = VkPipeline::new(
            &context.device,
            &context.swap_chain,
            &context.render_pass,
            &self.vertex_shader_module,
            &self.fragment_shader_module,
            &[self.descriptor_set_layout]
        );
        self.record_commands();
    }

    fn create_descriptor_set_layout(device: &VkDevice) -> vk::DescriptorSetLayout {
        let ubo_layout_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::VERTEX);
        let bindings = [ubo_layout_binding.build()];
        let layout_info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(&bindings);

        unsafe {
            device.handle.create_descriptor_set_layout(&layout_info, None).expect("Unable to create descriptor set layout")
        }
    }

    fn create_buffer<T: Copy>(
        instance: &ash::Instance,
        physical_device: &VkPhysicalDevice,
        device: &VkDevice,
        command_pool: &VkCommandPool,
        queue: vk::Queue,
        usage: vk::BufferUsageFlags,
        data: &[T],
    ) -> VkBuffer {
        let size = (data.len() * std::mem::size_of::<T>()) as u64;
        log::info!("creating vertex buffer of size {}", size);

        let staging_buffer = VkBuffer::new(
            instance,
            physical_device,
            device,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            size,
        );
        staging_buffer.map_memory(device, data);

        let vertex_buffer = VkBuffer::new(
            instance,
            physical_device,
            device,
            usage | vk::BufferUsageFlags::TRANSFER_DST,
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
                device.cmd_bind_index_buffer(
                    buffer,
                    self.index_buffer.handle,
                    0,
                    vk::IndexType::UINT16,
                );
                device.cmd_draw_indexed(buffer, INDICES.len() as u32, 1, 0, 0, 0);
                device.cmd_end_render_pass(buffer);
                device
                    .end_command_buffer(buffer)
                    .expect("Failed to record end of command buffer");
            };
        }
    }
}

impl App for TutorialApp {
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

impl Drop for TutorialApp {
    fn drop(&mut self) {
        let context = &self.vk_context;
        let device = &context.device;

        self.index_buffer.cleanup(&device);
        self.vertex_buffer.cleanup(&device);
        self.pipeline.cleanup(&device);
        
        unsafe {
            device.handle.destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        }

        self.vertex_shader_module.cleanup(device);
        self.fragment_shader_module.cleanup(device);
    }
}
