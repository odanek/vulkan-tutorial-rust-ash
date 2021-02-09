use ash::vk;

use super::VkDevice;

pub struct VkImage {
    pub handle: vk::Image,
    pub memory: vk::DeviceMemory,
    pub size: u64,
}

impl VkImage {
    pub fn new() -> VkImage {
        todo!();
    }

    pub fn load_file(device: &VkDevice, file_name: &str) -> VkImage {
        todo!();
    }
}

// fn create_texture_image(
//     vk_context: &VkContext,
//     command_pool: vk::CommandPool,
//     copy_queue: vk::Queue,
// ) -> Texture {
//     let cursor = fs::load("images/chalet.jpg");
//     let image = image::load(cursor, image::ImageFormat::Jpeg)
//         .unwrap()
//         .flipv();
//     let image_as_rgb = image.to_rgba();
//     let width = (&image_as_rgb).width();
//     let height = (&image_as_rgb).height();
//     let max_mip_levels = ((width.min(height) as f32).log2().floor() + 1.0) as u32;
//     let extent = vk::Extent2D { width, height };
//     let pixels = image_as_rgb.into_raw();
//     let image_size = (pixels.len() * size_of::<u8>()) as vk::DeviceSize;
//     let device = vk_context.device();

//     let (buffer, memory, mem_size) = Self::create_buffer(
//         vk_context,
//         image_size,
//         vk::BufferUsageFlags::TRANSFER_SRC,
//         vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
//     );

//     unsafe {
//         let ptr = device
//             .map_memory(memory, 0, image_size, vk::MemoryMapFlags::empty())
//             .unwrap();
//         let mut align = ash::util::Align::new(ptr, align_of::<u8>() as _, mem_size);
//         align.copy_from_slice(&pixels);
//         device.unmap_memory(memory);
//     }

//     let (image, image_memory) = Self::create_image(
//         vk_context,
//         vk::MemoryPropertyFlags::DEVICE_LOCAL,
//         extent,
//         max_mip_levels,
//         vk::SampleCountFlags::TYPE_1,
//         vk::Format::R8G8B8A8_UNORM,
//         vk::ImageTiling::OPTIMAL,
//         vk::ImageUsageFlags::TRANSFER_SRC
//             | vk::ImageUsageFlags::TRANSFER_DST
//             | vk::ImageUsageFlags::SAMPLED,
//     );

//     // Transition the image layout and copy the buffer into the image
//     // and transition the layout again to be readable from fragment shader.
//     {
//         Self::transition_image_layout(
//             device,
//             command_pool,
//             copy_queue,
//             image,
//             max_mip_levels,
//             vk::Format::R8G8B8A8_UNORM,
//             vk::ImageLayout::UNDEFINED,
//             vk::ImageLayout::TRANSFER_DST_OPTIMAL,
//         );

//         Self::copy_buffer_to_image(device, command_pool, copy_queue, buffer, image, extent);

//         Self::generate_mipmaps(
//             vk_context,
//             command_pool,
//             copy_queue,
//             image,
//             extent,
//             vk::Format::R8G8B8A8_UNORM,
//             max_mip_levels,
//         );
//     }

//     unsafe {
//         device.destroy_buffer(buffer, None);
//         device.free_memory(memory, None);
//     }

//     let image_view = Self::create_image_view(
//         device,
//         image,
//         max_mip_levels,
//         vk::Format::R8G8B8A8_UNORM,
//         vk::ImageAspectFlags::COLOR,
//     );

//     let sampler = {
//         let sampler_info = vk::SamplerCreateInfo::builder()
//             .mag_filter(vk::Filter::LINEAR)
//             .min_filter(vk::Filter::LINEAR)
//             .address_mode_u(vk::SamplerAddressMode::REPEAT)
//             .address_mode_v(vk::SamplerAddressMode::REPEAT)
//             .address_mode_w(vk::SamplerAddressMode::REPEAT)
//             .anisotropy_enable(true)
//             .max_anisotropy(16.0)
//             .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
//             .unnormalized_coordinates(false)
//             .compare_enable(false)
//             .compare_op(vk::CompareOp::ALWAYS)
//             .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
//             .mip_lod_bias(0.0)
//             .min_lod(0.0)
//             .max_lod(max_mip_levels as _)
//             .build();

//         unsafe { device.create_sampler(&sampler_info, None).unwrap() }
//     };

//     Texture::new(image, image_memory, image_view, Some(sampler))
// }

// fn create_image(
//     vk_context: &VkContext,
//     mem_properties: vk::MemoryPropertyFlags,
//     extent: vk::Extent2D,
//     mip_levels: u32,
//     sample_count: vk::SampleCountFlags,
//     format: vk::Format,
//     tiling: vk::ImageTiling,
//     usage: vk::ImageUsageFlags,
// ) -> (vk::Image, vk::DeviceMemory) {
//     let image_info = vk::ImageCreateInfo::builder()
//         .image_type(vk::ImageType::TYPE_2D)
//         .extent(vk::Extent3D {
//             width: extent.width,
//             height: extent.height,
//             depth: 1,
//         })
//         .mip_levels(mip_levels)
//         .array_layers(1)
//         .format(format)
//         .tiling(tiling)
//         .initial_layout(vk::ImageLayout::UNDEFINED)
//         .usage(usage)
//         .sharing_mode(vk::SharingMode::EXCLUSIVE)
//         .samples(sample_count)
//         .flags(vk::ImageCreateFlags::empty())
//         .build();

//     let device = vk_context.device();
//     let image = unsafe { device.create_image(&image_info, None).unwrap() };
//     let mem_requirements = unsafe { device.get_image_memory_requirements(image) };
//     let mem_type_index = Self::find_memory_type(
//         mem_requirements,
//         vk_context.get_mem_properties(),
//         mem_properties,
//     );

//     let alloc_info = vk::MemoryAllocateInfo::builder()
//         .allocation_size(mem_requirements.size)
//         .memory_type_index(mem_type_index)
//         .build();
//     let memory = unsafe {
//         let mem = device.allocate_memory(&alloc_info, None).unwrap();
//         device.bind_image_memory(image, mem, 0).unwrap();
//         mem
//     };

//     (image, memory)
// }

// fn transition_image_layout(
//     device: &Device,
//     command_pool: vk::CommandPool,
//     transition_queue: vk::Queue,
//     image: vk::Image,
//     mip_levels: u32,
//     format: vk::Format,
//     old_layout: vk::ImageLayout,
//     new_layout: vk::ImageLayout,
// ) {
//     Self::execute_one_time_commands(device, command_pool, transition_queue, |buffer| {
//         let (src_access_mask, dst_access_mask, src_stage, dst_stage) =
//             match (old_layout, new_layout) {
//                 (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => (
//                     vk::AccessFlags::empty(),
//                     vk::AccessFlags::TRANSFER_WRITE,
//                     vk::PipelineStageFlags::TOP_OF_PIPE,
//                     vk::PipelineStageFlags::TRANSFER,
//                 ),
//                 (
//                     vk::ImageLayout::TRANSFER_DST_OPTIMAL,
//                     vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
//                 ) => (
//                     vk::AccessFlags::TRANSFER_WRITE,
//                     vk::AccessFlags::SHADER_READ,
//                     vk::PipelineStageFlags::TRANSFER,
//                     vk::PipelineStageFlags::FRAGMENT_SHADER,
//                 ),
//                 (
//                     vk::ImageLayout::UNDEFINED,
//                     vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
//                 ) => (
//                     vk::AccessFlags::empty(),
//                     vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
//                         | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
//                     vk::PipelineStageFlags::TOP_OF_PIPE,
//                     vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
//                 ),
//                 (vk::ImageLayout::UNDEFINED, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL) => (
//                     vk::AccessFlags::empty(),
//                     vk::AccessFlags::COLOR_ATTACHMENT_READ
//                         | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
//                     vk::PipelineStageFlags::TOP_OF_PIPE,
//                     vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
//                 ),
//                 _ => panic!(
//                     "Unsupported layout transition({:?} => {:?}).",
//                     old_layout, new_layout
//                 ),
//             };

//         let aspect_mask = if new_layout == vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL {
//             let mut mask = vk::ImageAspectFlags::DEPTH;
//             if Self::has_stencil_component(format) {
//                 mask |= vk::ImageAspectFlags::STENCIL;
//             }
//             mask
//         } else {
//             vk::ImageAspectFlags::COLOR
//         };

//         let barrier = vk::ImageMemoryBarrier::builder()
//             .old_layout(old_layout)
//             .new_layout(new_layout)
//             .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
//             .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
//             .image(image)
//             .subresource_range(vk::ImageSubresourceRange {
//                 aspect_mask,
//                 base_mip_level: 0,
//                 level_count: mip_levels,
//                 base_array_layer: 0,
//                 layer_count: 1,
//             })
//             .src_access_mask(src_access_mask)
//             .dst_access_mask(dst_access_mask)
//             .build();
//         let barriers = [barrier];

//         unsafe {
//             device.cmd_pipeline_barrier(
//                 buffer,
//                 src_stage,
//                 dst_stage,
//                 vk::DependencyFlags::empty(),
//                 &[],
//                 &[],
//                 &barriers,
//             )
//         };
//     });
// }

// fn copy_buffer_to_image(
//     device: &Device,
//     command_pool: vk::CommandPool,
//     transition_queue: vk::Queue,
//     buffer: vk::Buffer,
//     image: vk::Image,
//     extent: vk::Extent2D,
// ) {
//     Self::execute_one_time_commands(device, command_pool, transition_queue, |command_buffer| {
//         let region = vk::BufferImageCopy::builder()
//             .buffer_offset(0)
//             .buffer_row_length(0)
//             .buffer_image_height(0)
//             .image_subresource(vk::ImageSubresourceLayers {
//                 aspect_mask: vk::ImageAspectFlags::COLOR,
//                 mip_level: 0,
//                 base_array_layer: 0,
//                 layer_count: 1,
//             })
//             .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
//             .image_extent(vk::Extent3D {
//                 width: extent.width,
//                 height: extent.height,
//                 depth: 1,
//             })
//             .build();
//         let regions = [region];
//         unsafe {
//             device.cmd_copy_buffer_to_image(
//                 command_buffer,
//                 buffer,
//                 image,
//                 vk::ImageLayout::TRANSFER_DST_OPTIMAL,
//                 &regions,
//             )
//         }
//     })
// }

// fn generate_mipmaps(
//     vk_context: &VkContext,
//     command_pool: vk::CommandPool,
//     transfer_queue: vk::Queue,
//     image: vk::Image,
//     extent: vk::Extent2D,
//     format: vk::Format,
//     mip_levels: u32,
// ) {
//     let format_properties = unsafe {
//         vk_context
//             .instance()
//             .get_physical_device_format_properties(vk_context.physical_device(), format)
//     };
//     if !format_properties
//         .optimal_tiling_features
//         .contains(vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR)
//     {
//         panic!("Linear blitting is not supported for format {:?}.", format)
//     }

//     Self::execute_one_time_commands(
//         vk_context.device(),
//         command_pool,
//         transfer_queue,
//         |buffer| {
//             let mut barrier = vk::ImageMemoryBarrier::builder()
//                 .image(image)
//                 .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
//                 .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
//                 .subresource_range(vk::ImageSubresourceRange {
//                     aspect_mask: vk::ImageAspectFlags::COLOR,
//                     base_array_layer: 0,
//                     layer_count: 1,
//                     level_count: 1,
//                     ..Default::default()
//                 })
//                 .build();

//             let mut mip_width = extent.width as i32;
//             let mut mip_height = extent.height as i32;
//             for level in 1..mip_levels {
//                 let next_mip_width = if mip_width > 1 {
//                     mip_width / 2
//                 } else {
//                     mip_width
//                 };
//                 let next_mip_height = if mip_height > 1 {
//                     mip_height / 2
//                 } else {
//                     mip_height
//                 };

//                 barrier.subresource_range.base_mip_level = level - 1;
//                 barrier.old_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
//                 barrier.new_layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
//                 barrier.src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
//                 barrier.dst_access_mask = vk::AccessFlags::TRANSFER_READ;
//                 let barriers = [barrier];

//                 unsafe {
//                     vk_context.device().cmd_pipeline_barrier(
//                         buffer,
//                         vk::PipelineStageFlags::TRANSFER,
//                         vk::PipelineStageFlags::TRANSFER,
//                         vk::DependencyFlags::empty(),
//                         &[],
//                         &[],
//                         &barriers,
//                     )
//                 };

//                 let blit = vk::ImageBlit::builder()
//                     .src_offsets([
//                         vk::Offset3D { x: 0, y: 0, z: 0 },
//                         vk::Offset3D {
//                             x: mip_width,
//                             y: mip_height,
//                             z: 1,
//                         },
//                     ])
//                     .src_subresource(vk::ImageSubresourceLayers {
//                         aspect_mask: vk::ImageAspectFlags::COLOR,
//                         mip_level: level - 1,
//                         base_array_layer: 0,
//                         layer_count: 1,
//                     })
//                     .dst_offsets([
//                         vk::Offset3D { x: 0, y: 0, z: 0 },
//                         vk::Offset3D {
//                             x: next_mip_width,
//                             y: next_mip_height,
//                             z: 1,
//                         },
//                     ])
//                     .dst_subresource(vk::ImageSubresourceLayers {
//                         aspect_mask: vk::ImageAspectFlags::COLOR,
//                         mip_level: level,
//                         base_array_layer: 0,
//                         layer_count: 1,
//                     })
//                     .build();
//                 let blits = [blit];

//                 unsafe {
//                     vk_context.device().cmd_blit_image(
//                         buffer,
//                         image,
//                         vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
//                         image,
//                         vk::ImageLayout::TRANSFER_DST_OPTIMAL,
//                         &blits,
//                         vk::Filter::LINEAR,
//                     )
//                 };

//                 barrier.old_layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
//                 barrier.new_layout = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
//                 barrier.src_access_mask = vk::AccessFlags::TRANSFER_READ;
//                 barrier.dst_access_mask = vk::AccessFlags::SHADER_READ;
//                 let barriers = [barrier];

//                 unsafe {
//                     vk_context.device().cmd_pipeline_barrier(
//                         buffer,
//                         vk::PipelineStageFlags::TRANSFER,
//                         vk::PipelineStageFlags::FRAGMENT_SHADER,
//                         vk::DependencyFlags::empty(),
//                         &[],
//                         &[],
//                         &barriers,
//                     )
//                 };

//                 mip_width = next_mip_width;
//                 mip_height = next_mip_height;
//             }

//             barrier.subresource_range.base_mip_level = mip_levels - 1;
//             barrier.old_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
//             barrier.new_layout = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
//             barrier.src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
//             barrier.dst_access_mask = vk::AccessFlags::SHADER_READ;
//             let barriers = [barrier];

//             unsafe {
//                 vk_context.device().cmd_pipeline_barrier(
//                     buffer,
//                     vk::PipelineStageFlags::TRANSFER,
//                     vk::PipelineStageFlags::FRAGMENT_SHADER,
//                     vk::DependencyFlags::empty(),
//                     &[],
//                     &[],
//                     &barriers,
//                 )
//             };
//         },
//     );
// }
