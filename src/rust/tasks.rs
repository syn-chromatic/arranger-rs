use std::fs;
use std::io::Error;

use serde::Serialize;

use crate::general::path::WPath;
use crate::general::terminal::Terminal;
use crate::general::terminal::{CyanANSI, GreenANSI, RedANSI};

#[derive(Serialize)]
struct TaskGroup {
    kind: String,
    isDefault: bool,
}

#[derive(Serialize)]
struct Task {
    #[serde(rename = "type")]
    task_type: String,
    command: String,
    problemMatcher: Vec<String>,
    label: String,
    group: TaskGroup,
}

#[derive(Serialize)]
struct Config {
    version: String,
    tasks: Vec<Task>,
}

fn get_rust_run_task() -> String {
    let config: Config = Config {
        version: "2.0.0".to_string(),
        tasks: vec![Task {
            task_type: "shell".to_string(),
            command: "cargo run --release".to_string(),
            problemMatcher: vec!["$rustc".to_string()],
            label: "rust: cargo run".to_string(),
            group: TaskGroup {
                kind: "build".to_string(),
                isDefault: true,
            },
        }],
    };

    let json: String = serde_json::to_string_pretty(&config).unwrap();
    json
}

pub fn generate_rust_run_task() {
    let terminal: Terminal = Terminal::new();
    let json: String = get_rust_run_task();
    let path: WPath = WPath::from_string("./.vscode/tasks.json");
    let path_dir: WPath = WPath::from_string("./.vscode/");

    if !path_dir.exists() {
        terminal.writeln_color("Creating Directory Structure..", CyanANSI);
        let result: Result<(), Error> = fs::create_dir_all(&path_dir);

        if let Err(error) = result {
            let parts: [&str; 2] = ["Error: ", &error.to_string()];
            terminal.writeln_2p_primary(&parts, RedANSI);
            return;
        }
    }

    let mut path_string: String = format!("{:?}", path);

    if !path.exists() {
        let result: Result<(), Error> = fs::write(&path, json);
        if let Err(error) = result {
            let parts: [&str; 2] = ["Error: ", &error.to_string()];
            terminal.writeln_2p_primary(&parts, RedANSI);
        } else {
            path_string = format!("{:?}", path);
            let parts: [&str; 2] = ["File Generated: ", &path_string];
            terminal.writeln_2p_primary(&parts, GreenANSI);
        }
        return;
    }

    let parts: [&str; 2] = ["File already exists: ", &path_string];
    terminal.writeln_2p_primary(&parts, RedANSI);
}
