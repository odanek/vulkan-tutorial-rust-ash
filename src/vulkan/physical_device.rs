use std::{ffi::CStr, sync::Arc};

use ash::{extensions::khr::Swapchain, version::InstanceV1_0, vk};

use super::{instance::VkInstance, queue_family::VkQueueFamily, surface::VkSurface, utils::coerce_string, version::VkVersion};

#[derive(Debug)]
pub enum DeviceType {
    Cpu,
    IntegratedGpu,
    DiscreteGpu,
    VirtualGpu,
    Other,
}

pub struct VkPhysicalDevice {
    pub instance: Arc<VkInstance>,
    pub handle: vk::PhysicalDevice,
    pub name: String,
    pub kind: DeviceType,
    pub api_version: VkVersion,
    pub queue_families: Vec<VkQueueFamily>,
}

impl VkPhysicalDevice {
    pub fn new(instance: &Arc<VkInstance>, surface: &VkSurface) -> VkPhysicalDevice {
        let physical_devices = enumerate_devices(&instance.handle);
        log::info!(
            "{} device(s) found with vulkan support",
            physical_devices.len()
        );

        let extensions = Self::get_required_device_extensions();
        let mut best_physical_device: Option<VkPhysicalDevice> = None;
        let mut best_score = -1;
        for &handle in physical_devices.iter() {
            let physical_device = create_physical_device(instance, handle);
            describe_device(&physical_device);

            let score = rate_device_suitability(&physical_device, surface, &extensions);
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

    pub fn get_mem_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        unsafe {
            self.instance.handle.get_physical_device_memory_properties(self.handle)
        }
    }

    pub fn get_format_properties(&self, format: vk::Format) -> vk::FormatProperties {
        unsafe {
            self.instance.handle.get_physical_device_format_properties(self.handle, format)
        }
    }

    pub fn get_device_features(&self) -> vk::PhysicalDeviceFeatures {
        unsafe {
            self.instance.handle.get_physical_device_features(self.handle)
        }
    }

    pub fn get_device_properties(&self) -> vk::PhysicalDeviceProperties {
        unsafe {
            self.instance.handle.get_physical_device_properties(self.handle)
        }
    }

    pub fn get_required_device_extensions() -> Vec<&'static CStr> {
        vec![Swapchain::name()]
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
    instance: &Arc<VkInstance>,
    handle: vk::PhysicalDevice,
) -> VkPhysicalDevice {
    let properties = unsafe { instance.handle.get_physical_device_properties(handle) };
    let kind = get_device_type(&properties);
    let name = coerce_string(&properties.device_name);
    let api_version = VkVersion::parse(properties.api_version);
    let queue_families = get_queue_families(&instance.handle, handle);

    VkPhysicalDevice {
        instance: Arc::clone(instance),
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

    for (index, queue_family) in queue_families.iter().enumerate() {
        log::info!(
            "Family: {}, queue count: {}",
            index,
            queue_family.queue_count
        );
        log::info!("Supported commands: {:?}", queue_family.flags);
    }

    let features = device.get_device_features();
    log::info!("Geometry Shader support: {}", features.geometry_shader == 1);

    let properties = device.get_device_properties();
    log::info!("Max sampler anisotropy: {}", properties.limits.max_sampler_anisotropy);
}

fn rate_device_suitability(
    device: &VkPhysicalDevice,
    surface: &VkSurface,
    extensions: &[&CStr],
) -> i32 {
    let mut score = 0i32;

    let queue_families = &device.queue_families;
    let has_graphics_family = has_queue_family(queue_families, |family| {
        family.flags.contains(vk::QueueFlags::GRAPHICS)
    });
    if !has_graphics_family {
        return -1;
    }
    let has_surface_support_family = has_queue_family(queue_families, |family| {
        surface.physical_device_queue_support(device, family.index)
    });
    if !has_surface_support_family {
        return -1;
    }

    let surface_caps = surface.get_physical_device_surface_capabilities(device);
    if surface_caps.formats.is_empty() || surface_caps.present_modes.is_empty() {
        return -1;
    }

    let features = device.get_device_features();
    if features.sampler_anisotropy == vk::FALSE {
        return -1;
    }

    if !check_device_extension_support(device, extensions) {
        return -1;
    }

    match device.kind {
        DeviceType::DiscreteGpu => score += 100,
        DeviceType::IntegratedGpu => score += 50,
        _ => (),
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

    let mut result = Vec::new();
    for (index, definition) in families.iter().enumerate() {
        result.push(VkQueueFamily::new(index as u32, definition));
    }

    result
}

fn has_queue_family(
    families: &[VkQueueFamily],
    predicate: impl Fn(&VkQueueFamily) -> bool,
) -> bool {
    families
        .iter()
        .any(|family| family.queue_count > 0 && predicate(family))
}

fn check_device_extension_support(   
    device: &VkPhysicalDevice,
    extensions: &[&CStr],
) -> bool {
    let instance = &device.instance.handle;
    let extension_props = unsafe {
        instance
            .enumerate_device_extension_properties(device.handle)
            .expect("Unable to query device extensions")
    };

    for required in extensions.iter() {
        let found = extension_props.iter().any(|ext| {
            let name = unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) };
            required == &name
        });

        if !found {
            return false;
        }
    }

    true
}
