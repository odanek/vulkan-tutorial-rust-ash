mod debug;

use winit::window::Window;

use ash::{extensions::ext::DebugUtils, version::EntryV1_0, vk, Entry};
use ash_window;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use debug::*;

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
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

impl Context {
    pub fn new(window: &Window, settings: &Settings) -> Self {
        let entry = Entry::new().expect("Failed to create Vulkan entry.");
        let instance = Self::create_instance(window, settings, &entry);
        let (debug_utils_loader, debug_messenger) =
            setup_debug_utils(settings.validation, &entry, &instance);

        Context {
            entry,
            instance,
            debug_utils_loader,
            debug_messenger,
        }
    }

    fn create_instance(window: &Window, settings: &Settings, entry: &ash::Entry) -> ash::Instance {
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

        let mut instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        if settings.validation {
            check_validation_layer_support(&entry);
            let validation_layers = get_validation_layers();
            let mut debug_utils_create_info = populate_debug_messenger_create_info();
            instance_create_info = instance_create_info
                .enabled_layer_names(&validation_layers)
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

    fn get_extensions_names(extensions: &Vec<&'static CStr>) -> Vec<*const c_char> {
        extensions
            .iter()
            .map(|ext| ext.as_ptr())
            .collect::<Vec<_>>()
    }
}

// impl Drop for VulkanApp {
//     fn drop(&mut self) {
//         unsafe {
//             if VALIDATION.is_enable {
//                 self.debug_utils_loader
//                     .destroy_debug_utils_messenger(self.debug_merssager, None);
//             }
//             self.instance.destroy_instance(None);
//         }
//     }
// }

// TODO: Vyzkouset ze loguje chyby
