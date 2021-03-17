# vulkan-tutorial-rust-ash
My take on [Vulkan tutorial](http://vulkan-tutorial.com) implementation in Rust and Ash

- Inspired by https://github.com/unknownue/vulkan-tutorial-rust and https://github.com/adrien-ben/vulkan-tutorial-rs
- Contains a very simple RAII - resources are automatically destroyed when dropped
- The matrix and vector code will be moved to a separate crate. You should use https://github.com/rustgd/cgmath.