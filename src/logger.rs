use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::Local;

const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10 MB
const BACKUP_COUNT: usize = 5;

fn rotate_logs(log_path: &str) {
    let metadata = match fs::metadata(log_path) {
        Ok(m) => m,
        Err(_) => return,
    };

    if metadata.len() > MAX_LOG_SIZE {
        // remove oldest file
        let oldest_file = format!("{}.{}", log_path, BACKUP_COUNT);
        if fs::metadata(&oldest_file).is_ok() {
            fs::remove_file(oldest_file).expect("Failed to remove oldest log file")
        }

        // shift names
        for i in (1..=BACKUP_COUNT-1).rev() {
            let src = format!("{}.{}", log_path, i);
            let dst = format!("{}.{}", log_path, i + 1);
            if fs::metadata(&src).is_err() {
                continue;
            }

            fs::rename(src, dst).expect("Failed to rename old log file")
        }

        let backup = format!("{}.1", log_path);
        fs::rename(log_path, backup).expect("Failed to backup current log");
    }
}

pub fn log_message(log_path: &str, message: &str) {
    let _ = fs::create_dir_all(PathBuf::from_str(log_path).unwrap().parent().unwrap());
    rotate_logs(log_path);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
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
