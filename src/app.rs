use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use ash::version::EntryV1_0;
use ash::{vk, Entry};
use ash_window;
use std::ffi::CString;

struct VulkanContext {
    entry: ash::Entry,
    instance: ash::Instance,
}

pub struct HelloTriangleApp {
    event_loop: EventLoop<()>,
    window: Window,
    vulkan_context: VulkanContext,
}

impl HelloTriangleApp {
    pub fn new(window_size: PhysicalSize<u32>) -> HelloTriangleApp {
        let (event_loop, window) = HelloTriangleApp::init_window(&window_size);
        let vulkan_context = HelloTriangleApp::init_vulkan(&window);

        return HelloTriangleApp {
            event_loop,
            window,
            vulkan_context,
        };
    }

    pub fn run(self) {
        let self_window_id = self.window.id();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self_window_id => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }

    fn init_window(size: &PhysicalSize<u32>) -> (EventLoop<()>, Window) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Vulkan Tutorial - Rust")
            .with_inner_size(*size)
            .build(&event_loop)
            .unwrap();
        (event_loop, window)
    }

    fn init_vulkan(window: &Window) -> VulkanContext {
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

        let extension_names = ash_window::enumerate_required_extensions(window).unwrap();
        let extension_names = extension_names
            .iter()
            .map(|ext| ext.as_ptr())
            .collect::<Vec<_>>();

        // if ENABLE_VALIDATION_LAYERS {
        //     extension_names.push(DebugReport::name().as_ptr());
        // }
        // let (_layer_names, layer_names_ptrs) = get_layer_names_and_pointers();

        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        // if ENABLE_VALIDATION_LAYERS {
        //     check_validation_layer_support(&entry);
        //     instance_create_info = instance_create_info.enabled_layer_names(&layer_names_ptrs);
        // }

        let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };

        VulkanContext { entry, instance }
    }

    // fn cleanup(&mut self) {

    // }
}
