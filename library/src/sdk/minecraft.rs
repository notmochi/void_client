use ::jni::objects::JObject;

use crate::util::jni::{get_static_object_field, get_object_field};

use super::{retrievable::Retrievable, player::Player};

pub struct Minecraft {
    obj: JObject<'static>
}

impl Retrievable for Minecraft {
    fn get_obj(&self) -> &JObject<'static> {
        &self.obj
    }
}

impl TMinecraft for Minecraft {
    unsafe fn get_minecraft() -> Minecraft {
        Minecraft { 
            obj: get_static_object_field("net.minecraft.client.Minecraft", "theMinecraft", "Lnet/minecraft/client/Minecraft;")
        }
    }

    unsafe fn the_player(&self) -> Player {
        Player {
            obj: get_object_field(self.get_obj(), "name", "signature")
        }
    }
}

pub trait TMinecraft {
    unsafe fn get_minecraft() -> Minecraft;
    unsafe fn the_player(&self) -> Player;
}