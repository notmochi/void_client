use jni::objects::JObject;

use super::retrievable::Retrievable;

pub struct Entity {
    obj: JObject<'static>
}

impl Retrievable for Entity {
    fn get_obj(&self) -> &JObject<'static> {
        &self.obj
    }
}

impl TEntity for Entity {
    unsafe fn get_pos_x(&self) -> f64 {
        todo!()
    }

    unsafe fn get_pos_y(&self) -> f64 {
        todo!()
    }

    unsafe fn get_pos_z(&self) -> f64 {
        todo!()
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
        todo!()
    }

    unsafe fn set_on_ground(&self) {
        todo!()
    }

    unsafe fn get_yaw(&self) -> f32 {
        todo!()
    }

    unsafe fn get_pitch(&self) -> f32 {
        todo!()
    }

    unsafe fn set_yaw(&self) {
        todo!()
    }

    unsafe fn set_pitch(&self) {
        todo!()
    }

    unsafe fn get_ticks_existed(&self) -> i32 {
        todo!()
    }
}

pub trait TEntity {
    unsafe fn get_pos_x(&self) -> f64;
    unsafe fn get_pos_y(&self) -> f64;
    unsafe fn get_pos_z(&self) -> f64;
    unsafe fn set_pos_x(&self);
    unsafe fn set_pos_y(&self);
    unsafe fn set_pos_z(&self);
    unsafe fn get_on_ground(&self) -> bool;
    unsafe fn set_on_ground(&self);
    unsafe fn get_yaw(&self) -> f32;
    unsafe fn get_pitch(&self) -> f32;
    unsafe fn set_yaw(&self);
    unsafe fn set_pitch(&self);
    unsafe fn get_ticks_existed(&self) -> i32;
}