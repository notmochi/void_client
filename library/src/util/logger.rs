use std::fmt;
pub struct Logger;

impl Logger {
    pub fn log<T: fmt::Display>(message: T) {
        println!("{}", message);
    }

    pub fn log_fmt(args: fmt::Arguments) {
        println!("{}", args);
    }
    
    // TODO: implement chat printing
    pub fn log_chat<T: fmt::Display>(message: T) {
        println!("{}", message); 
    }

    pub fn log_chat_fmt(args: fmt::Arguments) {
        println!("{}", args);
    }
}