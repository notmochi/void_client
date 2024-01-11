#![allow(unused)]

use jni::objects::{JClass, JValueOwned, JValueGen};
use jni::objects::{JObject, JValue};

use crate::void::ENV;
use crate::void::CLASS_LOADER;

pub unsafe fn get_class_obj(class_name: &str) -> JObject {
    ENV.as_mut().unwrap()
        .call_method(
        CLASS_LOADER.as_ref().unwrap(), "findClass", "(Ljava/lang/String;)Ljava/lang/Class;",
        &[JValue::Object(&*ENV.as_mut().unwrap().new_string(class_name).unwrap())]
        )
        .unwrap_or_else(|err|  {
            println!("Error when obtaining class {}: {}", class_name, err);
            return JValueOwned::from(JObject::null());
        })
        .l()
        .unwrap()
}

pub unsafe fn get_static_field<'a>(class_name: &'a str, name: &'a str, sig: &'a str) -> JValueGen<JObject<'a>> {
    return ENV.as_mut().unwrap()
        .get_static_field(JClass::from(get_class_obj(class_name)), name, sig)
        .unwrap_or_else(|err|  {
            println!("Error when statically getting {} ({}) from class {}: {}", name, sig, class_name, err);
            return JValueOwned::from(JObject::null());
        });
}

pub unsafe fn get_field<'a>(obj: &'a JObject<'a>, name: &'a str, sig: &'a str) -> JValueGen<JObject<'a>> {
    ENV.as_mut().unwrap()
        .get_field(obj, name, "I")
        .unwrap_or_else(|err| {
            println!("Error when getting {} ({}) from {:?}: {}", name, sig, obj, err);
            return JValueOwned::from(JObject::null());
        })
}

pub unsafe fn set_field(obj: &JObject<'_>, name: &str, sig: &str, set: JValue) {
    ENV.as_mut().unwrap()
        .set_field(obj, name, sig, set)
        .unwrap_or_else(|err| {
            println!("Error when setting {} ({}) from {:?} to {:?}: {}", name, sig, obj, set, err);
        })
}

pub unsafe fn get_static_int_field(class_name: &str, name: &str) -> i32 {
    get_static_field(class_name, name, "I")
        .i()
        .unwrap_or_else(|err| {
            println!("Error when casting {}::{}: {}", class_name, name, err);
            return i32::MIN;
        })
}

pub unsafe fn get_static_bool_field(class_name: &str, name: &str) -> bool {
    get_static_field(class_name, name, "Z")
        .z()
        .unwrap_or_else(|err| {
            println!("Error when casting {}::{}: {}", class_name, name, err);
            return false;
        })
}

pub unsafe fn get_static_byte_field(class_name: &str, name: &str) -> i8 {
    get_static_field(class_name, name, "B")
    .b()
        .unwrap_or_else(|err| {
            println!("Error when casting {}::{}: {}", class_name, name, err);
            return i8::MIN;
        })
}

pub unsafe fn get_static_short_field(class_name: &str, name: &str) -> i16 {
    get_static_field(class_name, name, "S")
    .s()
        .unwrap_or_else(|err| {
            println!("Error when casting {}::{}: {}", class_name, name, err);
            return i16::MIN;
        })
}

pub unsafe fn get_static_long_field(class_name: &str, name: &str) -> i64 {
    get_static_field(class_name, name, "J")
    .j()
        .unwrap_or_else(|err| {
            println!("Error when casting {}::{}: {}", class_name, name, err);
            return i64::MIN;
        })
}

pub unsafe fn get_static_float_field(class_name: &str, name: &str) -> f32 {
    get_static_field(class_name, name, "F")
    .f()
        .unwrap_or_else(|err| {
            println!("Error when casting {}::{}: {}", class_name, name, err);
            return f32::MIN;
        })
}

pub unsafe fn get_static_double_field(class_name: &str, name: &str) -> f64 {
    get_static_field(class_name, name, "D")
        .d()
        .unwrap_or_else(|err| {
            println!("Error when casting {}::{}: {}", class_name, name, err);
            return f64::MIN;
        })
}

pub unsafe fn get_static_object_field<'a>(class_name: &'a str, name: &'a str, signature: &'a str) -> JObject<'a> {
    get_static_field(class_name, name, signature)
        .l()
        .unwrap_or_else(move |err| {
            println!("Error when casting {}::{}: {}", class_name, name, err);
            return JObject::null();
        })
}

pub unsafe fn get_int_field(obj: &JObject, name: &str) -> i32 {
    get_field(obj, name, "I")
        .i()
        .unwrap_or_else(|err| {
            println!("Error when casting {}: {}", name, err);
            return i32::MIN;
        })
}

pub unsafe fn get_bool_field(obj: &JObject, name: &str) -> bool {
    get_field(obj, name, "Z")
        .z()
        .unwrap_or_else(|err| {
            println!("Error when casting {}: {}", name, err);
            return false;
        })
}

pub unsafe fn get_byte_field(obj: &JObject, name: &str) -> i8 {
    get_field(obj, name, "B")
        .b()
        .unwrap_or_else(|err| {
            println!("Error when casting {}: {}", name, err);
            return i8::MIN;
        })
}

pub unsafe fn get_short_field(obj: &JObject, name: &str) -> i16 {
    get_field(obj, name, "S")
        .s()
        .unwrap_or_else(|err| {
            println!("Error when casting {}: {}", name, err);
            return i16::MIN;
        })
}

pub unsafe fn get_long_field(obj: &JObject, name: &str) -> i64 {
    get_field(obj, name, "J")
        .j()
        .unwrap_or_else(|err| {
            println!("Error when casting {}: {}", name, err);
            return i64::MIN;
        })
}

pub unsafe fn get_float_field(obj: &JObject, name: &str) -> f32 {
    get_field(obj, name, "F")
        .f()
        .unwrap_or_else(|err| {
            println!("Error when casting {}: {}", name, err);
            return f32::MIN;
        })
}

pub unsafe fn get_double_field(obj: &JObject, name: &str) -> f64 {
    get_field(obj, name, "D")
        .d()
        .unwrap_or_else(|err| {
            println!("Error when casting {}: {}", name, err);
            return f64::MIN;
        })
}

pub unsafe fn get_object_field<'a>(obj: &'a JObject<'a>, name: &'a str, signature: &'a str) -> JObject<'a> {
    get_field(obj, name, signature)
        .l()
        .unwrap_or_else(move |err| {
            println!("Error when casting {}: {}", name, err);
            return JObject::null();
        })
}

pub unsafe fn set_bool_field(obj: &JObject, name: &str, set: bool) {
    set_field(obj, name, "Z", JValue::Bool(if set { 1 } else { 0 }));
}

pub unsafe fn set_byte_field(obj: &JObject, name: &str, set: i8) {
    set_field(obj, name, "B", JValue::Byte(set));
}

pub unsafe fn set_short_field(obj: &JObject, name: &str, set: i16) {
    set_field(obj, name, "S", JValue::Short(set));
}

pub unsafe fn set_long_field(obj: &JObject, name: &str, set: i64) {
    set_field(obj, name, "J", JValue::Long(set));
}

pub unsafe fn set_float_field(obj: &JObject, name: &str, set: f32) {
    set_field(obj, name, "F", JValue::Float(set));
}

pub unsafe fn set_double_field(obj: &JObject, name: &str, set: f64) {
    set_field(obj, name, "D", JValue::Double(set));
}

pub unsafe fn set_object_field(obj: &JObject, name: &str, signature: &str, set: &JObject) {
    set_field(obj, name, signature, JValue::Object(set));
}