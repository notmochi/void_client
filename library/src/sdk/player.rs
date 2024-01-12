use jni::objects::JObject;

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
}