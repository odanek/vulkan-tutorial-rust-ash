use std::{mem::align_of, sync::Arc};

use ash::vk;

use super::{VkCommandPool, VkDevice};

pub struct VkBuffer {
    device: Arc<VkDevice>,
    pub handle: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub size: u64,
}

impl VkBuffer {
    pub fn new(
        device: &Arc<VkDevice>,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
        size: u64,
    ) -> VkBuffer {
        let handle = create_vertex_buffer(device, usage, size);
        let memory = assign_buffer_memory(device, handle, properties);

        VkBuffer {
            device: Arc::clone(device),
            handle,
            memory,
            size,
        }
    }

    pub fn new_device_local<T: Copy>(
        device: &Arc<VkDevice>,
        command_pool: &Arc<VkCommandPool>,
        queue: vk::Queue,
        usage: vk::BufferUsageFlags,
        data: &[T],
    ) -> VkBuffer {
        let size = (data.len() * std::mem::size_of::<T>()) as u64;
        log::info!("creating device-local buffer of size {}", size);

        let staging_buffer = VkBuffer::new(
            device,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            size,
        );
        staging_buffer.map_memory(data);

        let buffer = VkBuffer::new(
            device,
            usage | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            size,
        );

        log::info!("Copying buffer data");
        device.copy_buffer(&staging_buffer, &buffer, command_pool, queue);

        buffer
    }

    pub fn map_memory<T: Copy>(&self, data: &[T]) {
        unsafe {
            let ptr = self
                .device
                .handle
                .map_memory(self.memory, 0, self.size, vk::MemoryMapFlags::empty())
                .expect("Unable to map memory");
            let mut align = ash::util::Align::new(ptr, align_of::<u8>() as _, self.size);
            align.copy_from_slice(data);
            self.device.handle.unmap_memory(self.memory);
        }
    }
}

impl Drop for VkBuffer {
    fn drop(&mut self) {
        log::debug!("Dropping buffer");
        unsafe {
            self.device.handle.destroy_buffer(self.handle, None);
            self.device.handle.free_memory(self.memory, None);
        }
    }
}

fn create_vertex_buffer(device: &VkDevice, usage: vk::BufferUsageFlags, size: u64) -> vk::Buffer {
    let buffer_info = vk::BufferCreateInfo::builder()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    unsafe {
        device
            .handle
            .create_buffer(&buffer_info, None)
            .expect("Unable to create vertex buffer")
    }
}

fn assign_buffer_memory(
    device: &VkDevice,
    buffer: vk::Buffer,
    properties: vk::MemoryPropertyFlags,
) -> vk::DeviceMemory {
    let mem_requirements = unsafe { device.handle.get_buffer_memory_requirements(buffer) };
    let mem_type_index = device.find_memory_type(mem_requirements, properties);

    let alloc_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(mem_requirements.size)
        .memory_type_index(mem_type_index)
        .build();
    unsafe {
        let vertex_buffer_memory = device
            .handle
            .allocate_memory(&alloc_info, None)
            .expect("Unable to allocate buffer memory");
        device
            .handle
            .bind_buffer_memory(buffer, vertex_buffer_memory, 0)
            .expect("Unable to bind image memory");
        vertex_buffer_memory
    }
}
