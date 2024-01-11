use std::{fs::OpenOptions, time::Duration};
use std::os::windows::io::AsRawHandle;

use winapi::um::winuser::GetAsyncKeyState;
use winapi::um::libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleA};
use windows::core::s;

use jni::{AttachGuard, JavaVM};
use jni::objects::JObject;

use crate::hooks::patcher;
use crate::keys::key_handler::KeyHandler;
use crate::modules::manager;
use crate::modules::module::ModuleData;
use crate::sdk::mappings;
use crate::util::logger::Logger;

pub static mut JAVA_VM: Option<JavaVM> = None;
pub static mut ENV: Option<AttachGuard<'static>> = None;
pub static mut CLASS_LOADER: Option<JObject> = None;

pub static mut RUNNING: bool = true;

pub unsafe fn entry() {

    Logger::log("Attached to javaw.exe");

    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open("CONOUT$")
        .unwrap();
    let _ = winapi::um::processenv::SetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE, file.as_raw_handle() as *mut winapi::ctypes::c_void);

    Logger::log("Set STD output handle");

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

    Logger::log("Applying patches...");
    patcher::apply_patches();
    Logger::log("Patches applied");

    Logger::log("Loading Mappings...");
    mappings::init_mappings();
    Logger::log("Mappings loaded");
    
    Logger::log("Getting MC Type...");
    mappings::init_mappings();
    Logger::log_fmt(format_args!("{}{:?}", "MC Type: ", mappings::CURRENT_TYPE));

    Logger::log("Initializing Modules...");
    let mut key_handler = KeyHandler::new();
    manager::init();
    Logger::log("Modules initialized");

    Logger::log("Looping...");

    let loop_thread = std::thread::spawn(|| {
        loop {
            manager::on_loop();
            if GetAsyncKeyState(win_key_codes::VK_0) != 0 || !RUNNING {
                RUNNING = false;
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        Logger::log("Quitted loop thread");
    });
    
    loop {
        key_handler.on_tick();
        manager::on_tick();
        if !RUNNING {
            break;
        }
        std::thread::sleep(Duration::from_millis(50))
    }

    loop_thread.join().unwrap();

    exit_log("Exited loop, now freeing library...")
}

pub unsafe fn on_key(key: i32) {
    for module in manager::MODULES.as_mut().unwrap().iter_mut() {
        let m: &mut ModuleData = module.as_mut().get_mod();
        m.on_key(key as i16);
    }
}

pub unsafe fn exit_log(log: &str) {
    Logger::log(log);
    std::thread::sleep(Duration::from_millis(500));
    exit();
}

pub unsafe fn exit() {
    Logger::log("Deinitializing patches...");
    patcher::stop();
    Logger::log("Deinitialized patches, Freeing library...");
    let module = GetModuleHandleA(s!("void.dll").as_ptr() as *const i8);
    FreeLibraryAndExitThread(module, 0);
}