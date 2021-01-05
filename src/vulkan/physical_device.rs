use std::ops::Deref;

use ash::{version::InstanceV1_0, vk};

use super::{utils::coerce_string, version::VkVersion};

#[derive(Debug)]
pub enum DeviceType {
    Cpu,
    IntegratedGpu,
    DiscreteGpu,
    VirtualGpu,
    Other
}

pub struct QueueFamily {
    queue_count: u32,
    graphics: bool,
    compute: bool, 
    transfer: bool,
    sparse_binding: bool,
}

pub struct VkPhysicalDevice {
    handle: vk::PhysicalDevice,
    pub name: String,
    pub kind: DeviceType,
    pub api_version: VkVersion,
    pub queue_families: Vec<QueueFamily>,
}

impl VkPhysicalDevice {
    pub fn new(instance: &ash::Instance) -> VkPhysicalDevice {
        let physical_devices = enumerate_devices(instance);
        log::info!(
            "{} device(s) found with vulkan support",
            physical_devices.len()
        );

        let mut best_physical_device: Option<VkPhysicalDevice> = None;
        let mut best_score = DeviceScore::unsuitable();
        for &handle in physical_devices.iter() {
            let physical_device = create_physical_device(instance, handle);
            describe_device(&physical_device);

            let score = rate_device_suitability(&physical_device);
            if score.is_suitable() && score.is_better_than(&best_score) {
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

struct DeviceScore(i32);

impl DeviceScore {
    fn unsuitable() -> DeviceScore {
        DeviceScore(-1)
    }

    fn add(&self, score: i32) -> DeviceScore {
        DeviceScore(self.0 + score)
    }

    fn is_suitable(&self) -> bool {
        self.0 >= -1
    }

    fn is_better_than(&self, other: &DeviceScore) -> bool {
        self.0 > other.0
    }
}

impl Default for DeviceScore {
    fn default() -> Self {
        DeviceScore(0)
    }
}

fn enumerate_devices(instance: &ash::Instance) -> Vec<vk::PhysicalDevice> {
    unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate Physical Devices!")
    }
}

fn create_physical_device(instance: &ash::Instance, handle: vk::PhysicalDevice) -> VkPhysicalDevice {
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
        queue_families
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
        log::info!(
            "Supported commands: {}",
            get_queue_supported_commands(queue_family)
        );
        index += 1;
    }

    // let features = unsafe { instance.get_physical_device_features(physical_device.handle) };
    // log::info!("Geometry Shader support: {}", features.geometry_shader == 1);
}

fn rate_device_suitability(device: &VkPhysicalDevice) -> DeviceScore {
    let mut score = DeviceScore::default();    
    match device.kind {
        DeviceType::DiscreteGpu => score = score.add(100),
        DeviceType::IntegratedGpu => score = score.add(50),
        _ => ()
    }

    let queue_families = &device.queue_families;
    let has_graphics_family = queue_families.iter().any(|family| family.queue_count > 0 && family.graphics);
    if !has_graphics_family {
        return DeviceScore::unsuitable()
    }
    
    score
}

fn get_device_type(properties: &vk::PhysicalDeviceProperties) -> DeviceType {
    match properties.device_type {
        vk::PhysicalDeviceType::CPU => DeviceType::Cpu,
        vk::PhysicalDeviceType::INTEGRATED_GPU => DeviceType::IntegratedGpu,
        vk::PhysicalDeviceType::DISCRETE_GPU => DeviceType::DiscreteGpu,
        vk::PhysicalDeviceType::VIRTUAL_GPU => DeviceType::VirtualGpu,
        _ => DeviceType::Other
    }
}

fn get_queue_supported_commands(queue_family: &QueueFamily) -> String {
    let mut result: Vec<String> = Vec::new();

    if queue_family.graphics {
        result.push("Graphics".to_owned());
    }
    if queue_family.compute {
        result.push("Compute".to_owned());
    }
    if queue_family.transfer {
        result.push("Transfer".to_owned());
    }
    if queue_family.sparse_binding {
        result.push("SparseBinding".to_owned());
    }

    result.join(", ")
}

fn get_queue_families(instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> Vec<QueueFamily> {
    unsafe { 
        instance
        .get_physical_device_queue_family_properties(physical_device)
        .iter()
        .map(|definition| describe_queue_family(definition))
        .collect::<Vec<_>>()
    }
}

fn describe_queue_family(queue_family: &vk::QueueFamilyProperties) -> QueueFamily {
    QueueFamily {
        queue_count: queue_family.queue_count,
        graphics: queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS),
        compute: queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE),
        transfer: queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER),
        sparse_binding: queue_family.queue_flags.contains(vk::QueueFlags::SPARSE_BINDING),
    }
}
