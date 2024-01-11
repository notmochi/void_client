use std::ffi::CString;
use std::fs::{self};
use std::path::PathBuf;
use std::ptr::{null_mut, null};
use winapi::um::processthreadsapi::{OpenProcess, CreateRemoteThread};
use winapi::um::winnt::{PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_WRITE, PROCESS_VM_READ};

use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::libloaderapi::{GetProcAddress, GetModuleHandleA};
use winapi::um::winuser::{FindWindowA, GetWindowThreadProcessId};
use winapi::um::winnt::{MEM_COMMIT, PAGE_READWRITE};

fn main() {
    let dll = PathBuf::from("target/debug/void.dll");
    let dll_path = fs::canonicalize(&dll).unwrap();
    println!("Injecting: {:?}", dll_path);
    
    let hwnd = unsafe { FindWindowA(b"LWJGL\0".as_ptr() as *const _, null()) };
    let mut pid = 0;
    unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
    if pid == 0 {
        println!("Process not found");
        return;
    }
    println!("Process ID: {}", pid);
    let process_handle = unsafe { OpenProcess(PROCESS_CREATE_THREAD | PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ, 0, pid) };
    println!("Handle: {:X}", process_handle as usize);
    let dll_path_cstring = CString::new(dll_path.to_str().unwrap()).unwrap();
    let dll_path_address = unsafe { VirtualAllocEx(process_handle, null_mut(), dll_path_cstring.to_bytes().len(), MEM_COMMIT, PAGE_READWRITE) };
    println!("Allocated Memory: {:X}", dll_path_address as usize);
    unsafe { WriteProcessMemory(process_handle, dll_path_address, dll_path_cstring.as_ptr() as *mut _, dll_path_cstring.to_bytes().len(), null_mut()) };
    let load_library_address = unsafe { GetProcAddress(GetModuleHandleA(b"kernel32\0".as_ptr() as *const _), b"LoadLibraryA\0".as_ptr() as *const _) };
    let thread_id = unsafe { CreateRemoteThread(process_handle, null_mut(), 0, Some(std::mem::transmute(load_library_address)), dll_path_address, 0, null_mut()) };
    if thread_id.is_null() {
        println!("invalid thread");
        return;
    }
    unsafe { CloseHandle(process_handle) };
}
