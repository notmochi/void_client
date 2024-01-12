use crate::{modules::module::{Module, ModuleData}, util::jni};

pub struct SpeedModule {
    pub base: ModuleData
}

impl SpeedModule {
    pub fn new() -> Self {
        Self {
            base: ModuleData::new(win_key_codes::VK_B as i16, "Speed", "Allows you to go faster", true)
        }
    }
}

static mut TICKS: i32 = 0;

impl Module for SpeedModule {
    fn get_mod(&mut self) -> &mut ModuleData {
        &mut self.base
    }

    unsafe fn on_loop(&self) {
        let last_tick = TICKS;
        let mc = jni::get_static_object_field("net.minecraft.client.Minecraft", "theMinecraft", "Lnet/minecraft/client/Minecraft;");
        let player = jni::get_object_field(&mc, "thePlayer", "Lnet/minecraft/client/entity/EntityPlayerSP;");
        let ticks_existed = jni::get_int_field(&player, "ticksExisted");
        TICKS = ticks_existed;
        if TICKS != last_tick {
            real_tick();
        }
    }

    unsafe fn on_tick(&self) {
    }
}

pub unsafe fn real_tick() {
    let mc = jni::get_static_object_field("net.minecraft.client.Minecraft", "theMinecraft", "Lnet/minecraft/client/Minecraft;");
    let player = jni::get_object_field(&mc, "thePlayer", "Lnet/minecraft/client/entity/EntityPlayerSP;");
    let on_ground = jni::get_bool_field(&player, "onGround");
    if on_ground {
        jni::call_method(&player, "jump", "()V", &[]).v().unwrap();
    }
}