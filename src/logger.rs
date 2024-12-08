use std::fs::{self, OpenOptions};
use std::io::Write;

use chrono::Local;

const LOG_DIR: &str = "./logs";
const LOG_FILE: &str = "./logs/watchdog.log";
const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10 MB
const BACKUP_COUNT: usize = 5;

fn rotate_logs() {
    let metadata = match fs::metadata(LOG_FILE) {
        Ok(m) => m,
        Err(_) => return,
    };

    if metadata.len() > MAX_LOG_SIZE {
        // remove oldest file
        let oldest_file = format!("{}.{}", LOG_FILE, BACKUP_COUNT);
        if fs::metadata(&oldest_file).is_ok() {
            fs::remove_file(oldest_file).expect("Failed to remove oldest log file")
        }

        // shift names
        for i in (1..=BACKUP_COUNT-1).rev() {
            let src = format!("{}.{}", LOG_FILE, i);
            let dst = format!("{}.{}", LOG_FILE, i + 1);
            if fs::metadata(&src).is_err() {
                continue;
            }

            fs::rename(src, dst).expect("Failed to rename old log file")
        }

        let backup = format!("{}.1", LOG_FILE);
        fs::rename(LOG_FILE, backup).expect("Failed to backup current log");
    }
}

pub fn log_message(message: &str) {
    let _ = fs::create_dir(LOG_DIR);
    rotate_logs();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .expect("Failed to open log file");

    println!("{}", message);
    writeln!(file, "{}", message).expect("Failed to write to log file");
}

pub fn prepare_log_message(event: &str, pid: u32, start_time: u64, name: &str, cmd: &str) -> String {
    format!(
        r#"{{"event": {}, "timestamp": {}, "pid": {}, "start_time": {}, "process_name": {}, "cmd": {:?}}}"#,
        event,
        &Local::now().to_rfc3339(),
        pid,
        start_time,
        name, 
        cmd,
    )
}
