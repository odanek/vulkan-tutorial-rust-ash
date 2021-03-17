// #![windows_subsystem = "windows"]

mod app;
mod logger;
mod render;
mod tutorial;
mod vulkan;

use app::App;
use log::LevelFilter;
use logger::init_logging;
use tutorial::TutorialApp;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Error;

fn main() {
    init_logging(LOG_LEVEL);

    let window_size = PhysicalSize::new(800, 600);
    let (event_loop, window) = create_window(&window_size);
    let mut app = TutorialApp::new(&window);
    let mut exit = false;

    log::info!("Starting event loop");
    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            // app.update();
            if !exit {
                app.draw_frame(&window);
            }
            // window.request_redraw();
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                app.wait_idle();
                exit = true;
                *control_flow = ControlFlow::Exit
            }
            WindowEvent::Resized(size) => {
                if size.width != 0 || size.height != 0 {
                    app.resized(&window, size);
                } else {
                    app.minimized(&window);
                }
            }
            _ => *control_flow = ControlFlow::Poll,
        },
        // Event::RedrawRequested(_window_id) => {
        //     app.draw_frame(&window);
        // }
        _ => *control_flow = ControlFlow::Poll,
    });
}

fn create_window(size: &PhysicalSize<u32>) -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial - Rust")
        .with_inner_size(*size)
        .build(&event_loop)
        .expect("Unable to create application window");
    (event_loop, window)
}
