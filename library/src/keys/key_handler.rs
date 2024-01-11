use std::collections::HashMap;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use crate::void;

pub struct KeyHandler {
    keys: HashMap<i32, bool>,
}

impl KeyHandler {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub unsafe fn on_tick(&mut self) {
        for key in 0..255 {
            let state = GetAsyncKeyState(key);
            let pressed = state != 0;

            if let Some(old_state) = self.keys.get(&key) {
                if *old_state != pressed && pressed {
                    void::on_key(key);
                }
            }

            self.keys.insert(key, pressed);
        }
    }
}