use std::os::raw::{c_int, c_void};

use jni::objects::{JObject, JObjectArray, JValue};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

use windows::core::s;

use crate::void::ENV;
use crate::util::logger::Logger;

type GetCreatedJavaVMs = extern "system" fn(*mut *mut c_void, c_int, *mut c_int) -> c_int;

pub fn get_jni_get_created_jvms() -> Option<GetCreatedJavaVMs> {
    let jvm_module = unsafe { GetModuleHandleA(s!("jvm.dll").as_ptr() as *const i8) };
    if jvm_module.is_null() {
        return None;
    }
    let jvm_proc_address = unsafe { GetProcAddress(jvm_module, s!("JNI_GetCreatedJavaVMs").as_ptr() as *const i8) };
    if jvm_proc_address.is_null() {
        return None;
    }
    let get_created_jvm = unsafe { std::mem::transmute(jvm_proc_address) };
    Some(get_created_jvm)
}

pub fn get_created_jvms() -> Option<Vec<*mut c_void>> {
    let get_created_jvms = get_jni_get_created_jvms()?;
    let mut jvm = Vec::with_capacity(1);
    let mut num_vms = 0;
    let status = get_created_jvms(jvm.as_mut_ptr(), jvm.capacity() as c_int, &mut num_vms as *mut c_int);
    if status != 0 {
        return None;
    }
    unsafe { jvm.set_len(num_vms as usize) };
    Some(jvm)
}

pub unsafe fn get_class_loader<'a>() -> Option<JObject<'a>> {
    let stack_traces_map: JObject = ENV.as_deref_mut().unwrap().call_static_method("java/lang/Thread", "getAllStackTraces", "()Ljava/util/Map;", &[]).unwrap().l().unwrap();
    let threads_set: JObject = ENV.as_deref_mut().unwrap().call_method(&stack_traces_map, "keySet", "()Ljava/util/Set;", &[]).unwrap().l().unwrap();
    let threads: JObject = ENV.as_deref_mut().unwrap().call_method(&threads_set, "toArray", "()[Ljava/lang/Object;", &[]).unwrap().l().unwrap();
    let threads_array: JObjectArray = JObjectArray::from(threads);
    let threads_amount: i32 = ENV.as_deref_mut().unwrap().get_array_length(&threads_array).unwrap();
    let mut class_loader: JObject = JObject::null();

    Logger::log_fmt(format_args!("{}{}", "Threads: ", threads_amount));

    for i in 0..threads_amount {
        let thread: JObject<'_> = ENV.as_deref_mut().unwrap().get_object_array_element(&threads_array, i).unwrap();
        class_loader = ENV.as_deref_mut().unwrap().call_method(&thread, "getContextClassLoader", "()Ljava/lang/ClassLoader;", &[]).unwrap().l().unwrap();
        if !class_loader.is_null() {
            let class_name = ENV.as_deref_mut().unwrap().new_string("net.minecraft.client.Minecraft").unwrap();
            let minecraft_class = ENV.as_deref_mut().unwrap().call_method(&class_loader, "findClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[JValue::Object(&*class_name)]).unwrap().l().unwrap();
            if !minecraft_class.is_null() {
                Logger::log("Found Minecraft class");
                ENV.as_deref_mut().unwrap().delete_local_ref(minecraft_class).unwrap();
                break;
            }
        }
        ENV.as_deref_mut().unwrap().delete_local_ref(thread).unwrap();
    }

    Some(class_loader)
}
