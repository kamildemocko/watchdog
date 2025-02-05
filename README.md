# Watchdog

Watchdog is a Rust application that monitors specific processes on your system and logs their start and end events. It uses the `sysinfo` crate to gather process information and logs the events to a file.

## Dependencies

- `chrono`: For handling date and time.
- `serde`: For serializing and deserializing data.
- `sysinfo`: For retrieving system information.
- `toml`: For parsing the settings file.
- `csv`: For writing CSV file

## Configuration

The application reads its configuration from a `settings.toml` file located in the root directory. The file should contain a list of process names to monitor:

```toml
process_names = ["anki"]
log_path = "./logs/watchdog.csv"
```

## Usage
To run the application, use the following command:
```cmd
cargo run
```

The application will start monitoring the processes specified in the settings.toml file and log their start and end events to watchdog.log.

## Logging
The log file watchdog.csv contains entries for each process start and end event. Each entry includes the event type, timestamp, process ID, start time, process name, and command line.  

Log is located at logs folder  

Set delimeter with `loggerinstance.with_delimeter(";")` and set custom settings (f.e.: log rotation config) with `loggerinstance.with_config(LogConfig{max_file_size: 10 * 1024 * 1024, rotation_count: 5})`  

Example log entry:
```csv
event;pid;start_time;name;cmd
start;13319;1738739816;anki;/Applications/Anki.app/Contents/MacOS/anki
end;13319;1738739816;anki;/Applications/Anki.app/Contents/MacOS/anki
```
