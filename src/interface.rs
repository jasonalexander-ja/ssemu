use std::io;
use std::fs;
use std::io::Write;
use std::io::stdout;
use std::path::PathBuf;
use colored::Colorize;


/// Defines standard methods for interacting with a user/host system. 
pub trait Interface {
    /// Logs a message to the user. 
    fn log_msg(&self, msg: String);
    /// Logs a message to the user without a linebreak. 
    fn log_inline(&self, msg: String);
    /// Logs a warning to the user. 
    fn log_warn(&self, msg: String);
    /// Logs an error to the user. 
    fn log_error(&self, msg: String);
    /// Gets a line of input from a user. 
    fn get_line(&self) -> String;
    /// Reads a file to a string. 
    fn read_fs_string(&self, path: &PathBuf) -> Result<String, ()>;
    /// Reads a file to bytes. 
    fn read_fs_bytes(&self, path: &PathBuf) -> Result<Vec<u8>, ()>;
    /// Writes bytes to a file. 
    fn write_fs_bytes(&self, data: Vec<u8>, out: &PathBuf) -> Result<(), ()>;
}

/// An interface for interacting with the cli. 
pub struct CliInterface();

impl Interface for CliInterface {
    fn log_msg(&self, msg: String) {
        println!("{}", msg);
    }
    #[allow(unused_must_use)]
    fn log_inline(&self, msg: String) {
        print!("{}", msg);
        stdout().flush();
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
    fn read_fs_string(&self, path: &PathBuf) -> Result<String, ()> {
        fs::read_to_string(path)
            .map_err(|_| ())
    }
    fn read_fs_bytes(&self, path: &PathBuf) -> Result<Vec<u8>, ()> {
        fs::read(path)
            .map_err(|_| ())
    }
    fn write_fs_bytes(&self, data: Vec<u8>, out: &PathBuf) -> Result<(), ()> {
        fs::write(&out, &data)
            .map_err(|_| ())
    }
}

