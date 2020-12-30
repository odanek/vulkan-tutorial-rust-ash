use winit::window::Window;

use ash::{    
    vk, 
    Entry,
    version::{
        EntryV1_0,
    },
    extensions::{
        ext::DebugReport,
    }
};
use ash_window;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub struct Settings {
    validation: bool
}

impl Settings {
    pub fn new(validation: bool) -> Self {
        Self {
            validation
        }
    }
}

pub struct Context {
    entry: ash::Entry,
    instance: ash::Instance,
}

impl Context {
    pub fn new(window: &Window, settings: &Settings) -> Self {
        let entry = Entry::new().expect("Failed to create Vulkan entry.");
        let app_name = CString::new("Vulkan Application").unwrap();
        let engine_name = CString::new("No Engine").unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name.as_c_str())
            .application_version(vk::make_version(1, 0, 0))
            .engine_name(engine_name.as_c_str())
            .engine_version(vk::make_version(0, 0, 1))
            .api_version(vk::make_version(1, 2, 0))
            .build();

        let extensions = Self::enumerate_extensions(window, settings);
        let extension_names = Self::get_extensions_names(&extensions);

        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        // if ENABLE_VALIDATION_LAYERS {
        //     check_validation_layer_support(&entry);
        //     instance_create_info = instance_create_info.enabled_layer_names(&layer_names_ptrs);
        // }

        let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };

        Context { entry, instance }
    }

    fn enumerate_extensions(window: &Window, settings: &Settings) -> Vec<&'static CStr> {
        let window_extensions = ash_window::enumerate_required_extensions(window).unwrap();
        let mut extensions = window_extensions;

        if settings.validation {
            extensions.push(DebugReport::name());
        }
        // let (_layer_names, layer_names_ptrs) = get_layer_names_and_pointers();

        extensions
    }

    fn get_extensions_names(extensions: &Vec<&'static CStr>) -> Vec<*const c_char> {
        extensions
            .iter()
            .map(|ext| ext.as_ptr())
            .collect::<Vec<_>>()
    }
}
