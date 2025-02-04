use std::path::PathBuf;

#[derive(Debug, serde::Serialize)]
pub struct LogEntry<'a> {
    pub event: &'a str,
    pub pid: u32,
    pub start_time: u64,
    pub name: &'a str,
    pub cmd: &'a str,
}

pub trait Logger {
    fn new(path: PathBuf, sep: char) -> Self;
    fn log_item(&mut self, entry: LogEntry);
}
