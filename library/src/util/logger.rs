use std::fmt;
pub struct Logger;

impl Logger {
    pub fn log<T: fmt::Display>(message: T) {
        println!("{}", message);
    }

    pub fn log_fmt(args: fmt::Arguments) {
        println!("{}", args);
    }
}