use std::ffi::CString;

use winapi::{um::{libloaderapi::{GetProcAddress, GetModuleHandleA}, wingdi::wglGetProcAddress}, shared::minwindef::HINSTANCE__};
use windows::core::PCSTR;

pub unsafe fn get_proc_address(function_name: &str) -> *const usize {
    let opengl32 = get_module("opengl32.dll");
    let c = CString::new(function_name).unwrap();
    let d = PCSTR::from_raw(c.as_ptr() as *const u8).as_ptr() as _;
    let mut process_address = GetProcAddress(opengl32, d);
    if process_address.is_null() {
        process_address = wglGetProcAddress(d);
    }
    process_address as *const usize
}

pub fn get_module(module_name: &str) -> *mut HINSTANCE__ {
    let module: *mut HINSTANCE__;
    unsafe {
        let o = CString::new(module_name).unwrap();
        module = GetModuleHandleA(PCSTR::from_raw(o.as_ptr() as *const u8).as_ptr() as _);
    }
    module
}
