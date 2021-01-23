use winit::{dpi::PhysicalSize, window::Window};

pub trait App {
    fn wait_idle(&self);
    fn update(&mut self);
    fn resized(&mut self, window: &Window, size: PhysicalSize<u32>);
    fn minimized(&mut self, window: &Window);
    fn draw_frame(&mut self, window: &Window);
}