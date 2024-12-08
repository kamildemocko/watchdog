# Watchdog

Watchdog is a Rust application that monitors specific processes on your system and logs their start and end events. It uses the `sysinfo` crate to gather process information and logs the events to a file.

## Dependencies

- `chrono`: For handling date and time.
- `serde`: For serializing and deserializing data.
- `sysinfo`: For retrieving system information.
- `toml`: For parsing the settings file.

## Configuration

The application reads its configuration from a `settings.toml` file located in the root directory. The file should contain a list of process names to monitor:

```toml
process_names = ["python.exe"]
```

## Usage
To run the application, use the following command:
```cmd
cargo run
```

The application will start monitoring the processes specified in the settings.toml file and log their start and end events to watchdog.log.

## Logging
The log file watchdog.log (JSONL) contains JSON-formatted entries for each process start and end event. Each entry includes the event type, timestamp, process ID, start time, process name, and command line.
Log is located at logs folder

Example log entry:
```plain
{"event": start, "timestamp": 2024-12-08T18:37:43.724121600+01:00, "pid": 1412, "start_time": 1733679463, "process_name": python.exe, "cmd": "C:\\Users\\kamil\\AppData\\Local\\Programs\\Python\\Python312\\python.exe F:\\Development\\Python\\Mood.py"}
{"event": end, "timestamp": 2024-12-08T18:37:50.954438400+01:00, "pid": 1412, "start_time": 1733679463, "process_name": python.exe, "cmd": "C:\\Users\\kamil\\AppData\\Local\\Programs\\Python\\Python312\\python.exe F:\\Development\\Python\\Mood.py"}
```