use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::interface::Interface;


pub struct TestInterface {
    pub should_log: String,
    pub should_warn: String,
    pub should_err: String,
    pub line_return: String,
    pub string_files: HashMap<PathBuf, String>,
    pub bin_files: HashMap<PathBuf, Vec<u8>>,
    pub should_write_addr: PathBuf,
    pub should_write_data: Vec<u8>,
    pub should_write_str_addr: PathBuf,
    pub should_write_str_data: String,
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
            should_write_str_addr: PathBuf::new(),
            should_write_str_data: format!(""),
        }
    }
}

impl Interface for TestInterface {
    fn log_msg(&self, msg: String) {
        assert_eq!(self.should_log, msg)
    }
    fn log_inline(&self, _msg: String) {
        
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
    fn write_fs_string(&self, data: String, out: &PathBuf) -> Result<(), ()> {
        assert_eq!(self.should_write_str_addr, *out);
        assert_eq!(self.should_write_str_data, data);
        Ok(())
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

pub struct TestApplyInterface {
    should_log: fn(String) -> (),
    should_warn: fn(String) -> (),
    should_err: fn(String) -> (),
    line_return: String,
    string_files: HashMap<PathBuf, String>,
    bin_files: HashMap<PathBuf, Vec<u8>>,
    should_write_addr: PathBuf,
    should_write_data: Vec<u8>,
    should_write_str_addr: PathBuf,
    should_write_str_data: String
}

impl TestApplyInterface {
    pub fn new_logger_test(should_log: fn(String) -> (), should_warn: fn(String) -> (), should_err: fn(String) -> ()) -> TestApplyInterface {
        TestApplyInterface {
            should_log: should_log.to_owned(),
            should_warn: should_warn.to_owned(),
            should_err: should_err.to_owned(),
            line_return: format!(""),
            string_files: HashMap::new(),
            bin_files: HashMap::new(),
            should_write_addr: PathBuf::new(),
            should_write_data: Vec::new(),
            should_write_str_addr: PathBuf::new(),
            should_write_str_data: format!("")
        }
    }
}

impl Interface for TestApplyInterface {
    fn log_msg(&self, msg: String) {
        (self.should_log)(msg)
    }
    fn log_inline(&self, _msg: String) {
        
    }
    fn log_warn(&self, msg: String) {
        (self.should_warn)(msg)
    }
    fn log_error(&self, msg: String) {
        (self.should_err)(msg)
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
    fn write_fs_string(&self, data: String, out: &PathBuf) -> Result<(), ()> {
        assert_eq!(self.should_write_str_addr, *out);
        assert_eq!(self.should_write_str_data, data);
        Ok(())
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

pub struct TestSucessiveInterface {
    pub log_count: AtomicUsize,
    pub should_log: Vec<String>,
    pub warn_count: AtomicUsize,
    pub should_warn: Vec<String>,
    pub err_count: AtomicUsize,
    pub should_err: Vec<String>,
    pub return_count: AtomicUsize,
    pub line_return: Vec<String>,
    pub str_file_count: AtomicUsize,
    pub string_files: Vec<HashMap<PathBuf, String>>,
    pub bin_file_count: AtomicUsize,
    pub bin_files: Vec<HashMap<PathBuf, Vec<u8>>>,
    pub write_count: AtomicUsize,
    pub should_write_addr: Vec<PathBuf>,
    pub should_write_data: Vec<Vec<u8>>,
    pub should_write_str_addr: Vec<PathBuf>,
    pub should_write_str_data: Vec<String>
}

impl TestSucessiveInterface {
    pub fn new_logger_test(should_log: Vec<&str>, should_warn: Vec<&str>, should_err: Vec<&str>) -> TestSucessiveInterface {
        TestSucessiveInterface {
            log_count: AtomicUsize::new(0),
            should_log: should_log.iter().map(|v| v.to_string()).collect(),
            warn_count: AtomicUsize::new(0),
            should_warn: should_warn.iter().map(|v| v.to_string()).collect(),
            err_count: AtomicUsize::new(0),
            should_err: should_err.iter().map(|v| v.to_string()).collect(),
            return_count: AtomicUsize::new(0),
            line_return: Vec::new(),
            str_file_count: AtomicUsize::new(0),
            string_files: Vec::new(),
            bin_file_count: AtomicUsize::new(0),
            bin_files: Vec::new(),
            write_count: AtomicUsize::new(0),
            should_write_addr: Vec::new(),
            should_write_data: Vec::new(),
            should_write_str_addr: Vec::new(),
            should_write_str_data: Vec::new()
        }
    }
}

impl Interface for TestSucessiveInterface {
    fn log_msg(&self, msg: String) {
        assert_eq!(self.should_log[self.log_count.load(Ordering::Relaxed)], msg);
        self.log_count.fetch_add(1, Ordering::Relaxed);
    }
    fn log_inline(&self, _msg: String) {
        
    }
    fn log_warn(&self, msg: String) {
        assert_eq!(self.should_warn[self.warn_count.load(Ordering::Relaxed)], msg);
        self.warn_count.fetch_add(1, Ordering::Relaxed);
    }
    fn log_error(&self, msg: String) {
        assert_eq!(self.should_err[self.err_count.load(Ordering::Relaxed)], msg);
        self.err_count.fetch_add(1, Ordering::Relaxed);
    }
    fn get_line(&self) -> String {
        let res = self.line_return[self.return_count.load(Ordering::Relaxed)].clone();
        self.return_count.fetch_add(1, Ordering::Relaxed);
        res
    }
    fn read_fs_string(&self, path: &PathBuf) -> Result<String, ()> {
        let res = match self.string_files[self.str_file_count.load(Ordering::Relaxed)].get(path) {
            Some(v) => Ok(v.clone()),
            None => Err(())
        };
        self.str_file_count.fetch_add(1, Ordering::Relaxed);
        res
    }
    fn write_fs_string(&self, data: String, out: &PathBuf) -> Result<(), ()> {
        assert_eq!(self.should_write_str_addr[self.write_count.load(Ordering::Relaxed)], *out);
        assert_eq!(self.should_write_str_data[self.write_count.load(Ordering::Relaxed)], data);
        self.write_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
    fn read_fs_bytes(&self, path: &PathBuf) -> Result<Vec<u8>, ()> {
        let res = match self.bin_files[self.bin_file_count.load(Ordering::Relaxed)].get(path) {
            Some(v) => Ok(v.clone()),
            None => Err(())
        };
        self.bin_file_count.fetch_add(1, Ordering::Relaxed);
        res
    }
    fn write_fs_bytes(&self, data: Vec<u8>, out: &PathBuf) -> Result<(), ()> {
        assert_eq!(self.should_write_addr[self.write_count.load(Ordering::Relaxed)], *out);
        assert_eq!(self.should_write_data[self.write_count.load(Ordering::Relaxed)], data);
        self.write_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}
