use std::{
    fs::File,
    io::{Cursor, Read},
};

use ash::{version::DeviceV1_0, vk};

use super::device::VkDevice;

pub fn read_shader_from_file(path: &str, device: &VkDevice) -> vk::ShaderModule {
    log::info!("Loading shader file {}", path);

    let mut buf = Vec::new();
    let mut file = File::open(path).expect("Unable to open shader file");
    file.read_to_end(&mut buf).unwrap();
    let mut cursor = Cursor::new(buf);
    let binary = ash::util::read_spv(&mut cursor).expect("Unable to read shader");
    let create_info = vk::ShaderModuleCreateInfo::builder().code(&binary);
    unsafe {
        device
            .handle
            .create_shader_module(&create_info, None)
            .unwrap()
    }
}
