use jni::objects::JObject;

use crate::util::jni::{*};

// consider revisiting LATER
// macro_rules! static_field {
//     ($name:ident, $class_name:expr, $field_name:expr, $return_type:ty, $sig:expr) => {
//         pub unsafe fn $name() -> $return_type {
//             println!("Getting {}:{}", $class_name, stringify!($field_name));
//             get_static_field($class_name, stringify!($field_name), $sig).unwrap()
//         }
//     };
// }

pub struct Minecraft;

impl Minecraft {
    pub unsafe fn get_obj<'a>() -> JObject<'a> {
        get_static_object_field("net.minecraft.client.Minecraft", "theMinecraft", "Lnet/minecraft/client/Minecraft;")
    }

    
}
