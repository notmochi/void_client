use crate::modules::module::{Module, ModuleData};

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
        // let last_ticks = TICKS;
        // let player = Minecraft::thePlayer(). 
        // TICKS = player.getTicksExisted(); 
        // if last_ticks != TICKS ..
        //  real_tick();
    }

    unsafe fn on_tick(&self) {
        let last_tick = TICKS;
        TICKS += 1;
        if last_tick != TICKS {
            real_tick();
        }
    }
}

pub unsafe fn real_tick() {
    // Logger::log_fmt(format_args!("{}", TICKS));
}