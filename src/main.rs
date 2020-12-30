mod vulkan;

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct HelloTriangleApp {
    event_loop: EventLoop<()>,
    window: Window,
    vulkan_context: vulkan::Context,
}

impl HelloTriangleApp {
    pub fn new(window_size: PhysicalSize<u32>) -> HelloTriangleApp {
        let (event_loop, window) = HelloTriangleApp::init_window(&window_size);
        let vulkan_settings = vulkan::Settings::new(false);
        let vulkan_context = vulkan::Context::new(&window, &vulkan_settings);

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

    // fn cleanup(&mut self) {

    // }
}

fn main() {
    let window_size = PhysicalSize::new(800, 600);
    let app = HelloTriangleApp::new(window_size);
    app.run();
}
