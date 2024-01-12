use jni::objects::JObject;

use crate::util::jni::{get_double_field, get_bool_field, get_float_field, get_int_field};

use super::{entity::TEntity, retrievable::Retrievable};

pub struct Player {
    pub obj: JObject<'static>
}

impl Retrievable for Player {
    fn get_obj(&self) -> &JObject<'static> {
        &self.obj
    }
}

impl TPlayer for Player {
    unsafe fn jump(&self) {
        todo!()
    }
}

pub trait TPlayer {
    unsafe fn jump(&self);
}


impl TEntity for Player {
    unsafe fn get_pos_x(&self) -> f64 {
        get_double_field(self.get_obj(), "posX")
    }

    unsafe fn get_pos_y(&self) -> f64 {
        get_double_field(self.get_obj(), "posY")
    }

    unsafe fn get_pos_z(&self) -> f64 {
        get_double_field(self.get_obj(), "posZ")
    }

    unsafe fn set_pos_x(&self) {
        todo!()
    }

    unsafe fn set_pos_y(&self) {
        todo!()
    }

    unsafe fn set_pos_z(&self) {
        todo!()
    }

    unsafe fn get_on_ground(&self) -> bool {
        get_bool_field(self.get_obj(), "onGround")
    }

    unsafe fn set_on_ground(&self) {
        todo!()
    }

    unsafe fn get_yaw(&self) -> f32 {
        get_float_field(self.get_obj(), "rotationYaw")
    }

    unsafe fn get_pitch(&self) -> f32 {
        get_float_field(self.get_obj(), "rotationPitch")
    }

    unsafe fn set_yaw(&self) {
        todo!()
    }

    unsafe fn set_pitch(&self) {
        todo!()
    }

    unsafe fn get_ticks_existed(&self) -> i32 {
        get_int_field(self.get_obj(), "ticksExisted")
    }
}