#![allow(unused)]

use jni::objects::JClass;
use jni::objects::{JObject, JValue};

use crate::void::ENV;
use crate::void::CLASS_LOADER;

pub unsafe fn get_class_obj(class_name: &str) -> JObject {
    ENV.as_mut().unwrap()
        .call_method(
        CLASS_LOADER.as_ref().unwrap(), "findClass", "(Ljava/lang/String;)Ljava/lang/Class;",
        &[JValue::Object(&*ENV.as_mut().unwrap().new_string(class_name).unwrap())]
        )
        .unwrap()
        .l()
        .unwrap()
}

pub unsafe fn get_static_int_field(class_name: &str, name: &str) -> i32 {
    ENV.as_mut().unwrap()
        .get_static_field(JClass::from(get_class_obj(class_name)), name, "I")
        .unwrap()
        .i()
        .unwrap()
}

pub unsafe fn get_static_bool_field(class_name: &str, name: &str) -> bool {
    ENV.as_mut().unwrap()
        .get_static_field(JClass::from(get_class_obj(class_name)), name, "Z")
        .unwrap()
        .z()
        .unwrap()
}

pub unsafe fn get_static_byte_field(class_name: &str, name: &str) -> i8 {
    ENV.as_mut().unwrap()
        .get_static_field(JClass::from(get_class_obj(class_name)), name, "B")
        .unwrap()
        .b()
        .unwrap()
}

pub unsafe fn get_static_short_field(class_name: &str, name: &str) -> i16 {
    ENV.as_mut().unwrap()
        .get_static_field(JClass::from(get_class_obj(class_name)), name, "S")
        .unwrap()
        .s()
        .unwrap()
}

pub unsafe fn get_static_long_field(class_name: &str, name: &str) -> i64 {
    ENV.as_mut().unwrap()
        .get_static_field(JClass::from(get_class_obj(class_name)), name, "J")
        .unwrap()
        .j()
        .unwrap()
}

pub unsafe fn get_static_float_field(class_name: &str, name: &str) -> f32 {
    ENV.as_mut().unwrap()
        .get_static_field(JClass::from(get_class_obj(class_name)), name, "F")
        .unwrap()
        .f()
        .unwrap()
}

pub unsafe fn get_static_double_field(class_name: &str, name: &str) -> f64 {
    ENV.as_mut().unwrap()
        .get_static_field(JClass::from(get_class_obj(class_name)), name, "D")
        .unwrap()
        .d()
        .unwrap()
}

pub unsafe fn get_static_object_field(class_name: &str, name: &str, signature: &str) -> JObject<'static> {
    ENV.as_mut().unwrap()
        .get_static_field(JClass::from(get_class_obj(class_name)), name, signature)
        .unwrap()
        .l()
        .unwrap()
}

pub unsafe fn get_int_field(obj: &JObject, name: &str) -> i32 {
    ENV.as_mut().unwrap()
        .get_field(obj, name, "I")
        .unwrap()
        .i()
        .unwrap()
}

pub unsafe fn get_bool_field(obj: &JObject, name: &str) -> bool {
    ENV.as_mut().unwrap()
        .get_field(obj, name, "Z")
        .unwrap()
        .z()
        .unwrap()
}

pub unsafe fn get_byte_field(obj: &JObject, name: &str) -> i8 {
    ENV.as_mut().unwrap()
        .get_field(obj, name, "F")
        .unwrap()
        .b()
        .unwrap()
}

pub unsafe fn get_short_field(obj: &JObject, name: &str) -> i16 {
    ENV.as_mut().unwrap()
        .get_field(obj, name, "S")
        .unwrap()
        .s()
        .unwrap()
}

pub unsafe fn get_long_field(obj: &JObject, name: &str) -> i64 {
    ENV.as_mut().unwrap()
        .get_field(obj, name, "J")
        .unwrap()
        .j()
        .unwrap()
}

pub unsafe fn get_float_field(obj: &JObject, name: &str) -> f32 {
    ENV.as_mut().unwrap()
        .get_field(obj, name, "F")
        .unwrap()
        .f()
        .unwrap()
}

pub unsafe fn get_double_field(obj: &JObject, name: &str) -> f64 {
    ENV.as_mut().unwrap()
        .get_field(obj, name, "D")
        .unwrap()
        .d()
        .unwrap()
}

pub unsafe fn get_object_field(obj: &JObject, name: &str, signature: &str) -> JObject<'static> {
    ENV.as_mut().unwrap()
        .get_field(obj, name, signature)
        .unwrap()
        .l()
        .unwrap()
}

pub unsafe fn set_bool_field(obj: &JObject, name: &str, set: bool) {
    ENV.as_mut().unwrap()
        .set_field(obj, name, "Z", JValue::Bool(if set { 1 } else { 0 }))
        .unwrap()
}

pub unsafe fn set_byte_field(obj: &JObject, name: &str, set: i8) {
    ENV.as_mut().unwrap()
        .set_field(obj, name, "B", JValue::Byte(set))
        .unwrap()
}

pub unsafe fn set_short_field(obj: &JObject, name: &str, set: i16) {
    ENV.as_mut().unwrap()
        .set_field(obj, name, "S", JValue::Short(set))
        .unwrap()
}

pub unsafe fn set_long_field(obj: &JObject, name: &str, set: i64) {
    ENV.as_mut().unwrap()
        .set_field(obj, name, "J", JValue::Long(set))
        .unwrap()
}

pub unsafe fn set_float_field(obj: &JObject, name: &str, set: f32) {
    ENV.as_mut().unwrap()
        .set_field(obj, name, "F", JValue::Float(set))
        .unwrap()
}

pub unsafe fn set_double_field(obj: &JObject, name: &str, set: f64) {
    ENV.as_mut().unwrap()
        .set_field(obj, name, "D", JValue::Double(set))
        .unwrap()
}

pub unsafe fn set_object_field(obj: &JObject, name: &str, signature: &str, set: &JObject) {
    ENV.as_mut().unwrap()
        .set_field(obj, name, signature, JValue::Object(set))
        .unwrap()
}