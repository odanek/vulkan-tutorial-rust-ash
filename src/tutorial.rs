use std::time::Instant;

use crate::{
    app::App,
    render::{Mat4, Vec2, Vec3, Vertex},
    vulkan::{
        VkBuffer, VkCommandPool, VkContext, VkDescriptorPool, VkDescriptorSetLayout, VkDevice,
        VkImage, VkPipeline, VkRenderPass, VkSampler, VkSettings, VkShaderModule, VkSurface,
        VkSwapChain, VkTexture,
    },
};
use ash::{version::DeviceV1_0, vk};
use winit::{dpi::PhysicalSize, window::Window};

const VERTICES: [Vertex; 8] = [
    Vertex {
        position: Vec3::new(-0.5, 0.5, 0.0),
        color: Vec3::new(1.0, 0.0, 0.0),
        tex_coord: Vec2::new(0.0, 1.0),
    },
    Vertex {
        position: Vec3::new(0.5, 0.5, 0.0),
        color: Vec3::new(0.0, 1.0, 0.0),
        tex_coord: Vec2::new(1.0, 1.0),
    },
    Vertex {
        position: Vec3::new(0.5, -0.5, 0.0),
        color: Vec3::new(0.0, 0.0, 1.0),
        tex_coord: Vec2::new(1.0, 0.0),
    },
    Vertex {
        position: Vec3::new(-0.5, -0.5, 0.0),
        color: Vec3::new(1.0, 0.0, 1.0),
        tex_coord: Vec2::new(0.0, 0.0),
    },
    Vertex {
        position: Vec3::new(-0.5, 0.5, -1.0),
        color: Vec3::new(1.0, 0.0, 0.0),
        tex_coord: Vec2::new(0.0, 1.0),
    },
    Vertex {
        position: Vec3::new(0.5, 0.5, -1.0),
        color: Vec3::new(0.0, 1.0, 0.0),
        tex_coord: Vec2::new(1.0, 1.0),
    },
    Vertex {
        position: Vec3::new(0.5, -0.5, -1.0),
        color: Vec3::new(0.0, 0.0, 1.0),
        tex_coord: Vec2::new(1.0, 0.0),
    },
    Vertex {
        position: Vec3::new(-0.5, -0.5, -1.0),
        color: Vec3::new(1.0, 0.0, 1.0),
        tex_coord: Vec2::new(0.0, 0.0),
    },
];

const INDICES: [u16; 12] = [0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4];

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
    current_frame: usize,
    swap_chain: VkSwapChain,
}

pub struct TutorialApp {
    start_time: Instant,
    swap_chain_context: Option<TutorialAppSwapChainContext>,
    sampler: VkSampler,
    texture_image: VkTexture,
    index_buffer: VkBuffer,
    vertex_buffer: VkBuffer,
    descriptor_pool: VkDescriptorPool,
    descriptor_set_layout: VkDescriptorSetLayout,
    vertex_shader_module: VkShaderModule,
    fragment_shader_module: VkShaderModule,
    render_pass: VkRenderPass,
    swap_chain_format: vk::SurfaceFormatKHR,
    swap_chain_present_mode: vk::PresentModeKHR,
    swap_image_count: u32,
    command_pool: VkCommandPool,
    depth_format: vk::Format,
    msaa_samples: vk::SampleCountFlags,
    window_size: PhysicalSize<u32>,
    vk_context: VkContext,
}

impl TutorialApp {
    pub fn new(window: &Window) -> TutorialApp {
        let vk_settings = VkSettings { validation: true };
        let vk_context = VkContext::new(&window, &vk_settings);
        let device = &vk_context.device;

        let msaa_samples = device.get_max_usable_sample_count();
        log::info!("Using {:?} MSAA samples", msaa_samples);

        log::info!("Creating swap-chain command pool");
        let command_pool = VkCommandPool::new(&device, device.graphics_queue_family);

        let depth_format = VkImage::find_depth_format(&vk_context.physical_device);
        log::info!("Choosing depth format {:?}", depth_format);

        let (swap_chain_format, swap_chain_present_mode, swap_image_count) =
            Self::choose_swap_chain_format(device, &vk_context.surface);

        log::info!("Creating render pass");
        let render_pass = VkRenderPass::new(
            &device,
            swap_chain_format.format,
            depth_format,
            msaa_samples,
        );

        let vertex_shader_module = VkShaderModule::new_from_file(
            &device,
            vk::ShaderStageFlags::VERTEX,
            "shader/vert.spv",
            "main",
        );
        let fragment_shader_module = VkShaderModule::new_from_file(
            &device,
            vk::ShaderStageFlags::FRAGMENT,
            "shader/frag.spv",
            "main",
        );
        let descriptor_set_layout = Self::create_descriptor_set_layout(&vk_context);
        let descriptor_pool = Self::create_descriptor_pool(&vk_context, swap_image_count);

        let vertex_buffer = Self::create_vertex_buffer(&vk_context, &command_pool);
        let index_buffer = Self::create_index_buffer(&vk_context, &command_pool);
        let texture_image = Self::create_texture_image(&vk_context, &command_pool);
        let sampler = Self::create_sampler(&vk_context, &texture_image);
        let window_size = window.inner_size();

        let mut app = TutorialApp {
            start_time: Instant::now(),
            swap_chain_context: None,
            sampler,
            texture_image,
            index_buffer,
            vertex_buffer,
            descriptor_set_layout,
            descriptor_pool,
            vertex_shader_module,
            fragment_shader_module,
            render_pass,
            swap_chain_format,
            swap_chain_present_mode,
            swap_image_count,
            command_pool,
            msaa_samples,
            depth_format,
            vk_context,
            window_size,
        };
        app.swap_chain_context = Some(app.create_swap_chain(app.window_size));
        app.record_commands();

        app
    }

    fn choose_swap_chain_format(
        device: &VkDevice,
        surface: &VkSurface,
    ) -> (vk::SurfaceFormatKHR, vk::PresentModeKHR, u32) {
        let surface_caps =
            surface.get_physical_device_surface_capabilities(&device.physical_device);
        let format = Self::choose_swapchain_surface_format(&surface_caps.formats);
        log::info!("Choosing swap-chain image format: {:?}", format);
        let present_mode = Self::choose_swapchain_surface_present_mode(&surface_caps.present_modes);
        log::info!("Choosing swap-chain presentation mode: {:?}", present_mode);
        let image_count = Self::choose_image_count(&surface_caps.capabilities);
        log::info!("Choosing swap-chain image count: {}", image_count);

        (format, present_mode, image_count)
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

    fn choose_image_count(capabilities: &vk::SurfaceCapabilitiesKHR) -> u32 {
        let max = capabilities.max_image_count;
        let mut preferred = capabilities.min_image_count + 1;
        if max > 0 && preferred > max {
            preferred = max;
        }
        preferred
    }

    fn create_swap_chain(&mut self, size: PhysicalSize<u32>) -> TutorialAppSwapChainContext {
        log::info!("Creating swap-chain");

        let mut swap_chain = VkSwapChain::new(
            &self.vk_context.device,
            &self.vk_context.surface,
            self.swap_chain_format,
            self.swap_chain_present_mode,
            self.swap_image_count,
            &[size.width, size.height],
        );
        swap_chain.initialize_images(
            2,
            &self.render_pass,
            self.depth_format,
            self.msaa_samples,
            &self.command_pool,
            self.vk_context.device.graphics_queue,
        );

        let context = &self.vk_context;
        let pipeline = self.create_pipeline(swap_chain.extent);
        let uniform_buffers = Self::create_uniform_buffers(context, self.swap_image_count);
        let descriptor_sets = Self::create_descriptor_sets(
            &context.device,
            &self.descriptor_pool,
            &self.descriptor_set_layout,
            &uniform_buffers,
            &self.texture_image,
            &self.sampler,
        );

        TutorialAppSwapChainContext {
            swap_chain,
            current_frame: 0,
            pipeline,
            uniform_buffers,
            descriptor_sets,
        }        
    }

    fn recreate_swap_chain(&mut self, size: PhysicalSize<u32>) {
        self.vk_context.device.wait_idle();
        self.destroy_descriptor_sets();
        self.swap_chain_context = Some(self.create_swap_chain(size));
        self.record_commands();
    }

    fn create_descriptor_set_layout(context: &VkContext) -> VkDescriptorSetLayout {
        let ubo_layout_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::VERTEX);
        let sampler_layout_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(1)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER) // TODO Use combined or separate?
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT);
        VkDescriptorSetLayout::new(
            &context.device,
            &[ubo_layout_binding.build(), sampler_layout_binding.build()],
        )
    }

    fn create_pipeline(&self, extent: vk::Extent2D) -> VkPipeline {
        VkPipeline::new(
            &self.vk_context.device,
            extent,
            &self.render_pass,
            &self.vertex_shader_module,
            &self.fragment_shader_module,
            &[self.descriptor_set_layout.handle],
            self.msaa_samples,
        )
    }

    fn create_vertex_buffer(context: &VkContext, command_pool: &VkCommandPool) -> VkBuffer {
        VkBuffer::new_device_local(
            &context.device,
            &command_pool,
            context.device.graphics_queue,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            &VERTICES,
        )
    }

    fn create_index_buffer(context: &VkContext, command_pool: &VkCommandPool) -> VkBuffer {
        VkBuffer::new_device_local(
            &context.device,
            &command_pool,
            context.device.graphics_queue,
            vk::BufferUsageFlags::INDEX_BUFFER,
            &INDICES,
        )
    }

    fn create_uniform_buffers(context: &VkContext, count: u32) -> Vec<VkBuffer> {
        let size = std::mem::size_of::<UniformBufferObject>() as u64;
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

    fn update_uniform_buffer(buffer: &VkBuffer, extent: vk::Extent2D, elapsed_time: f32) {
        let screen_width = extent.width as f32;
        let screen_height = extent.height as f32;
        let ubo = UniformBufferObject {
            model: Mat4::rotate_z(elapsed_time),
            view: Mat4::look_at(
                &Vec3::new(1.0, 1.0, 3.0),
                &Vec3::new(0.0, 0.0, 0.0),
                &Vec3::new(0.0, 1.0, 0.0),
            ),
            proj: Mat4::perspective(0.785, screen_width / screen_height, 0.1, 10.0),
        };

        buffer.map_memory(&[ubo]);
    }

    fn create_descriptor_pool(context: &VkContext, count: u32) -> VkDescriptorPool {
        let ubo_pool_size = vk::DescriptorPoolSize::builder()
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(count);
        let sampler_pool_size = vk::DescriptorPoolSize::builder()
            .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(count);

        VkDescriptorPool::new(
            &context.device,
            &[ubo_pool_size.build(), sampler_pool_size.build()],
            count,
        )
    }

    fn create_descriptor_sets(
        device: &VkDevice,
        pool: &VkDescriptorPool,
        layout: &VkDescriptorSetLayout,
        uniform_buffers: &[VkBuffer],
        texture: &VkTexture,
        sampler: &VkSampler,
    ) -> Vec<vk::DescriptorSet> {
        let count = uniform_buffers.len();
        log::info!("Creating {} descriptor sets", count);

        let descriptor_sets = pool.create_descriptor_sets(layout, count);

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

                let image_info = vk::DescriptorImageInfo::builder()
                    .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                    .image_view(texture.view)
                    .sampler(sampler.handle)
                    .build();
                let image_infos = [image_info];

                let image_descriptor_write = vk::WriteDescriptorSet::builder()
                    .dst_set(*set)
                    .dst_binding(1)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(&image_infos)
                    .build();

                let descriptor_writes = [ubo_descriptor_write, image_descriptor_write];

                unsafe {
                    device
                        .handle
                        .update_descriptor_sets(&descriptor_writes, &[])
                }
            });

        descriptor_sets
    }

    fn destroy_descriptor_sets(&self) {
        self.descriptor_pool.reset_descriptor_sets();
    }

    fn create_texture_image(context: &VkContext, command_pool: &VkCommandPool) -> VkTexture {
        VkImage::load_texture(
            &context.device,
            "assets/texture.jpg",
            &command_pool,
            context.device.graphics_queue, // TODO: Use transfer queue
        )
    }

    fn create_sampler(context: &VkContext, texture: &VkTexture) -> VkSampler {
        let properties = context.device.get_properties();
        VkSampler::new(
            &context.device,
            texture.image.mip_levels,
            properties.limits.max_sampler_anisotropy,
        )
    }

    fn record_commands(&self) {
        let context = &self.vk_context;
        let device = &context.device.handle;
        let swap_context = match &self.swap_chain_context {
            Some(ref context) => context,
            None => return,
        };

        let swap_chain = &swap_context.swap_chain;
        for (index, swap_image) in swap_chain.images.iter().enumerate() {
            let buffer = swap_image.command_buffer;
            let command_begin_info = vk::CommandBufferBeginInfo::builder();
            unsafe {
                device
                    .begin_command_buffer(buffer, &command_begin_info)
                    .expect("Unable to begin command buffer")
            };

            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    },
                },
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
            ];

            let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
                .render_pass(self.render_pass.handle)
                .framebuffer(swap_image.framebuffer)
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: swap_chain.extent,
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
        let swap_context = match &mut self.swap_chain_context {
            Some(context) => context,
            None => return,
        };

        let current_frame = swap_context.current_frame;
        let swap_chain = &mut swap_context.swap_chain;
        let context = &self.vk_context;
        let device = &context.device;

        let swap_frame = &swap_chain.frames[current_frame];
        let fence = &swap_frame.in_flight;

        device.wait_for_fences(&[fence]);

        let acquire_result = swap_chain.acquire_next_image(&swap_frame.available);
        let image_index = match acquire_result {
            Ok((index, _)) => index as usize,
            Err(_) => {
                self.recreate_swap_chain(window.inner_size());
                return;
            }
        };

        let swap_image = &mut swap_chain.images[image_index];
        if let Some(image_frame) = swap_image.frame {
            let in_flight_fence = &swap_chain.frames[image_frame].in_flight;
            device.wait_for_fences(&[in_flight_fence]);
        }

        swap_image.frame = Some(current_frame);

        let fence = &swap_frame.in_flight;
        let elapsed_time = self.start_time.elapsed().as_secs_f32();
        Self::update_uniform_buffer(
            &swap_context.uniform_buffers[image_index],
            swap_chain.extent,
            elapsed_time,
        );

        let wait_semaphores = [swap_frame.available.handle];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [swap_image.command_buffer];
        let signal_semaphores = [swap_frame.finished.handle];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);
        let infos = [submit_info.build()];

        device.reset_fences(&[fence]);
        unsafe {
            device
                .handle
                .queue_submit(context.device.graphics_queue, &infos, fence.handle)
                .expect("Unable to submit queue")
        };

        swap_context.current_frame = swap_chain.advance_frame(current_frame);

        let result = swap_chain.present_image(
            device.presentation_queue,
            image_index as _,
            &[&swap_frame.finished],
        );
        match result {
            Ok(true) | Err(_) => {
                self.recreate_swap_chain(window.inner_size());
            }
            Ok(false) => (),
        }
    }
}
