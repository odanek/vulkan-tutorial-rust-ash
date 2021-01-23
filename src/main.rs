mod app;
mod hello_triangle;
mod logger;
mod vulkan;

use app::App;
use hello_triangle::HelloTriangleApp;
use log::LevelFilter;
use logger::init_logging;
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

    let mut app = HelloTriangleApp::new(&window);
    app.record_commands();

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            // app.update();
            // window.request_redraw();
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                app.wait_idle();
                *control_flow = ControlFlow::Exit
            },
            WindowEvent::Resized(size) => {
                if size.width != 0 || size.height != 0 {
                    app.resized(&window, size);
                } else {
                    app.minimized(&window);
                }
            },            
            _ => (),
        },
        Event::RedrawRequested(_window_id) => {
            app.draw_frame(&window);
        }
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        //_ => *control_flow = ControlFlow::Poll,
        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        _ => *control_flow = ControlFlow::Wait,
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

// https://github.com/adrien-ben/vulkan-tutorial-rs/blob/master/src/main.rs
// https://github.com/unknownue/vulkan-tutorial-rust/blob/master/src/tutorials/15_hello_triangle.rs

// https://github.com/gfx-rs/wgpu-rs/blob/master/examples/hello-triangle/main.rs

// https://stackoverflow.com/questions/30938499/why-is-the-sized-bound-necessary-in-this-trait
// https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct?rq=1
// https://stevedonovan.github.io/rustifications/2018/08/18/rust-closures-are-hard.html
// https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md
// Struct<'a> -> muzu na ni drzet referenci maximalne po 'a, i.e. Struct<'static> -> muzu na ni drzet referenci jak dlouho chci
