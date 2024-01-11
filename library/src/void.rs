use std::{fs::OpenOptions, time::Duration};
use std::os::windows::io::AsRawHandle;

use winapi::um::winuser::GetAsyncKeyState;
use winapi::um::{libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleA}};
use windows::core::s;

use jni::{AttachGuard, JavaVM};
use jni::objects::JObject;

use crate::util::logger::Logger;

pub static mut JAVA_VM: Option<JavaVM> = None;
pub static mut ENV: Option<AttachGuard<'static>> = None;
pub static mut CLASS_LOADER: Option<JObject> = None;

pub unsafe fn entry() {

    Logger::log("Attached to javaw.exe");

    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open("CONOUT$")
        .unwrap();
    let _ = winapi::um::processenv::SetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE, file.as_raw_handle() as *mut winapi::ctypes::c_void);

    Logger::log("Set std output handle");

    let vms = crate::util::jvm::get_created_jvms();

    if vms.is_none() {
        exit_log("No JVM Was found, exiting...");
    }
    Logger::log_fmt(format_args!("{}{:?}", "Found JVMS: ", vms));

    let vm = vms.unwrap()[0];

    if vm.is_null() {
        exit_log("First VM was null, exiting...");
    }
    Logger::log_fmt(format_args!("{}{:?}", "First JVM: ", vm));

    JAVA_VM = Some(JavaVM::from_raw(vm as *mut jni::sys::JavaVM).unwrap());
    ENV = Some(JAVA_VM.as_ref().unwrap().attach_current_thread().unwrap());

    Logger::log("Set Global vars");
    Logger::log("Retrieving Class Loader...");

    CLASS_LOADER = crate::util::jvm::get_class_loader();

    Logger::log_fmt(format_args!("{}{:?}", "Retrieved Class Loader: ", CLASS_LOADER.as_ref().unwrap()));
    Logger::log("Looping...");

    loop {
        if GetAsyncKeyState(win_key_codes::VK_0) != 0 {
            break;
        }
    }

    exit_log("Exited loop, now freeing library...")
}

pub unsafe fn exit_log(log: &str) {
    Logger::log(log);
    std::thread::sleep(Duration::from_millis(500));
    exit();
}

pub unsafe fn exit() {
    let module = GetModuleHandleA(s!("void.dll").as_ptr() as *const i8);
    FreeLibraryAndExitThread(module, 0);
}