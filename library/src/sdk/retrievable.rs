use jni::objects::JObject;

pub trait Retrievable {
    fn get_obj(&self) -> &JObject<'static>;
}