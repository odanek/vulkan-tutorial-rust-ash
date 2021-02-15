use std::sync::Arc;

use ash::{version::DeviceV1_0, vk};

use super::VkDevice;

pub struct VkDescriptorSetLayout {
    device: Arc<VkDevice>,
    pub handle: vk::DescriptorSetLayout,
}

impl VkDescriptorSetLayout {
    pub fn new(
        device: &Arc<VkDevice>,
        bindings: &[vk::DescriptorSetLayoutBinding],
    ) -> VkDescriptorSetLayout {
        let layout_info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(bindings);
        let handle = unsafe {
            device
                .handle
                .create_descriptor_set_layout(&layout_info, None)
                .expect("Unable to create descriptor set layout")
        };
        VkDescriptorSetLayout {
            device: Arc::clone(device),
            handle,
        }
    }
}

impl Drop for VkDescriptorSetLayout {
    fn drop(&mut self) {
        log::debug!("Dropping descriptor set layout");
        unsafe {
            self.device
                .handle
                .destroy_descriptor_set_layout(self.handle, None);
        }
    }
}

pub struct VkDescriptorPool {
    device: Arc<VkDevice>,
    pub handle: vk::DescriptorPool,
}

impl VkDescriptorPool {
    pub fn new(device: &Arc<VkDevice>, pool_sizes: &[vk::DescriptorPoolSize], count: u32) -> VkDescriptorPool {
        let create_info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(&pool_sizes)
            .max_sets(count);

        let handle = unsafe {
            device
                .handle
                .create_descriptor_pool(&create_info, None)
                .expect("Unable to create descriptor pool")
        };

        VkDescriptorPool {
            device: Arc::clone(device),
            handle,
        }
    }
}

impl Drop for VkDescriptorPool {
    fn drop(&mut self) {
        log::debug!("Dropping descriptor pool");
        unsafe {
            self.device
                .handle
                .destroy_descriptor_pool(self.handle, None);
        }
    }
}
