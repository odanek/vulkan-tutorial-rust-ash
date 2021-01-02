mod debug;
mod extensions;

use winit::window::Window;

use ash::{extensions::ext::DebugUtils, version::{EntryV1_0, InstanceV1_0}, vk, Entry};
use ash_window;

use std::ffi::{CStr, CString};

use debug::*;
use extensions::*;

pub struct VkSettings {
    pub validation: bool,
}

struct VkInstance(ash::Instance);

impl Drop for VkInstance {
    fn drop(&mut self) {
        println!("Dropping instance");
        unsafe {            
            self.0.destroy_instance(None);            
        }
    }
}

impl VkInstance {
    pub fn new(window: &Window, settings: &VkSettings, entry: &ash::Entry) -> VkInstance {
        let app_name = CString::new("Vulkan Application").unwrap();
        let engine_name = CString::new("No Engine").unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name.as_c_str())
            .application_version(vk::make_version(1, 0, 0))
            .engine_name(engine_name.as_c_str())
            .engine_version(vk::make_version(0, 0, 1))
            .api_version(vk::make_version(1, 2, 0));            

        let extensions = VkInstance::enumerate_extensions(window, settings);
        let extension_names = coerce_extension_names(&extensions);

        let mut instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        if settings.validation {
            check_validation_layer_support(&entry);
            let validation_layers = get_validation_layers();
            let validation_layer_names = coerce_extension_names(&validation_layers);
            let mut debug_utils_create_info = populate_debug_messenger_create_info();
            instance_create_info = instance_create_info
                .enabled_layer_names(&validation_layer_names)
                .push_next(&mut debug_utils_create_info);
            VkInstance::build_instance(entry, instance_create_info)
        } else {
            VkInstance::build_instance(entry, instance_create_info)
        }
    }

    fn build_instance(entry: &ash::Entry, info: vk::InstanceCreateInfoBuilder) -> VkInstance {
        let instance = unsafe {
            entry
                .create_instance(&info, None)
                .expect("Unable t ocreate Vulkan instance")
        };
        VkInstance(instance)
    }

    fn enumerate_extensions(window: &Window, settings: &VkSettings) -> Vec<&'static CStr> {
        let window_extensions = ash_window::enumerate_required_extensions(window)
            .expect("Unable to enumerate rrequired window extensions");
        let mut extensions = window_extensions;

        if settings.validation {
            extensions.push(DebugUtils::name());
        }

        extensions
    }   
}


pub struct VkContext {    
    validation: Option<VkValidation>,
    instance: VkInstance,
    entry: ash::Entry,
}

impl VkContext {
    pub fn new(window: &Window, settings: &VkSettings) -> VkContext {
        let entry = Entry::new().expect("Failed to create Vulkan entry.");
        let instance = VkInstance::new(window, settings, &entry); // Move to instance.rs
        let validation = if settings.validation { Some(VkValidation::new(&entry, &instance.0)) } else { None };

        VkContext {
            entry,
            instance,
            validation
        }
    }

    pub fn wait_device_idle(&self) {
        // TODO method on VkDevice
        // unsafe {
            // self.device
            //     .device_wait_idle()
            //     .expect("Failed to wait device idle!")
        // };
    } 
}