mod vulkan;

use vulkan::{VkContext, VkSettings};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct HelloTriangleApp {
    event_loop: EventLoop<()>,
    window: Window,
    vk_context: VkContext,
}

impl HelloTriangleApp {
    pub fn new(window_size: PhysicalSize<u32>) -> HelloTriangleApp {
        let (event_loop, window) = HelloTriangleApp::init_window(&window_size);
        let vk_settings = VkSettings { validation: true };
        let vk_context = VkContext::new(&window, &vk_settings);

        return HelloTriangleApp {
            event_loop,
            window,
            vk_context,
        };
    }

    pub fn run(self) {
        let _ = self.window;
        let vk_context = self.vk_context;

        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        vk_context.wait_device_idle();
                        *control_flow = ControlFlow::Exit
                    }
                    _ => (),
                },
                _ => (),
            });
    }

    fn init_window(size: &PhysicalSize<u32>) -> (EventLoop<()>, Window) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Vulkan Tutorial - Rust")
            .with_inner_size(*size)
            .build(&event_loop)
            .expect("Unable to create application window");
        (event_loop, window)
    }
}

fn main() {
    let window_size = PhysicalSize::new(800, 600);
    let app = HelloTriangleApp::new(window_size);
    app.run();
}
