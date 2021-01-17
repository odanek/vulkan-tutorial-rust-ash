pub trait App {
    fn wait_idle(&self);
    fn update(&mut self);
    fn draw_frame(&mut self);
}