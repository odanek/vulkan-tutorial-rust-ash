use std::ops::Deref;

use ash::{version::InstanceV1_0, vk};

use super::{
    queue_family::VkQueueFamily, surface::VkSurface, utils::coerce_string, version::VkVersion,
};

#[derive(Debug)]
pub enum DeviceType {
    Cpu,
    IntegratedGpu,
    DiscreteGpu,
    VirtualGpu,
    Other,
}

pub struct VkPhysicalDevice {
    pub handle: vk::PhysicalDevice,
    pub name: String,
    pub kind: DeviceType,
    pub api_version: VkVersion,
    pub queue_families: Vec<VkQueueFamily>,
}

impl VkPhysicalDevice {
    pub fn new(instance: &ash::Instance, surface: &VkSurface) -> VkPhysicalDevice {
        let physical_devices = enumerate_devices(instance);
        log::info!(
            "{} device(s) found with vulkan support",
            physical_devices.len()
        );

        let mut best_physical_device: Option<VkPhysicalDevice> = None;
        let mut best_score = -1;
        for &handle in physical_devices.iter() {
            let physical_device = create_physical_device(instance, handle);
            describe_device(&physical_device);

            let score = rate_device_suitability(&physical_device, surface);
            if score > best_score {
                best_physical_device = Some(physical_device);
                best_score = score;
            }
        }

        if let Some(physical_device) = best_physical_device {
            return physical_device;
        }
        panic!("Failed to find a suitable GPU!");
    }
}

impl Deref for VkPhysicalDevice {
    type Target = vk::PhysicalDevice;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

fn enumerate_devices(instance: &ash::Instance) -> Vec<vk::PhysicalDevice> {
    unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate Physical Devices!")
    }
}

fn create_physical_device(
    instance: &ash::Instance,
    handle: vk::PhysicalDevice,
) -> VkPhysicalDevice {
    let properties = unsafe { instance.get_physical_device_properties(handle) };
    let kind = get_device_type(&properties);
    let name = coerce_string(&properties.device_name);
    let api_version = VkVersion::parse(properties.api_version);
    let queue_families = get_queue_families(instance, handle);
    VkPhysicalDevice {
        handle,
        name,
        kind,
        api_version,
        queue_families,
    }
}

fn describe_device(device: &VkPhysicalDevice) {
    log::info!(
        "Device Name: {}, type: {:?}, api: {}",
        device.name,
        device.kind,
        device.api_version
    );

    let queue_families = &device.queue_families;
    log::info!("Queue Family Count: {}", queue_families.len());
    let mut index = 0;
    for queue_family in queue_families.iter() {
        log::info!(
            "Family: {}, queue count: {}",
            index,
            queue_family.queue_count
        );
        log::info!("Supported commands: {:?}", queue_family.flags);
        index += 1;
    }

    // let features = unsafe { instance.get_physical_device_features(physical_device.handle) };
    // log::info!("Geometry Shader support: {}", features.geometry_shader == 1);
}

fn rate_device_suitability(device: &VkPhysicalDevice, surface: &VkSurface) -> i32 {
    let mut score = 0i32;
    match device.kind {
        DeviceType::DiscreteGpu => score += 100,
        DeviceType::IntegratedGpu => score += 50,
        _ => (),
    }

    let queue_families = &device.queue_families;
    if !has_queue_family(queue_families, |family| {
        family.flags.contains(vk::QueueFlags::GRAPHICS)
    }) {
        return -1;
    }
    if !has_queue_family(queue_families, |family| {
        surface.physical_device_queue_support(device, family.index)
    }) {
        return -1;
    }

    score
}

fn get_device_type(properties: &vk::PhysicalDeviceProperties) -> DeviceType {
    match properties.device_type {
        vk::PhysicalDeviceType::CPU => DeviceType::Cpu,
        vk::PhysicalDeviceType::INTEGRATED_GPU => DeviceType::IntegratedGpu,
        vk::PhysicalDeviceType::DISCRETE_GPU => DeviceType::DiscreteGpu,
        vk::PhysicalDeviceType::VIRTUAL_GPU => DeviceType::VirtualGpu,
        _ => DeviceType::Other,
    }
}

fn get_queue_families(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> Vec<VkQueueFamily> {
    let families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    let mut index = 0u32;
    let mut result = Vec::new();
    for definition in families.iter() {
        result.push(VkQueueFamily::new(index, definition));
        index += 1;
    }

    result
}

fn has_queue_family(
    families: &Vec<VkQueueFamily>,
    predicate: impl Fn(&VkQueueFamily) -> bool,
) -> bool {
    families
        .iter()
        .any(|family| family.queue_count > 0 && predicate(family))
}
