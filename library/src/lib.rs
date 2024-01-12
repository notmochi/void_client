use winapi::um::{wincon::FreeConsole, consoleapi::AllocConsole};
use windows::{ Win32::Foundation::*, Win32::System::SystemServices::*, };

pub mod void;

pub mod guilib {
    pub mod app;
    pub mod input;
    pub mod painter;
    pub mod utils;
    pub mod shader;
}

pub mod util {
    pub mod jvm;
    pub mod logger;
    pub mod mappings;
    pub mod jni;
}

pub mod hooks {
    pub mod gui;
    pub mod patcher;
}

pub mod modules {
    pub mod manager;
    pub mod module;
    pub mod example {
        pub mod speed;
    }
}

pub mod keys {
    pub mod key_handler;
}

pub mod sdk {
    pub mod minecraft;
    pub mod entity;
    pub mod player;
    pub mod retrievable;
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