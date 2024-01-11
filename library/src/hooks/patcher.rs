use crate::util::logger::Logger;

use super::gui;

pub fn apply_patches() {
    Logger::log("Applying GUI Hook...");
    setup_gui_hook();
    Logger::log("GUI Hook Applied");
}

pub fn stop() {
    unsafe { gui::stop(); }
}

pub fn setup_gui_hook() {
    unsafe { gui::init(); }
}