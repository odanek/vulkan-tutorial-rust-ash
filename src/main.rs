use winit::{    
    dpi::{PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};

#[derive(Copy, Clone)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl WindowSize {
    pub fn new(width: u32, height: u32) -> WindowSize {
        return WindowSize {
            width,
            height,
        }
    }
}

pub struct HelloTriangleApp {    
    event_loop: EventLoop<()>,
    window: Window,
}

impl HelloTriangleApp {
    pub fn new(window_size: PhysicalSize<u32>) -> HelloTriangleApp {    
        let (event_loop, window) = HelloTriangleApp::init_window(&window_size);

        return HelloTriangleApp {
            event_loop,
            window
        }
    }

    pub fn run(self) {        
        // self.initVulkan();
        self.main_loop();
        // self.cleanup();
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

    // fn init_vulkan(&mut self) {

    // }

    fn main_loop(self) {
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

    // fn cleanup(&mut self) {

    // }
}

fn main() {
    let window_size = PhysicalSize::new(800, 600);
    let app = HelloTriangleApp::new(window_size);
    app.run();
    // println!("Window size: {:?}", app.window_size);
}
