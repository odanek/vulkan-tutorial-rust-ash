use winit::window::Window;

use ash::{extensions::ext::DebugUtils, vk};

use std::ffi::{CStr, CString};

use super::{debug::*, utils};

pub struct VkInstance {
    pub handle: ash::Instance,
}

impl VkInstance {
    pub fn new(window: &Window, entry: &ash::Entry, validation: bool) -> VkInstance {
        let app_name = CString::new("Vulkan Application").unwrap();
        let engine_name = CString::new("No Engine").unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name.as_c_str())
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(engine_name.as_c_str())
            .engine_version(vk::make_api_version(0, 0, 0, 1))
            .api_version(vk::make_api_version(0, 1, 2, 0));

        let extensions = enumerate_extensions(window, validation);
        let extension_names = utils::as_raw_handles(&extensions);

        let mut instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        if validation {
            check_validation_layer_support(&entry);
            let validation_layers = get_validation_layers();
            let validation_layer_names = utils::as_raw_handles(&validation_layers);
            let mut debug_utils_create_info = populate_debug_messenger_create_info();
            instance_create_info = instance_create_info
                .enabled_layer_names(&validation_layer_names)
                .push_next(&mut debug_utils_create_info);

            build_instance(entry, instance_create_info)
        } else {
            build_instance(entry, instance_create_info)
        }
    }
}

impl Drop for VkInstance {
    fn drop(&mut self) {
        log::debug!("Dropping instance");
        unsafe {
            self.handle.destroy_instance(None);
        }
    }
}

fn build_instance(entry: &ash::Entry, info: vk::InstanceCreateInfoBuilder) -> VkInstance {
    let handle = unsafe {
        entry
            .create_instance(&info, None)
            .expect("Unable t ocreate Vulkan instance")
    };
    VkInstance { handle }
}

fn enumerate_extensions(window: &Window, validation: bool) -> Vec<&'static CStr> {
    let window_extensions = ash_window::enumerate_required_extensions(window)
        .expect("Unable to enumerate rrequired window extensions");
    let mut extensions = window_extensions;

    if validation {
        extensions.push(DebugUtils::name());
    }

    extensions
}
