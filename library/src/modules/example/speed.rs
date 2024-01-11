use crate::{modules::module::{Module, ModuleData}, util::logger::Logger};

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

    unsafe fn on_tick(&self) {
        Logger::log_fmt(format_args!("{}", TICKS));
        TICKS += 1;
    }
}