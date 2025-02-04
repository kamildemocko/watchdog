use std::{fs::{self, File}, path::{Path, PathBuf}};
use std::io::Write;
use std::error::Error;

use csv::WriterBuilder;

use crate::logger::{Logger,LogEntry};

pub struct CSVLogger {
    path: PathBuf,
    exists: bool,
    sep: char,
}

impl Logger for CSVLogger {
    fn new(path: PathBuf, sep: char) -> Self {
        let file_exists = Path::exists(&path);
        println!("log file exists: {}", file_exists);

        CSVLogger { path: path, sep: sep, exists: file_exists}
    }

    fn log_item(&mut self, entry: LogEntry) {
        let message = self.prepare_log_entry(entry).expect("cannot prepare log message");
        self.write_log(message).expect("cannot write log file to fs");
        }
}

impl CSVLogger {
    fn prepare_log_entry(&self, entry: LogEntry) -> Result<String, Box<dyn Error>> {
        let mut wrt = WriterBuilder::new()
            .has_headers(!self.exists)
            .delimiter(self.sep as u8)
            .from_writer(vec![]);
        wrt.serialize(entry)?;

        let data = String::from_utf8(wrt.into_inner()?)?;
        Ok(data)
    }

    fn write_log(&mut self, message: String) -> Result<(), Box<dyn Error>> {
        let mut file: fs::File;
        if self.exists {
            file = fs::OpenOptions::new()
                .append(true)
                .open(&self.path)?;
        } else {
            fs::create_dir_all(&self.path.parent().unwrap())?;
            file = File::create(&self.path)?;
            self.exists = true
        }

        write!(file, "{}", message)?;

        Ok(())
    }
}