use std::{collections::HashMap, path::PathBuf, thread, time::Duration};

use logger::Logger;
use sysinfo::{Pid, ProcessesToUpdate, RefreshKind, System};

mod models;
mod logger;
mod logger_csv;

use models::{load_settings_file, ProcessInfo};
use logger::LogEntry;
use logger_csv::CSVLogger;


fn main() {
    let settings = load_settings_file("settings.toml");
    let logger = CSVLogger::new(PathBuf::from(&settings.log_path)).with_delimeter(';');
    let mut system = System::new_all();
    let mut monitored_processes: HashMap<u32, ProcessInfo> = HashMap::new();

    println!("Watching for processes {:?}", settings.process_names);

    loop {
        let now = chrono::Local::now().timestamp();

        system.refresh_processes(ProcessesToUpdate::All, true);
        system.refresh_specifics(RefreshKind::everything());

        // add to monitored processes if not already there and keyword is in path
        for (pid, process) in system.processes() {
            let pid = pid.as_u32();

            if monitored_processes.contains_key(&pid) {
                continue;
            }

            let name = process.name().to_str().unwrap_or("");
            if settings.process_names.iter().any(|proc_name| proc_name == name) {
                if process.cmd().len() == 0 {
                    break;
                }

                let process_info = ProcessInfo{
                        pid: pid, 
                        start_time: process.start_time(), 
                        name: name.to_string(), 
                        cmd: process.cmd()
                            .iter()
                            .map(|i| i.to_string_lossy())
                            .map(|i| i.replace("\n", " "))
                            .map(|i| i.replace("\t", " "))
                            .collect::<Vec<_>>()
                            .join(" ")
                };

                logger.log_item(LogEntry{
                    event: "start",
                    pid: pid,
                    name: name,
                    cmd: &process_info.cmd,
                    timestamp: now,
                    start_time: process.start_time() as i64,
                    seconds: (now - process.start_time() as i64) as u32
                });
                println!(" \u{00BB} Add process:\t{} ({})", name, pid);

                monitored_processes.insert(
                    process_info.pid, 
                    process_info,
                );
            }
        }

        // retain only existing processes
        monitored_processes.retain(|&pid, inf| {
            if system.process(Pid::from_u32(pid)).is_none() {
                logger.log_item(LogEntry{
                    event: "end",
                    pid: pid,
                    name: &inf.name,
                    cmd: &inf.cmd,
                    timestamp: now,
                    start_time: inf.start_time as i64,
                    seconds: (now - inf.start_time as i64) as u32
                });
                println!(" \u{00BB} Rem process:\t{} ({})", &inf.name, pid);

                false
            } else {
                true
            }
        });

        logger.rotate_if_needed();

        thread::sleep(Duration::from_secs(1));
    }
}
