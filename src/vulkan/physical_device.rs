use std::ops::Deref;

use ash::{version::InstanceV1_0, vk};

use super::utils::coerce_string;

pub struct VkPhysicalDevice(vk::PhysicalDevice);

impl VkPhysicalDevice {
    pub fn new(instance: &ash::Instance) -> VkPhysicalDevice {
        let physical_devices = enumerate_devices(instance);
        log::info!(
            "{} devices found with vulkan support",
            physical_devices.len()
        );

        let mut best_physical_device = vk::PhysicalDevice::null();
        let mut best_score = DeviceScore::unsuitable();
        for &physical_device in physical_devices.iter() {
            let score = rate_device_suitability(instance, physical_device);
            if score.is_suitable() && score.is_better_than(&best_score) {
                best_physical_device = physical_device;
                best_score = score;
            }
        }

        if !best_score.is_suitable() {
            panic!("Failed to find a suitable GPU!");
        }
        VkPhysicalDevice(best_physical_device)
    }
}

impl Deref for VkPhysicalDevice {
    type Target = vk::PhysicalDevice;

    fn deref(&self) -> &Self::Target {
        &self.0
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

fn rate_device_suitability(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> DeviceScore {
    let mut score = DeviceScore::default();
    let properties = unsafe { instance.get_physical_device_properties(physical_device) };
    let features = unsafe { instance.get_physical_device_features(physical_device) };

    log::info!(
        "Device Name: {}, id: {}, type: {}",
        get_device_name(&properties),
        properties.device_id,
        get_device_type(&properties)
    );
    log::info!("API Version: {}", get_api_version(&properties));

    if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
        score = score.add(100);
    }

    let device_queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    log::info!("Queue Family Count: {}", device_queue_families.len());

    let mut queue_family_indices = QueueFamilyIndices {
        graphics_family: None,
    };
    let mut index = 0;
    for queue_family in device_queue_families.iter() {
        log::info!(
            "Family: {}, queue count: {}",
            index,
            queue_family.queue_count
        );
        log::info!(
            "Supported commands: {}",
            get_queue_supported_commands(queue_family)
        );

        if queue_family.queue_count > 0
            && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        {
            queue_family_indices.graphics_family = Some(index);
        }

        index += 1;
    }

    log::info!("Geometry Shader support: {}", features.geometry_shader == 1);

    if queue_family_indices.is_complete() {
        score
    } else {
        DeviceScore::unsuitable()
    }
}

fn get_device_name(properties: &vk::PhysicalDeviceProperties) -> String {
    coerce_string(&properties.device_name)
}

fn get_device_type(properties: &vk::PhysicalDeviceProperties) -> &'static str {
    match properties.device_type {
        vk::PhysicalDeviceType::CPU => "Cpu",
        vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
        vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
        vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
        vk::PhysicalDeviceType::OTHER => "Unknown",
        _ => panic!(),
    }
}

fn get_api_version(properties: &vk::PhysicalDeviceProperties) -> String {
    let major = vk::version_major(properties.api_version);
    let minor = vk::version_minor(properties.api_version);
    let patch = vk::version_patch(properties.api_version);
    format!("{}.{}.{}", major, minor, patch)
}

fn get_queue_supported_commands(queue_family: &vk::QueueFamilyProperties) -> String {
    let mut result: Vec<String> = Vec::new();

    if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
        result.push(String::from("Graphics"));
    }
    if queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE) {
        result.push(String::from("Compute"));
    }
    if queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER) {
        result.push(String::from("Transfer"));
    }
    if queue_family
        .queue_flags
        .contains(vk::QueueFlags::SPARSE_BINDING)
    {
        result.push(String::from("SparseBinding"));
    }

    result.join(", ")
}
