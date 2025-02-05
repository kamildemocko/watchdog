use std::{fs::{self, File}, path::{Path, PathBuf}};
use std::io::Write;
use std::error::Error;

use csv::WriterBuilder;

use crate::logger::{Logger,LogEntry};

#[derive(Debug)]
pub struct LogConfig {
    pub max_file_size: u64,
    pub rotation_count: u32,
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            max_file_size: 10 * 1024 * 1024,
            rotation_count: 5,
        }
    }
}

pub struct CSVLogger {
    path: PathBuf,
    delimeter: char,
    config: LogConfig
}

impl Logger for CSVLogger {
    fn new(path: PathBuf) -> Self {
        let cfg = LogConfig::default();
        let default_delimeter = ',';

        CSVLogger {
            path: path, 
            delimeter: default_delimeter, 
            config: cfg
        }
    }

    fn log_item(&self, entry: LogEntry) {
        let message = self.prepare_log_entry(entry).expect("cannot prepare log message");
        self.write_log(message).expect("cannot write log file to fs");
        }
}

impl CSVLogger {
    #[allow(dead_code)]
    pub fn with_config(mut self, config: LogConfig) -> Self {
        self.config = config;
        self
    }

    #[allow(dead_code)]
    pub fn with_delimeter(mut self, delimeter: char) -> Self {
        self.delimeter = delimeter;
        self
    }

    fn prepare_log_entry(&self, entry: LogEntry) -> Result<String, Box<dyn Error>> {
        let mut wrt = WriterBuilder::new()
            .has_headers(!Path::exists(&self.path))
            .delimiter(self.delimeter as u8)
            .from_writer(vec![]);
        wrt.serialize(entry)?;

        let data = String::from_utf8(wrt.into_inner()?)?;
        Ok(data)
    }

    fn write_log(&self, message: String) -> Result<(), Box<dyn Error>> {
        let mut file: fs::File;
        if Path::exists(&self.path) {
            file = fs::OpenOptions::new()
                .append(true)
                .open(&self.path)?;
        } else {
            fs::create_dir_all(&self.path.parent().unwrap())?;
            file = File::create(&self.path)?;
        }

        write!(file, "{}", message)?;

        Ok(())
    }

    pub fn rotate_if_needed(&self) {
        if !Path::exists(&self.path) { return }

        let metadata = fs::metadata(&self.path).expect("cannot get log metadata");
        if metadata.len() >= self.config.max_file_size {
            self.rotate_logs().unwrap_or_else(|err| panic!("cannot rotate logs: {}", err));
        }
    }

    fn rotate_logs(&self) -> Result<(), std::io::Error> {
        let ext = self.path.extension().unwrap().to_str().unwrap();
        for i in (1..self.config.rotation_count).rev() {
            let old_path = self.path.with_extension(format!("{}.{}", i-1, ext));
            let new_path = self.path.with_extension(format!("{}.{}", i, ext));

            println!("i: {}, old: {:?}, new: {:?}", i, old_path, new_path);

            if old_path.exists() {
                fs::rename(old_path, new_path)?;
            }
        }

        fs::rename(&self.path, self.path.with_extension(format!("{}.{}", "0", ext)))?;

        Ok(())
    }
}
