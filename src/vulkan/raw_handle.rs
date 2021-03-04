pub trait VkRawHandle {
    type Handle;
    
    fn raw_handle(&self) -> Self::Handle;
}

pub fn to_raw_handles<T>(slice: &[&T]) -> Vec<<T as VkRawHandle>::Handle> where T: VkRawHandle {
    slice.iter().map(|item| item.raw_handle()).collect::<Vec<_>>()
}