use std::{
    ffi::CString,
    fs::File,
    io::{Cursor, Read},
    sync::Arc,
};

use ash::{version::DeviceV1_0, vk};

use super::device::VkDevice;

pub struct VkShaderModule {
    device: Arc<VkDevice>,
    pub handle: vk::ShaderModule,
    pub stage: vk::ShaderStageFlags,
    pub entry_point: CString,
}

impl VkShaderModule {
    pub fn new_from_file(
        device: &Arc<VkDevice>,
        stage: vk::ShaderStageFlags,
        path: &str,
        entry_point: &str,
    ) -> VkShaderModule {
        log::info!(
            "Creating shader module from file {}, entry point {}",
            path,
            entry_point
        );

        let mut buf = Vec::new();
        let mut file = File::open(path).expect("Unable to open shader file");
        file.read_to_end(&mut buf)
            .expect("Unable to read shader file");
        let mut cursor = Cursor::new(buf);
        let binary = ash::util::read_spv(&mut cursor).expect("Unable to read shader");
        let create_info = vk::ShaderModuleCreateInfo::builder().code(&binary);
        let handle = unsafe {
            device
                .handle
                .create_shader_module(&create_info, None)
                .expect("Unable to create shader module")
        };

        VkShaderModule {
            device: Arc::clone(device),
            handle,
            stage,
            entry_point: CString::new(entry_point).unwrap(),
        }
    }

    pub fn create_pipeline_shader_stage(&self) -> vk::PipelineShaderStageCreateInfoBuilder {
        vk::PipelineShaderStageCreateInfo::builder()
            .stage(self.stage)
            .module(self.handle)
            .name(&self.entry_point)
    }
}

impl Drop for VkShaderModule {
    fn drop(&mut self) {
        log::debug!("Dropping shader module");
        unsafe {
            self.device.handle.destroy_shader_module(self.handle, None);
        }
    }
}
