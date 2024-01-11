use super::{example::speed::SpeedModule, module::Module};

pub static mut MODULES: Option<Vec<Box<dyn Module>>> = None;

pub unsafe fn init() {
    MODULES = Some(Vec::new());
    MODULES.as_mut().unwrap().push(Box::new(SpeedModule::new()));
}

pub unsafe fn on_tick() {
    for module in MODULES.as_mut().unwrap().iter_mut() {
        if module.get_mod().toggled {
            module.on_tick();
        }
    }
}