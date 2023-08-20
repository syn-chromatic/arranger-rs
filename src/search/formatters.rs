use chrono::{DateTime, Local};
use std::time::SystemTime;

pub fn format_size(bytes: u64) -> String {
    const KB: f64 = (1u64 << 10) as f64;
    const MB: f64 = (1u64 << 20) as f64;
    const GB: f64 = (1u64 << 30) as f64;
    const TB: f64 = (1u64 << 40) as f64;

    let bytes: f64 = bytes as f64;
    match bytes {
        _ if bytes <= KB => format!("{:.2} B", bytes),
        _ if bytes < MB => format!("{:.2} KB", bytes / KB),
        _ if bytes < GB => format!("{:.2} MB", bytes / MB),
        _ if bytes < TB => format!("{:.2} GB", bytes / GB),
        _ => format!("{:.2} TB", bytes / TB),
    }
}

pub fn format_time(nanoseconds: u128) -> String {
    const US: f64 = 1_000.0;
    const MS: f64 = 1_000_000.0;
    const S: f64 = 1_000_000_000.0;

    let nanoseconds: f64 = nanoseconds as f64;
    match nanoseconds {
        _ if nanoseconds < US => format!("{:.2} ns", nanoseconds),
        _ if nanoseconds < MS => format!("{:.2} µs", nanoseconds / US),
        _ if nanoseconds < S => format!("{:.2} ms", nanoseconds / MS),
        _ => format!("{:.2} s", nanoseconds / S),
    }
}

pub fn format_system_time(time: SystemTime, fmt: &str) -> String {
    let date_time: DateTime<Local> = DateTime::<Local>::from(time);
    let string: String = date_time.format(fmt).to_string();
    string
}
