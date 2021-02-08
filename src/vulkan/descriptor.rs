use ash::{version::DeviceV1_0, vk};

use super::VkDevice;

pub struct VkDescriptorSetLayout {
    pub handle: vk::DescriptorSetLayout,
}

impl VkDescriptorSetLayout {
    pub fn new(
        device: &VkDevice,
        bindings: &[vk::DescriptorSetLayoutBinding],
    ) -> VkDescriptorSetLayout {
        let layout_info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(bindings);
        let handle = unsafe {
            device
                .handle
                .create_descriptor_set_layout(&layout_info, None)
                .expect("Unable to create descriptor set layout")
        };
        VkDescriptorSetLayout { handle }
    }

    pub fn cleanup(&self, device: &VkDevice) {
        unsafe {
            device.destroy_descriptor_set_layout(self.handle, None);
        }
    }
}

pub struct VkDescriptorPool {
    pub handle: vk::DescriptorPool,
}

impl VkDescriptorPool {
    pub fn new(device: &VkDevice, ty: vk::DescriptorType, count: u32) -> VkDescriptorPool {
        let pool_size = vk::DescriptorPoolSize::builder()
            .ty(ty)
            .descriptor_count(count);
        let pool_sizes = [pool_size.build()];

        let create_info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(&pool_sizes)
            .max_sets(count);

        let handle = unsafe {
            device
                .create_descriptor_pool(&create_info, None)
                .expect("Unable to create descriptor pool")
        };

        VkDescriptorPool { handle }
    }

    pub fn cleanup(&self, device: &VkDevice) {
        unsafe {            
            device.destroy_descriptor_pool(self.handle, None);
        }
    }
}

