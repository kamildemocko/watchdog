use std::path::PathBuf;

#[derive(Debug, serde::Serialize)]
pub struct LogEntry<'a> {
    pub event: &'a str,
    pub pid: u32,
    pub name: &'a str,
    pub cmd: &'a str,
    pub timestamp: i64,
    pub start_time: i64,
    pub seconds: u32,
}

pub trait Logger {
    fn new(path: PathBuf) -> Self;
    fn log_item(&self, entry: LogEntry);
}
