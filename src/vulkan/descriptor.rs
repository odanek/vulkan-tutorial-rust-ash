use std::sync::Arc;

use ash::vk;

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
    pub fn new(
        device: &Arc<VkDevice>,
        pool_sizes: &[vk::DescriptorPoolSize],
        count: u32,
    ) -> VkDescriptorPool {
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

    pub fn create_descriptor_sets(
        &self,
        layout: &VkDescriptorSetLayout,
        count: usize,
    ) -> Vec<vk::DescriptorSet> {
        let layouts = (0..count).map(|_| layout.handle).collect::<Vec<_>>();
        let alloc_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.handle)
            .set_layouts(&layouts)
            .build();
        unsafe {
            self.device
                .handle
                .allocate_descriptor_sets(&alloc_info)
                .expect("Unable to create descriptor sets")
        }
    }

    pub fn reset_descriptor_sets(&self) {
        unsafe {
            self.device
                .handle
                .reset_descriptor_pool(self.handle, vk::DescriptorPoolResetFlags::empty())
                .expect("Resetting descriptor pool failed");
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
