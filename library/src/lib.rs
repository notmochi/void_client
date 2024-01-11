use winapi::um::{wincon::FreeConsole, consoleapi::AllocConsole};
use windows::{ Win32::Foundation::*, Win32::System::SystemServices::*, };

pub mod void;

pub mod util {
    pub mod jvm;
    pub mod logger;
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            unsafe { AllocConsole(); }
            std::thread::spawn(|| unsafe {
                void::entry()
            });
            ()
        },
        DLL_PROCESS_DETACH => {
            println!("Freeing console, you can now close this window.");
            unsafe { FreeConsole(); }
            ()
        },
        _ => ()
    }
    true
}