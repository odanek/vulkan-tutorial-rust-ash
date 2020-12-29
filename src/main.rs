use winit::{    
    dpi::{PhysicalSize},
};

mod app;
mod vulkan_context;

use app::HelloTriangleApp;

fn main() {
    let window_size = PhysicalSize::new(800, 600);
    let app = HelloTriangleApp::new(window_size);
    app.run();
}
