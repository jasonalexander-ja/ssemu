use std::collections::HashMap;
use std::path::PathBuf;
use crate::interface::Interface;


pub struct TestInterface {
    should_log: String,
    should_warn: String,
    should_err: String,
    line_return: String,
    string_files: HashMap<PathBuf, String>,
    bin_files: HashMap<PathBuf, Vec<u8>>,
    should_write_addr: PathBuf,
    should_write_data: Vec<u8>
}

impl TestInterface {
    pub fn new_logger_test(should_log: &str, should_warn: &str, should_err: &str) -> TestInterface {
        TestInterface {
            should_log: should_log.to_owned(),
            should_warn: should_warn.to_owned(),
            should_err: should_err.to_owned(),
            line_return: format!(""),
            string_files: HashMap::new(),
            bin_files: HashMap::new(),
            should_write_addr: PathBuf::new(),
            should_write_data: Vec::new(),
        }
    }
}

impl Interface for TestInterface {
    fn log_msg(&self, msg: String) {
        assert_eq!(self.should_log, msg)
    }
    fn log_warn(&self, msg: String) {
        assert_eq!(self.should_warn, msg)
    }
    fn log_error(&self, msg: String) {
        assert_eq!(self.should_err, msg)
    }
    fn get_line(&self) -> String {
        self.line_return.clone()
    }
    fn read_fs_string(&self, path: &PathBuf) -> Result<String, ()> {
        match self.string_files.get(path) {
            Some(v) => Ok(v.clone()),
            None => Err(())
        }
    }
    fn read_fs_bytes(&self, path: &PathBuf) -> Result<Vec<u8>, ()> {
        match self.bin_files.get(path) {
            Some(v) => Ok(v.clone()),
            None => Err(())
        }
    }
    fn write_fs_bytes(&self, data: Vec<u8>, out: &PathBuf) -> Result<(), ()> {
        assert_eq!(self.should_write_addr, *out);
        assert_eq!(self.should_write_data, data);
        Ok(())
    }
}
