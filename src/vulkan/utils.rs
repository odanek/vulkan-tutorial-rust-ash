use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

pub trait AsRawHandle {
    type Handle;

    fn as_raw_handle(&self) -> Self::Handle;
}

impl<'a> AsRawHandle for &'a CStr {
    type Handle = *const c_char;

    fn as_raw_handle(&self) -> Self::Handle {
        self.as_ptr()
    }
}

impl AsRawHandle for CString {
    type Handle = *const c_char;

    fn as_raw_handle(&self) -> Self::Handle {
        self.as_ptr()
    }
}

pub fn as_raw_handles<T>(slice: &[T]) -> Vec<<T as AsRawHandle>::Handle>
where
    T: AsRawHandle,
{
    slice.iter().map(|item| item.as_raw_handle()).collect()
}

pub fn coerce_string(raw_string_array: &[c_char]) -> String {
    let raw_string = unsafe { CStr::from_ptr(raw_string_array.as_ptr()) };
    raw_string
        .to_str()
        .expect("Failed to convert vulkan raw string")
        .to_owned()
}
