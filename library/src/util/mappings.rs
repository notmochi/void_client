use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Type {
    Lunar,
    Forge,
    Vanilla,
}

pub static mut HASHMAP: Option<HashMap<(&'static str, Type), &'static str>> = None;
pub static mut CURRENT_TYPE: Type = Type::Lunar;

pub fn init_mappings() {
    let mut map = HashMap::new();

    map.insert(("thePlayer", Type::Forge), "h");
    map.insert(("thePlayer", Type::Lunar), "thePlayer");
    map.insert(("thePlayer", Type::Vanilla), "undef");
    map.insert(("theWorld", Type::Forge), "f");
    map.insert(("theWorld", Type::Lunar), "theWorld");
    map.insert(("theWorld", Type::Vanilla), "undef");

    unsafe {
        HASHMAP = Some(map);
    }
}

pub fn init_type() {
    unsafe {
        CURRENT_TYPE = Type::Lunar;
    }
}

pub fn get(key: &str) -> &'static str {
    let hashmap = unsafe { HASHMAP.as_ref().unwrap() };
    let current_type = unsafe { CURRENT_TYPE };
    hashmap.get(&(key, current_type)).unwrap_or(&"undef")
}