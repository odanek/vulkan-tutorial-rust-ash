use std::{ffi::{CStr, CString}, os::raw::c_char};

pub trait RawPtrConvertible {
    fn as_raw_ptr(&self) -> *const c_char;
}

impl<'a> RawPtrConvertible for &'a CStr {
    fn as_raw_ptr(&self) -> *const c_char {
        self.as_ptr()
    }
}

impl RawPtrConvertible for CString {
    fn as_raw_ptr(&self) -> *const c_char {
        self.as_ptr()
    }
}

pub fn coerce_extension_names<T: RawPtrConvertible>(extensions: &Vec<T>) -> Vec<*const c_char> {
    extensions
        .iter()
        .map(|ext| ext.as_raw_ptr())
        .collect::<Vec<_>>()
}
