use std::mem::align_of;

use ash::{version::DeviceV1_0, vk};

use super::{physical_device::VkPhysicalDevice, VkDevice};

pub struct VkBuffer {
    pub handle: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub size: u64,
}

impl VkBuffer {
    pub fn new(
        instance: &ash::Instance,
        physical_device: &VkPhysicalDevice,
        device: &VkDevice,
        size: u64,
    ) -> VkBuffer {
        let handle = create_vertex_buffer(device, size);
        let memory = assign_buffer_memory(instance, physical_device, device, handle);

        VkBuffer {
            handle,
            memory,
            size,
        }
    }

    pub fn map_memory<T: Copy>(&self, device: &VkDevice, data: &[T]) {
        unsafe {
            let ptr = device
                .handle
                .map_memory(self.memory, 0, self.size, vk::MemoryMapFlags::empty())
                .expect("Unable to map memory");
            let mut align = ash::util::Align::new(ptr, align_of::<u8>() as _, self.size);
            align.copy_from_slice(data);
            device.unmap_memory(self.memory);
        }
    }

    pub fn cleanup(&self, device: &VkDevice) {
        unsafe {
            device.handle.destroy_buffer(self.handle, None);
        }
        unsafe {
            device.handle.free_memory(self.memory, None);
        }
    }
}

fn create_vertex_buffer(device: &VkDevice, size: u64) -> vk::Buffer {
    let buffer_info = vk::BufferCreateInfo::builder()
        .size(size)
        .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    unsafe {
        device
            .handle
            .create_buffer(&buffer_info, None)
            .expect("Unable to create vertex buffer")
    }
}

fn assign_buffer_memory(
    instance: &ash::Instance,
    physical_device: &VkPhysicalDevice,
    device: &VkDevice,
    buffer: vk::Buffer,
) -> vk::DeviceMemory {
    let mem_requirements = unsafe { device.handle.get_buffer_memory_requirements(buffer) };
    let mem_type_index = find_memory_type(
        mem_requirements,
        physical_device.get_mem_properties(instance),
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    );

    let alloc_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(mem_requirements.size)
        .memory_type_index(mem_type_index)
        .build();
    let memory = unsafe {
        let vertex_buffer_memory = device
            .allocate_memory(&alloc_info, None)
            .expect("Unable to allocate buffer memory");
        device
            .bind_buffer_memory(buffer, vertex_buffer_memory, 0)
            .expect("Unable to bind image memory");
        vertex_buffer_memory
    };
    memory
}

fn find_memory_type(
    requirements: vk::MemoryRequirements,
    mem_properties: vk::PhysicalDeviceMemoryProperties,
    required_properties: vk::MemoryPropertyFlags,
) -> u32 {
    for i in 0..mem_properties.memory_type_count {
        if requirements.memory_type_bits & (1 << i) != 0
            && mem_properties.memory_types[i as usize]
                .property_flags
                .contains(required_properties)
        {
            return i;
        }
    }
    panic!("Failed to find suitable memory type.")
}
