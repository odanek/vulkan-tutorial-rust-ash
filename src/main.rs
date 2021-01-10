mod logger;
mod vulkan;

use log::LevelFilter;
use logger::init_logging;
use vulkan::{VkContext, VkSettings};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct HelloTriangleApp {
    event_loop: EventLoop<()>,
    _window: Window,
    vk_context: VkContext,
}

impl HelloTriangleApp {
    pub fn new(window_size: PhysicalSize<u32>) -> HelloTriangleApp {
        let (event_loop, window) = HelloTriangleApp::init_window(&window_size);
        let vk_settings = VkSettings { validation: true };
        let vk_context = VkContext::new(&window, &vk_settings);

        HelloTriangleApp {
            event_loop,
            _window: window,
            vk_context,
        }
    }

    pub fn run(self) {
        let vk_context = self.vk_context;

        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        vk_context.device.wait_idle();
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
    init_logging(LevelFilter::Debug);

    let window_size = PhysicalSize::new(800, 600);
    let app = HelloTriangleApp::new(window_size);
    app.run();
}

// https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct?rq=1
// https://stevedonovan.github.io/rustifications/2018/08/18/rust-closures-are-hard.html
// https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md
// Struct<'a> -> muzu na ni drzet referenci maximalne po 'a, i.e. Struct<'static> -> muzu na ni drzet referenci jak dlouho chci
