#[allow(unused)]
pub struct ModuleData {
    pub key: i16,
    pub name: String,
    pub description: String,
    pub toggled: bool,
}

impl ModuleData {
    pub fn new(key: i16, name: &str, description: &str, toggled: bool) -> Self {
        Self {
            key,
            name: name.to_string(),
            description: description.to_string(),
            toggled,
        }
    }

    pub fn on_key(&mut self, pressed_key: i16) {
        if self.key == pressed_key {
            self.toggled = !self.toggled;
        }
    }
}

pub trait Module {
    fn get_mod(&mut self) -> &mut ModuleData;
    unsafe fn on_tick(&self) { }
    unsafe fn on_loop(&self) { }
    unsafe fn on_enable(&self) { }
    unsafe fn on_disable(&self) { }
}
