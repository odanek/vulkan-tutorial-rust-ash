# vulkan-tutorial-rust-ash
My take on [Vulkan tutorial](http://vulkan-tutorial.com) implementation in Rust and Ash

- Inspired by https://github.com/unknownue/vulkan-tutorial-rust and https://github.com/adrien-ben/vulkan-tutorial-rs
- Contains a very simple RAII - resources are automatically destroyed when dropped
- Lots of unused function warnings in the Matrix and Vec code. I plan to move this part into a seprate library.