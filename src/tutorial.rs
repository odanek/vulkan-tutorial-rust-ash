use std::time::Instant;

use crate::{app::App, render::{Mat4, Vec3, Vertex}, vulkan::{VkBuffer, VkContext, VkDescriptorPool, VkDescriptorSetLayout, VkDevice, VkFence, VkImage, VkPipeline, VkSettings, VkShaderModule, VkSwapChain, VkSwapChainSync}};
use ash::{version::DeviceV1_0, vk};
use winit::{dpi::PhysicalSize, window::Window};

const VERTICES: [Vertex; 4] = [
    Vertex {
        position: Vec3::new(-0.5, 0.5, 0.0),
        color: Vec3::new(1.0, 0.0, 0.0),
    },
    Vertex {
        position: Vec3::new(0.5, 0.5, 0.0),
        color: Vec3::new(0.0, 1.0, 0.0),
    },
    Vertex {
        position: Vec3::new(0.5, -0.5, 0.0),
        color: Vec3::new(0.0, 0.0, 1.0),
    },
    Vertex {
        position: Vec3::new(-0.5, -0.5, 0.0),
        color: Vec3::new(1.0, 0.0, 1.0),
    },
];

const INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

#[repr(C)]
#[derive(Clone, Copy)]
struct UniformBufferObject {
    model: Mat4,
    view: Mat4,
    proj: Mat4,
}

pub struct TutorialAppSwapChainContext {
    uniform_buffers: Vec<VkBuffer>,
    pipeline: VkPipeline,
    descriptor_sets: Vec<vk::DescriptorSet>,
    descriptor_pool: VkDescriptorPool,
}

pub struct TutorialApp {
    start_time: Instant,
    swap_chain_context: Option<TutorialAppSwapChainContext>,
    // texture_image: VkImage,
    index_buffer: VkBuffer,
    vertex_buffer: VkBuffer,
    descriptor_set_layout: VkDescriptorSetLayout,
    vertex_shader_module: VkShaderModule,
    fragment_shader_module: VkShaderModule,
    swap_chain_sync: VkSwapChainSync,
    vk_context: VkContext,
}

impl TutorialApp {
    pub fn new(window: &Window) -> TutorialApp {
        let vk_settings = VkSettings { validation: true };
        let vk_context = VkContext::new(&window, &vk_settings);
        let swap_chain_sync = VkSwapChainSync::new(&vk_context.device, &vk_context.swap_chain, 2);
        let vertex_shader_module = VkShaderModule::new_from_file(
            &vk_context.device,
            vk::ShaderStageFlags::VERTEX,
            "shader/vert.spv",
            "main",
        );
        let fragment_shader_module = VkShaderModule::new_from_file(
            &vk_context.device,
            vk::ShaderStageFlags::FRAGMENT,
            "shader/frag.spv",
            "main",
        );
        let descriptor_set_layout = Self::create_descriptor_set_layout(&vk_context);
        let vertex_buffer = Self::create_vertex_buffer(&vk_context);
        let index_buffer = Self::create_index_buffer(&vk_context);
        // let texture_image = Self::create_texture_image(&vk_context);

        let mut app = TutorialApp {
            start_time: Instant::now(),
            swap_chain_context: None,
            // texture_image,
            index_buffer,
            vertex_buffer,
            descriptor_set_layout,
            vertex_shader_module,
            fragment_shader_module,
            swap_chain_sync,
            vk_context,
        };
        app.create_swap_chain();        

        app
    }

    fn create_swap_chain(&mut self) {
        let context = &self.vk_context;
        let pipeline = Self::create_pipeline(
            context,
            &self.vertex_shader_module,
            &self.fragment_shader_module,
            &self.descriptor_set_layout,
        );
        let uniform_buffers = Self::create_uniform_buffers(context);
        let descriptor_pool = Self::create_descriptor_pool(context);
        let descriptor_sets = Self::create_descriptor_sets(
            &context.device,
            &descriptor_pool,
            &self.descriptor_set_layout,
            &uniform_buffers,
        );

        self.swap_chain_context = Some(TutorialAppSwapChainContext {
            pipeline,
            uniform_buffers,
            descriptor_sets,
            descriptor_pool
        });

        self.record_commands();
    }

    fn recreate_swap_chain(&mut self, size: PhysicalSize<u32>) {
        self.vk_context.device.wait_idle();
        self.swap_chain_context = None;
        self.vk_context.cleanup_swap_chain();
        self.vk_context.recreate_swap_chain(size);
        self.create_swap_chain();
    }

    fn create_descriptor_set_layout(context: &VkContext) -> VkDescriptorSetLayout {
        let ubo_layout_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::VERTEX);
        VkDescriptorSetLayout::new(&context.device, &[ubo_layout_binding.build()])
    }

    fn create_pipeline(
        context: &VkContext,
        vs: &VkShaderModule,
        fs: &VkShaderModule,
        descriptor_set_layout: &VkDescriptorSetLayout,
    ) -> VkPipeline {
        VkPipeline::new(
            &context.device,
            &context.swap_chain,
            &context.render_pass,
            &vs,
            &fs,
            &[descriptor_set_layout.handle],
        )
    }

    fn create_vertex_buffer(context: &VkContext) -> VkBuffer {
        VkBuffer::new_device_local(
            &context.device,
            &context.command_pool,
            context.device.graphics_queue,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            &VERTICES,
        )
    }

    fn create_index_buffer(context: &VkContext) -> VkBuffer {
        VkBuffer::new_device_local(
            &context.device,
            &context.command_pool,
            context.device.graphics_queue,
            vk::BufferUsageFlags::INDEX_BUFFER,
            &INDICES,
        )
    }

    fn create_uniform_buffers(context: &VkContext) -> Vec<VkBuffer> {
        let size = std::mem::size_of::<UniformBufferObject>() as u64;
        let count = context.swap_chain.image_count;
        log::info!("Creating {} uniform buffers", count);

        (0..count)
            .map(|_| {
                VkBuffer::new(
                    &context.device,
                    vk::BufferUsageFlags::UNIFORM_BUFFER,
                    vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                    size,
                )
            })
            .collect::<Vec<_>>()
    }

    fn update_uniform_buffer(&self, image_index: usize, elapsed_time: f32) {
        let swap_context = match &self.swap_chain_context {
            Some(ref context) => context,
            None => return
        };

        let extent = self.vk_context.swap_chain.swap_extent;
        let screen_width = extent.width as f32;
        let screen_height = extent.height as f32;
        let ubo = UniformBufferObject {
            model: Mat4::rotate_z(elapsed_time),
            view: Mat4::look_at(
                &Vec3::new(0.0, 0.0, 5.0),
                &Vec3::new(0.0, 0.0, 0.0),
                &Vec3::new(0.0, 1.0, 0.0),
            ),
            proj: Mat4::perspective(0.785, screen_width / screen_height, 0.1, 10.0),
        };

        swap_context.uniform_buffers[image_index].map_memory(&self.vk_context.device, &[ubo]);
    }

    fn create_descriptor_pool(context: &VkContext) -> VkDescriptorPool {
        VkDescriptorPool::new(
            &context.device,
            vk::DescriptorType::UNIFORM_BUFFER,
            context.swap_chain.image_count,
        )
    }

    fn create_descriptor_sets(
        device: &VkDevice,
        pool: &VkDescriptorPool,
        layout: &VkDescriptorSetLayout,
        uniform_buffers: &[VkBuffer],
    ) -> Vec<vk::DescriptorSet> {
        let count = uniform_buffers.len();
        log::info!("Creating {} descriptor sets", count);

        let layouts = (0..count).map(|_| layout.handle).collect::<Vec<_>>();
        let alloc_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(pool.handle)
            .set_layouts(&layouts)
            .build();
        let descriptor_sets = unsafe {
            device
                .handle
                .allocate_descriptor_sets(&alloc_info)
                .expect("Unable to create descriptor sets")
        };

        descriptor_sets
            .iter()
            .zip(uniform_buffers.iter())
            .for_each(|(set, buffer)| {
                let buffer_info = vk::DescriptorBufferInfo::builder()
                    .buffer(buffer.handle)
                    .offset(0)
                    .range(std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize)
                    .build();
                let buffer_infos = [buffer_info];

                let ubo_descriptor_write = vk::WriteDescriptorSet::builder()
                    .dst_set(*set)
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .buffer_info(&buffer_infos)
                    .build();

                let descriptor_writes = [ubo_descriptor_write];

                unsafe {
                    device
                        .handle
                        .update_descriptor_sets(&descriptor_writes, &[])
                }
            });

        descriptor_sets
    }

    fn create_texture_image(context: &VkContext) -> VkImage {
        todo!()
    }

    fn record_commands(&self) {
        let context = &self.vk_context;
        let device = &context.device.handle;        
        let swap_context = match &self.swap_chain_context {
            Some(ref context) => context,
            None => return
        };

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
                    swap_context.pipeline.handle,
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
                device.cmd_bind_descriptor_sets(
                    buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    swap_context.pipeline.layout,
                    0,
                    &swap_context.descriptor_sets[index..=index],
                    &[],
                );
                device.cmd_draw_indexed(buffer, INDICES.len() as u32, 1, 0, 0, 0);
                device.cmd_end_render_pass(buffer);
                device
                    .end_command_buffer(buffer)
                    .expect("Failed to record end of command buffer");
            };
        }
    }

    fn acquire_image(
        &mut self,
        window: &Window,
    ) -> Option<usize> {
        let context = &self.vk_context;
        let sync = &mut self.swap_chain_sync;
        let device = &context.device.handle;

        let current_frame = sync.current_frame;
        let fence = &sync.in_flight_fences[current_frame];

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
                return None;
            }
        };

        if let Some(ref fence) = sync.images_in_flight[image_index] {
            unsafe {
                let fences = [fence.handle];
                device
                    .wait_for_fences(&fences, true, std::u64::MAX)
                    .expect("Waiting for fence failed");
            }
        }

        sync.images_in_flight[image_index] = Some(fence.clone());

        Some(image_index)
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

        let image_index = match self.acquire_image(window) {
            Some(index) => index,
            None => return,
        };

        let current_frame = self.swap_chain_sync.current_frame;
        let fence = &self.swap_chain_sync.in_flight_fences[current_frame];
        let elapsed_time = self.start_time.elapsed().as_secs_f32();
        self.update_uniform_buffer(image_index, elapsed_time);

        let context = &self.vk_context;        
        let device = &context.device.handle;

        let wait_semaphores = [self.swap_chain_sync.image_available_semaphore[current_frame].handle];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [context.command_buffers[image_index as usize]];
        let signal_semaphores = [self.swap_chain_sync.render_finished_semaphore[current_frame].handle];
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

        self.swap_chain_sync.advance_frame();

        match result {
            Ok(true) | Err(_) => {
                self.recreate_swap_chain(window.inner_size());
            }
            Ok(false) => (),
        }
    }
}