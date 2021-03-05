use std::ffi::{c_void, CStr, CString};

use ash::{extensions::ext::DebugUtils, version::EntryV1_0, vk, Entry};

use super::instance::VkInstance;

const REQUIRED_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

pub struct VkValidation {
    extension: DebugUtils,
    messenger: vk::DebugUtilsMessengerEXT,
}

impl VkValidation {
    pub fn new(entry: &ash::Entry, instance: &VkInstance) -> VkValidation {
        let extension = DebugUtils::new(entry, &instance.handle);
        let messanger_ci = populate_debug_messenger_create_info();

        let messenger = unsafe {
            extension
                .create_debug_utils_messenger(&messanger_ci, None)
                .expect("Unable to create debug utils messenger")
        };

        VkValidation {
            extension,
            messenger,
        }
    }
}

impl Drop for VkValidation {
    fn drop(&mut self) {
        log::debug!("Dropping validation");
        unsafe {
            self.extension
                .destroy_debug_utils_messenger(self.messenger, None);
        }
    }
}

pub fn get_validation_layers() -> Vec<CString> {
    REQUIRED_LAYERS
        .iter()
        .map(|name| CString::new(*name).unwrap())
        .collect()
}

pub fn check_validation_layer_support(entry: &Entry) {
    let available_layers = entry
        .enumerate_instance_layer_properties()
        .expect("Failed to enumerate Instance Layers Properties");

    for required in &REQUIRED_LAYERS {
        let found = available_layers.iter().any(|layer| {
            let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) };
            let name = name.to_str().expect("Failed to get layer name pointer");
            required == &name
        });

        if !found {
            panic!("Validation layer not supported: {}", required);
        }
    }
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let severity = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
        _ => "[Unknown]",
    };
    let types = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = CStr::from_ptr((*p_callback_data).p_message);
    log::debug!("[Vulkan] {}{}{:?}", severity, types, message);

    vk::FALSE
}

pub fn populate_debug_messenger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        )
        .pfn_user_callback(Some(vulkan_debug_utils_callback))
        .build()
}
