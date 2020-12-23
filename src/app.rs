use winit::{    
    dpi::{PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};

// use ash::version::{DeviceV1_2, EntryV1_2, InstanceV1_2};
// use ash::{vk, Device, Entry, Instance};
use ash::version::{EntryV1_0};
use ash::{vk, Entry};
use std::ffi::{CString};

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
        let vulkan_context = HelloTriangleApp::init_vulkan();

        return HelloTriangleApp {
            event_loop,
            window,
            vulkan_context
        }
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

    fn init_vulkan() -> VulkanContext {
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

        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info);
                        
        let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };
        
        VulkanContext {
            entry,
            instance,
        }
    }

    // fn cleanup(&mut self) {

    // }
}