use std::ffi::CString;

use ash::{version::DeviceV1_0, vk};

use super::{device::VkDevice, shader::read_shader_from_file};

pub struct VkPipeline {
    pub vertex_shader_module: vk::ShaderModule,
    pub fragment_shader_module: vk::ShaderModule,
}

impl VkPipeline {
    pub fn new(device: &VkDevice) -> VkPipeline {
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

        VkPipeline {
            vertex_shader_module,
            fragment_shader_module,
        }
    }

    pub fn cleanup(&self, device: &VkDevice) {
        log::debug!("Dropping pipeline");

        let handle = &device.handle;
        unsafe {
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
