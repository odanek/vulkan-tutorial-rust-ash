use ash::vk;

pub struct VkQueueFamily {
    pub index: u32,
    pub queue_count: u32,
    pub flags: vk::QueueFlags,
}

impl VkQueueFamily {
    pub fn new(index: u32, queue_family: &vk::QueueFamilyProperties) -> VkQueueFamily {
        VkQueueFamily {
            index,
            queue_count: queue_family.queue_count,
            flags: queue_family.queue_flags,
        }
    }
}
