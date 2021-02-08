use std::mem::align_of;

use ash::{version::DeviceV1_0, vk};

use super::{physical_device::VkPhysicalDevice, VkCommandPool, VkDevice};

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
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
        size: u64,
    ) -> VkBuffer {
        let handle = create_vertex_buffer(device, usage, size);
        let memory = assign_buffer_memory(instance, physical_device, device, handle, properties);

        VkBuffer {
            handle,
            memory,
            size,
        }
    }

    pub fn new_device_local<T: Copy>(
        instance: &ash::Instance,
        physical_device: &VkPhysicalDevice,
        device: &VkDevice,
        command_pool: &VkCommandPool,
        queue: vk::Queue,
        usage: vk::BufferUsageFlags,
        data: &[T],
    ) -> VkBuffer {
        let size = (data.len() * std::mem::size_of::<T>()) as u64;
        log::info!("creating device-local buffer of size {}", size);

        let staging_buffer = VkBuffer::new(
            instance,
            physical_device,
            device,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            size,
        );
        staging_buffer.map_memory(device, data);

        let buffer = VkBuffer::new(
            instance,
            physical_device,
            device,
            usage | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            size,
        );

        log::info!("Copying buffer data");
        VkBuffer::copy(device, command_pool, queue, &staging_buffer, &buffer);
        staging_buffer.cleanup(device);

        buffer
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

    pub fn copy(
        device: &VkDevice,
        command_pool: &VkCommandPool,
        queue: vk::Queue,
        src: &VkBuffer,
        dst: &VkBuffer,
    ) {
        let command_buffer = command_pool.create_command_buffer(device);
        let command_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            device
                .begin_command_buffer(command_buffer, &command_begin_info)
                .expect("Unable to begin command buffer")
        };

        unsafe {
            let regions = [vk::BufferCopy {
                src_offset: 0,
                dst_offset: 0,
                size: src.size,
            }];
            device
                .handle
                .cmd_copy_buffer(command_buffer, src.handle, dst.handle, &regions);
            device
                .handle
                .end_command_buffer(command_buffer)
                .expect("Unable to end command buffer");

            let command_buffers = [command_buffer];
            let submit_info = vk::SubmitInfo::builder().command_buffers(&command_buffers);
            let infos = [submit_info.build()];
            device
                .queue_submit(queue, &infos, vk::Fence::null())
                .expect("Unable to submit queue");
            device
                .queue_wait_idle(queue)
                .expect("Unable to wait for queue idle state");
        }

        command_pool.clear_command_buffer(device, command_buffer);
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
    instance: &ash::Instance,
    physical_device: &VkPhysicalDevice,
    device: &VkDevice,
    buffer: vk::Buffer,
    properties: vk::MemoryPropertyFlags,
) -> vk::DeviceMemory {
    let mem_requirements = unsafe { device.handle.get_buffer_memory_requirements(buffer) };
    let physical_mem_properties = physical_device.get_mem_properties(instance);
    let mem_type_index = find_memory_type(mem_requirements, physical_mem_properties, properties);

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
