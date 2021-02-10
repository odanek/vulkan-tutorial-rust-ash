use std::{rc::Rc, sync::Arc};

use super::{device::VkDevice, fence::VkFence, semaphore::VkSemaphore, swap_chain::VkSwapChain};

pub struct VkSwapChainSync {
    pub max_frames_in_flight: usize,
    pub current_frame: usize,
    pub image_available_semaphore: Vec<VkSemaphore>,
    pub render_finished_semaphore: Vec<VkSemaphore>,
    pub in_flight_fences: Vec<Rc<VkFence>>,
    pub images_in_flight: Vec<Option<Rc<VkFence>>>,
}

impl VkSwapChainSync {
    pub fn new(
        device: &Arc<VkDevice>,
        swap_chain: &VkSwapChain,
        max_frames_in_flight: usize,
    ) -> VkSwapChainSync {
        let image_available_semaphore = create_semaphores(&device, max_frames_in_flight);
        let render_finished_semaphore = create_semaphores(&device, max_frames_in_flight);
        let in_flight_fences = create_fences(&device, max_frames_in_flight);
        let images_in_flight = (0..swap_chain.images.len())
            .map(|_| None)
            .collect::<Vec<_>>();

        VkSwapChainSync {
            max_frames_in_flight,
            current_frame: 0,
            image_available_semaphore,
            render_finished_semaphore,
            in_flight_fences,
            images_in_flight,
        }
    }

    pub fn advance_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.max_frames_in_flight;
    }
}

fn create_semaphores(device: &Arc<VkDevice>, count: usize) -> Vec<VkSemaphore> {
    (0..count)
        .map(|_| VkSemaphore::new(device))
        .collect::<Vec<_>>()
}

fn create_fences(device: &Arc<VkDevice>, count: usize) -> Vec<Rc<VkFence>> {
    (0..count).map(|_| Rc::new(VkFence::new(device))).collect::<Vec<_>>()
}
