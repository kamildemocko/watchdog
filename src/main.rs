use std::{collections::HashMap, thread, time::Duration};

use sysinfo::{Pid, ProcessesToUpdate, RefreshKind, System};

mod models;
mod logger;
mod tools;

use models::{load_settings_file, ProcessInfo};
use logger::{log_message, prepare_log_message};


fn main() {
    let settings = load_settings_file("settings.toml");
    let mut system = System::new_all();
    let mut monitored_processes: HashMap<u32, ProcessInfo> = HashMap::new();

    println!("Starting process watchdog");

    loop {
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
                        cmd: process.cmd().iter().map(|i| i.to_string_lossy()).collect::<Vec<_>>().join(" ")
                };

                log_message(
                    &settings.log_path, 
                    &prepare_log_message("start", 
                    pid, 
                    process.start_time(), 
                    name, 
                    &process_info.cmd)
                );

                monitored_processes.insert(
                    process_info.pid, 
                    process_info,
                );
            }
        }

        // retain only existing processes
        monitored_processes.retain(|&pid, inf| {
            if system.process(Pid::from_u32(pid)).is_none() {
                log_message(
                    &settings.log_path, 
                    &prepare_log_message("end", 
                    pid, 
                    inf.start_time, 
                    &inf.name, 
                    &inf.cmd)
                );
                false
            } else {
                true
            }
        });

        thread::sleep(Duration::from_secs(1));
    }
}
