use std::io;
use colored::Colorize;


pub trait Interface {
    fn log_msg(&self, msg: String);
    fn log_warn(&self, msg: String);
    fn log_error(&self, msg: String);
    fn get_line(&self) -> String;
}

pub struct CliInterface();

impl Interface for CliInterface {
    fn log_msg(&self, msg: String) {
        println!("{}", msg);
    }
    fn log_warn(&self, msg: String) {
        println!("{}", msg.as_str().yellow());
    }
    fn log_error(&self, msg: String) {
        println!("{}", msg.as_str().red());
    }
    fn get_line(&self) -> String {
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        line
    }
}

