use ash::{version::DeviceV1_0, vk};

use crate::render::Vertex;

use super::{VkShaderModule, device::VkDevice, render_pass::VkRenderPass, swap_chain::VkSwapChain};

use memoffset::offset_of;

pub struct VkPipeline {
    pub handle: vk::Pipeline,
    pub layout: vk::PipelineLayout,
}

impl VkPipeline {
    pub fn new(
        device: &VkDevice,
        swap_chain: &VkSwapChain,
        render_pass: &VkRenderPass,
        vertex_shader_module: &VkShaderModule,
        fragment_shader_module: &VkShaderModule,
    ) -> VkPipeline {
        log::info!("Creating pipeline");
        
        let vertex_shader_stage_info = vertex_shader_module.create_pipeline_shader_stage();
        let fragment_shader_stage_info = fragment_shader_module.create_pipeline_shader_stage();
        let shader_stages = [vertex_shader_stage_info.build(), fragment_shader_stage_info.build()];

        // TODO: Where to put this?
        let vertex_input_binding = create_vertex_input_binding_description();
        let binding_descriptions = [vertex_input_binding];
        let position_vertex_attribute = create_position_vertex_input_attribute_description();
        let color_vertex_attribute = create_color_vertex_input_attribute_description();
        let attribute_descriptions = [position_vertex_attribute, color_vertex_attribute];
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions);

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewports = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: swap_chain.swap_extent.width as f32,
            height: swap_chain.swap_extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let scissors = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swap_chain.swap_extent,
        }];
        let viewport_info = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterizer_info = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0f32)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false)
            .depth_bias_constant_factor(0.0)
            .depth_bias_clamp(0.0)
            .depth_bias_slope_factor(0.0);

        let multisampling_info = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            .min_sample_shading(1.0f32)
            // .sample_masks()
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false);

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(vk::ColorComponentFlags::all())
            .blend_enable(false)
            .src_color_blend_factor(vk::BlendFactor::ONE)
            .dst_color_blend_factor(vk::BlendFactor::ZERO)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            .alpha_blend_op(vk::BlendOp::ADD)
            .build();
        let color_blend_attachments = [color_blend_attachment];

        let color_blending_info = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(&color_blend_attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let layout_info = vk::PipelineLayoutCreateInfo::builder();
        let layout = unsafe {
            device
                .handle
                .create_pipeline_layout(&layout_info, None)
                .expect("Unable to create pipeline layout")
        };

        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_info)
            .rasterization_state(&rasterizer_info)
            .multisample_state(&multisampling_info)
            .color_blend_state(&color_blending_info)
            .layout(layout)
            .render_pass(render_pass.handle)
            .subpass(0)
            .build();
        let pipeline_infos = [pipeline_info];

        let handle = unsafe {
            device
                .create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_infos, None)
                .expect("Unable t ocreate graphics pipelines")[0]
        };

        VkPipeline {
            layout,
            handle,
        }
    }

    pub fn cleanup(&self, device: &VkDevice) {
        log::debug!("Dropping pipeline");

        let handle = &device.handle;
        unsafe {
            handle.destroy_pipeline(self.handle, None);
            handle.destroy_pipeline_layout(self.layout, None);            
        }
    }
}

fn create_vertex_input_binding_description() -> vk::VertexInputBindingDescription {
    vk::VertexInputBindingDescription::builder()
        .binding(0)
        .stride(std::mem::size_of::<Vertex>() as u32)
        .input_rate(vk::VertexInputRate::VERTEX)
        .build()
}

fn create_position_vertex_input_attribute_description() -> vk::VertexInputAttributeDescription {
    vk::VertexInputAttributeDescription::builder()
        .binding(0)
        .location(0)
        .format(vk::Format::R32G32B32_SFLOAT)
        .offset(offset_of!(Vertex, position) as u32)
        .build()
}

fn create_color_vertex_input_attribute_description() -> vk::VertexInputAttributeDescription {
    vk::VertexInputAttributeDescription::builder()
        .binding(0)
        .location(1)
        .format(vk::Format::R32G32B32_SFLOAT)
        .offset(offset_of!(Vertex, color) as u32)
        .build()
}
