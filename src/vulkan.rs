mod debug;
mod extensions;

use winit::window::Window;

use ash::{extensions::ext::DebugUtils, version::{EntryV1_0, InstanceV1_0}, vk, Entry};
use ash_window;

use std::ffi::{CStr, CString};

use debug::*;
use extensions::*;

pub struct Settings {
    validation: bool,
}

impl Settings {
    pub fn new(validation: bool) -> Self {
        Self { validation }
    }
}

pub struct Context {
    entry: ash::Entry,
    instance: ash::Instance,
    validation: Option<ValidationContext>
}

impl Context {
    pub fn new(window: &Window, settings: &Settings) -> Self {
        let entry = Entry::new().expect("Failed to create Vulkan entry.");
        let instance = Self::create_instance(window, settings, &entry);
        let validation = if settings.validation { Some(setup_debug_utils(&entry, &instance)) } else { None };

        Context {
            entry,
            instance,
            validation
        }
    }

    pub fn wait_device_idle(&self) {
        // TODO
        // unsafe {
            // self.device
            //     .device_wait_idle()
            //     .expect("Failed to wait device idle!")
        // };
    }

    fn create_instance(window: &Window, settings: &Settings, entry: &ash::Entry) -> ash::Instance {
        let app_name = CString::new("Vulkan Application").unwrap();
        let engine_name = CString::new("No Engine").unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name.as_c_str())
            .application_version(vk::make_version(1, 0, 0))
            .engine_name(engine_name.as_c_str())
            .engine_version(vk::make_version(0, 0, 1))
            .api_version(vk::make_version(1, 2, 0));            

        let extensions = Self::enumerate_extensions(window, settings);
        let extension_names = extension_names_from_cstr(&extensions);

        let mut instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        if settings.validation {
            check_validation_layer_support(&entry);
            let validation_layers = get_validation_layers();
            let validation_layer_names = extension_names_from_cstring(&validation_layers);
            let mut debug_utils_create_info = populate_debug_messenger_create_info();
            instance_create_info = instance_create_info
                .enabled_layer_names(&validation_layer_names)
                .push_next(&mut debug_utils_create_info);
            unsafe {
                entry
                    .create_instance(&instance_create_info, None)
                    .expect("Unable t ocreate Vulkan instance")
            }
        } else {
            unsafe {
                entry
                    .create_instance(&instance_create_info, None)
                    .expect("Unable t ocreate Vulkan instance")
            }
        }
    }

    fn enumerate_extensions(window: &Window, settings: &Settings) -> Vec<&'static CStr> {
        let window_extensions = ash_window::enumerate_required_extensions(window)
            .expect("Unable to enumerate rrequired window extensions");
        let mut extensions = window_extensions;

        if settings.validation {
            extensions.push(DebugUtils::name());
        }

        extensions
    }    
}

impl Drop for Context {
    fn drop(&mut self) {
        println!("Dropping context");
        // TODO: Improve this - Drop on ValidationContext and Drop on Instance?
        let validation = self.validation.take();
        if let Some(validation_context) = validation {
            validation_context.destroy();
        }
        unsafe {            
            self.instance.destroy_instance(None);            
        }
    }
}