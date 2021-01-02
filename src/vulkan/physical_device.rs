use ash::{version::InstanceV1_0, vk};

use super::utils::coerce_string;

pub struct VkPhysicalDevice(vk::PhysicalDevice);

impl VkPhysicalDevice {
    pub fn new(instance: &ash::Instance) -> VkPhysicalDevice {
        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate Physical Devices!")
        };

        println!(
            "{} devices (GPU) found with vulkan support.",
            physical_devices.len()
        );

        for &physical_device in physical_devices.iter() {
            if is_physical_device_suitable(instance, physical_device) {
                return VkPhysicalDevice(physical_device);
            }
        }

        panic!("Failed to find a suitable GPU!");
    }
}

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}

// TODO: Rate and choose best physical device
fn is_physical_device_suitable(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
    let device_features = unsafe { instance.get_physical_device_features(physical_device) };

    let device_type = match device_properties.device_type {
        vk::PhysicalDeviceType::CPU => "Cpu",
        vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
        vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
        vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
        vk::PhysicalDeviceType::OTHER => "Unknown",
        _ => panic!(),
    };

    let device_name = coerce_string(&device_properties.device_name);
    println!(
        "\tDevice Name: {}, id: {}, type: {}",
        device_name, device_properties.device_id, device_type
    );

    let major_version = vk::version_major(device_properties.api_version);
    let minor_version = vk::version_minor(device_properties.api_version);
    let patch_version = vk::version_patch(device_properties.api_version);

    println!(
        "\tAPI Version: {}.{}.{}",
        major_version, minor_version, patch_version
    );

    let device_queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    println!("\tSupport Queue Family: {}", device_queue_families.len());
    println!("\t\tQueue Count | Graphics, Compute, Transfer, Sparse Binding");
    for queue_family in device_queue_families.iter() {
        let is_graphics_support = if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            "support"
        } else {
            "unsupport"
        };
        let is_compute_support = if queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE) {
            "support"
        } else {
            "unsupport"
        };
        let is_transfer_support = if queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER) {
            "support"
        } else {
            "unsupport"
        };
        let is_sparse_support = if queue_family
            .queue_flags
            .contains(vk::QueueFlags::SPARSE_BINDING)
        {
            "support"
        } else {
            "unsupport"
        };

        println!(
            "\t\t{}\t    | {},  {},  {},  {}",
            queue_family.queue_count,
            is_graphics_support,
            is_compute_support,
            is_transfer_support,
            is_sparse_support
        );
    }

    // there are plenty of features
    println!(
        "\tGeometry Shader support: {}",
        if device_features.geometry_shader == 1 {
            "Support"
        } else {
            "Unsupport"
        }
    );

    let indices = find_queue_family(instance, physical_device);

    return indices.is_complete();
}

fn find_queue_family(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> QueueFamilyIndices {
    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    let mut index = 0;
    for queue_family in queue_families.iter() {
        if queue_family.queue_count > 0
            && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        {
            return QueueFamilyIndices {
                graphics_family: Some(index),
            };
        }
        index += 1;
    }

    QueueFamilyIndices {
        graphics_family: None,
    }
}
