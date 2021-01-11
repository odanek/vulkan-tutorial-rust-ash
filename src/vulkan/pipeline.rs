use std::ffi::CString;

use ash::{version::DeviceV1_0, vk};

use super::{
    device::VkDevice, render_pass::VkRenderPass, shader::read_shader_from_file,
    swap_chain::VkSwapChain,
};

pub struct VkPipeline {
    pub vertex_shader_module: vk::ShaderModule,
    pub fragment_shader_module: vk::ShaderModule,
    pub layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
}

impl VkPipeline {
    pub fn new(
        device: &VkDevice,
        swap_chain: &VkSwapChain,
        render_pass: &VkRenderPass,
    ) -> VkPipeline {
        log::info!("Creating pipeline");

        let vertex_shader_module = read_shader_from_file("shader/vert.spv", device);
        let fragment_shader_module = read_shader_from_file("shader/frag.spv", device);

        let entry_point_name = CString::new("main").unwrap();
        let vertex_shader_stage_info = create_shader_stage(
            vk::ShaderStageFlags::VERTEX,
            vertex_shader_module,
            &entry_point_name,
        );
        let fragment_shader_stage_info = create_shader_stage(
            vk::ShaderStageFlags::FRAGMENT,
            fragment_shader_module,
            &entry_point_name,
        );
        let shader_stages = [vertex_shader_stage_info, fragment_shader_stage_info];

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder();

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

        let pipeline = unsafe {
            device
                .create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_infos, None)
                .unwrap()[0]
        };

        VkPipeline {
            vertex_shader_module,
            fragment_shader_module,
            layout,
            pipeline,
        }
    }

    pub fn cleanup(&self, device: &VkDevice) {
        log::debug!("Dropping pipeline");

        let handle = &device.handle;
        unsafe {
            handle.destroy_pipeline(self.pipeline, None);
            handle.destroy_pipeline_layout(self.layout, None);
            handle.destroy_shader_module(self.vertex_shader_module, None);
            handle.destroy_shader_module(self.fragment_shader_module, None);
        }
    }
}

fn create_shader_stage(
    stage: vk::ShaderStageFlags,
    module: vk::ShaderModule,
    entry_point: &CString,
) -> vk::PipelineShaderStageCreateInfo {
    vk::PipelineShaderStageCreateInfo::builder()
        .stage(stage)
        .module(module)
        .name(&entry_point)
        .build()
}