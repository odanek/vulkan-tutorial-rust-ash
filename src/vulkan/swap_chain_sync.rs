use super::{device::VkDevice, fence::VkFence, semaphore::VkSemaphore, swap_chain::VkSwapChain};

pub struct VkSwapChainSync {
    pub max_frames_in_flight: usize,
    pub current_frame: usize,

    pub image_available_semaphore: Vec<VkSemaphore>,
    pub render_finished_semaphore: Vec<VkSemaphore>,
    pub in_flight_fences: Vec<VkFence>,
    pub images_in_flight: Vec<Option<VkFence>>,
}

impl VkSwapChainSync {
    pub fn new(
        device: &VkDevice,
        swap_chain: &VkSwapChain,
        max_frames_in_flight: usize,
    ) -> VkSwapChainSync {
        let image_available_semaphore = create_semaphores(&device, max_frames_in_flight);
        let render_finished_semaphore = create_semaphores(&device, max_frames_in_flight);
        let in_flight_fences = create_fences(&device, max_frames_in_flight);
        let images_in_flight: Vec<Option<VkFence>> = (0..swap_chain.images.len())
            .map(|_| None)
            .collect::<Vec<Option<VkFence>>>();

        VkSwapChainSync {
            max_frames_in_flight,
            current_frame: 0,
            image_available_semaphore,
            render_finished_semaphore,
            in_flight_fences,
            images_in_flight,
        }
    }

    pub fn cleanup(&self, device: &VkDevice) {
        self.image_available_semaphore
            .iter()
            .for_each(|semaphore| semaphore.cleanup(device));
        self.render_finished_semaphore
            .iter()
            .for_each(|semaphore| semaphore.cleanup(device));
        self.in_flight_fences
            .iter()
            .for_each(|fence| fence.cleanup(&device));
    }
}

fn create_semaphores(device: &VkDevice, count: usize) -> Vec<VkSemaphore> {
    (0..count)
        .map(|_| VkSemaphore::new(device))
        .collect::<Vec<_>>()
}

fn create_fences(device: &VkDevice, count: usize) -> Vec<VkFence> {
    (0..count).map(|_| VkFence::new(device)).collect::<Vec<_>>()
}
