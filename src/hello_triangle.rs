use std::mem::align_of;

use crate::{
    app::App,
    render::Vertex,
    vulkan::{read_shader_from_file, VkContext, VkDevice, VkPipeline, VkSettings},
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
    vertex_buffer_memory: vk::DeviceMemory,
    vertex_buffer: vk::Buffer,
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

        let vertex_buffer_size = (VERTICES.len() * std::mem::size_of::<Vertex>()) as u64;
        let vertex_buffer = Self::create_vertex_buffer(&vk_context.device, vertex_buffer_size);
        let vertex_buffer_memory = Self::assign_buffer_memory(&vk_context, vertex_buffer);
        Self::map_buffer_memory(&vk_context.device, vertex_buffer_memory, vertex_buffer_size);        

        let app = HelloTriangleApp {
            vertex_buffer_memory,
            vertex_buffer,
            pipeline,
            vertex_shader_module,
            fragment_shader_module,
            vk_context,
        };
        app.record_commands();

        app
    }

    fn create_vertex_buffer(device: &VkDevice, size: u64) -> vk::Buffer {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        unsafe {
            device
                .handle
                .create_buffer(&buffer_info, None)
                .expect("Unable to create vertex buffer")
        }
    }

    fn assign_buffer_memory(context: &VkContext, buffer: vk::Buffer) -> vk::DeviceMemory {
        let instance = &context.instance;
        let physical_device = &context.physical_device;
        let device = &context.device;

        let mem_requirements = unsafe { device.handle.get_buffer_memory_requirements(buffer) };
        let mem_type_index = Self::find_memory_type(
            mem_requirements,
            physical_device.get_mem_properties(instance),
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(mem_requirements.size)
            .memory_type_index(mem_type_index)
            .build();
        let memory = unsafe {
            let vertex_buffer_memory = device.allocate_memory(&alloc_info, None).expect("Unable to allocate buffer memory");
            device.bind_buffer_memory(buffer, vertex_buffer_memory, 0).expect("Unable to bind image memory");
            vertex_buffer_memory
        };
        memory
    }

    fn find_memory_type(
        requirements: vk::MemoryRequirements,
        mem_properties: vk::PhysicalDeviceMemoryProperties,
        required_properties: vk::MemoryPropertyFlags,
    ) -> u32 {
        for i in 0..mem_properties.memory_type_count {
            if requirements.memory_type_bits & (1 << i) != 0
                && mem_properties.memory_types[i as usize]
                    .property_flags
                    .contains(required_properties)
            {
                return i;
            }
        }
        panic!("Failed to find suitable memory type.")
    }

    fn map_buffer_memory(device: &VkDevice, buffer_memory: vk::DeviceMemory, size: u64) {
        unsafe {
            let data = device.handle.map_memory(buffer_memory, 0, size, vk::MemoryMapFlags::empty()).expect("Unable to map memory");
            let mut align = ash::util::Align::new(data, align_of::<u8>() as _, size);
            align.copy_from_slice(&VERTICES);
            device.unmap_memory(buffer_memory);
        }
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

    // TODO: Called from main
    pub fn record_commands(&self) {
        let context = &self.vk_context;
        let device = &context.device.handle;

        for (index, &buffer) in context.command_pool.buffers.iter().enumerate() {
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

                let buffers = [self.vertex_buffer];
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
        let command_buffers = [context.command_pool.buffers[image_index as usize]];
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
        unsafe {
            context
                .device
                .handle
                .destroy_buffer(self.vertex_buffer, None);
        }
        unsafe {
            context.device.handle.free_memory(self.vertex_buffer_memory, None);
        }
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
