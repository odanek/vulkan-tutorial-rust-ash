use winit::window::Window;

use ash::{version::DeviceV1_0, vk, Entry};

use super::{command::VkCommandPool, debug::VkValidation, device::VkDevice, fence::VkFence, instance::VkInstance, physical_device::VkPhysicalDevice, pipeline::VkPipeline, render_pass::VkRenderPass, semaphore::VkSemaphore, settings::VkSettings, surface::VkSurface, swap_chain::VkSwapChain};

pub struct VkContext {
    pub max_frames_in_flight: usize,
    pub current_frame: usize,

    pub image_available_semaphore: Vec<VkSemaphore>,
    pub render_finished_semaphore: Vec<VkSemaphore>,
    pub in_flight_fences: Vec<VkFence>,

    pub command_pool: VkCommandPool,
    pub pipeline: VkPipeline,
    pub render_pass: VkRenderPass,
    pub swap_chain: VkSwapChain,
    pub device: VkDevice,
    pub physical_device: VkPhysicalDevice,
    pub surface: VkSurface,
    pub validation: Option<VkValidation>,
    pub instance: VkInstance,
    pub entry: ash::Entry,
}

impl VkContext {
    pub fn new(window: &Window, settings: &VkSettings) -> VkContext {
        let entry = Entry::new().expect("Failed to create Vulkan entry.");
        let instance = VkInstance::new(window, &entry, settings.validation);
        let validation = if settings.validation {
            Some(VkValidation::new(&entry, &instance))
        } else {
            None
        };
        let surface = VkSurface::new(&entry, &instance, window);
        let physical_device = VkPhysicalDevice::new(&instance, &surface);
        let device = VkDevice::new(&instance, &physical_device, &surface);

        let window_size = window.inner_size();
        let mut swap_chain = VkSwapChain::new(
            &instance,
            &physical_device,
            &device,
            &surface,
            &[window_size.width, window_size.height],
        );

        let render_pass = VkRenderPass::new(&device, &swap_chain);
        let pipeline = VkPipeline::new(&device, &swap_chain, &render_pass);

        swap_chain.create_frame_buffers(&device, &render_pass);

        let command_pool = VkCommandPool::new(&device, swap_chain.framebuffers.len() as u32);

        let max_frames_in_flight = 2usize;
        let image_available_semaphore = create_semaphores(&device, max_frames_in_flight);
        let render_finished_semaphore = create_semaphores(&device, max_frames_in_flight);
        let in_flight_fences = create_fences(&device, max_frames_in_flight);

        VkContext {
            max_frames_in_flight,
            current_frame: 0,

            image_available_semaphore,
            render_finished_semaphore,
            in_flight_fences,

            command_pool,
            pipeline,
            render_pass,
            swap_chain,
            device,
            physical_device,
            surface,
            validation,
            instance,
            entry,
        }
    }

    pub fn record_commands(&self) {
        let device = &self.device.handle;

        for (index, &buffer) in self.command_pool.buffers.iter().enumerate() {
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
                .render_pass(self.render_pass.handle)
                .framebuffer(self.swap_chain.framebuffers[index])
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: self.swap_chain.swap_extent,
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
                device.cmd_draw(buffer, 3, 1, 0, 0);

                device.cmd_end_render_pass(buffer);

                device
                    .end_command_buffer(buffer)
                    .expect("Failed to record end of command buffer");
            };
        }
    }
}

impl Drop for VkContext {
    fn drop(&mut self) {
        self.image_available_semaphore
            .iter()
            .for_each(|semaphore| semaphore.cleanup(&self.device));
        self.render_finished_semaphore
            .iter()
            .for_each(|semaphore| semaphore.cleanup(&self.device));
        self.in_flight_fences
            .iter()
            .for_each(|fence| fence.cleanup(&self.device));

        self.command_pool.cleanup(&self.device);
        self.swap_chain.cleanup_framebuffers(&self.device);
        self.pipeline.cleanup(&self.device);
        self.render_pass.cleanup(&self.device);
        self.swap_chain.cleanup(&self.device);
        self.device.cleanup();
        self.surface.cleanup();

        let validation = self.validation.take();
        if let Some(mut validation) = validation {
            validation.cleanup();
        }
        self.instance.cleanup();
    }
}

fn create_semaphores(device: &VkDevice, count: usize) -> Vec<VkSemaphore> {
    (0..count)
        .map(|_| VkSemaphore::new(device))
        .collect::<Vec<_>>()
}

fn create_fences(device: &VkDevice, count: usize) -> Vec<VkFence> {
    (0..count)
        .map(|_| VkFence::new(device))
        .collect::<Vec<_>>()
}
