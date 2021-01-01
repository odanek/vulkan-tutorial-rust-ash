use std::{ffi::{CStr, CString}, os::raw::c_char};

pub fn extension_names_from_cstr(extensions: &Vec<&CStr>) -> Vec<*const c_char> {
    extensions
        .iter()
        .map(|ext| ext.as_ptr())
        .collect::<Vec<_>>()
}

pub fn extension_names_from_cstring(extensions: &Vec<CString>) -> Vec<*const c_char> {
    extensions
        .iter()
        .map(|ext| ext.as_ptr())
        .collect::<Vec<_>>()
}