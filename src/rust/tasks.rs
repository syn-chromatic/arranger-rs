use std::fs;
use std::io;

use serde::Serialize;
use serde_json::Value;

use crate::general::path::WPath;
use crate::general::terminal::Terminal;
use crate::general::terminal::{CyanANSI, GreenANSI, RedANSI};

#[derive(Serialize)]
struct TaskGroup {
    kind: String,
    #[serde(rename = "isDefault")]
    is_default: bool,
}

#[derive(Serialize)]
struct DefaultTask {
    #[serde(rename = "type")]
    task_type: String,
    command: String,
    #[serde(rename = "problemMatcher")]
    problem_matcher: Vec<String>,
    label: String,
    group: TaskGroup,
}

#[derive(Serialize)]
struct Task {
    #[serde(rename = "type")]
    task_type: String,
    command: String,
    #[serde(rename = "problemMatcher")]
    problem_matcher: Vec<String>,
    label: String,
    group: String,
}

#[derive(Serialize)]
struct DefaultConfig {
    version: String,
    tasks: Vec<DefaultTask>,
}

pub struct RustVSCodeTask {
    path: WPath,
    terminal: Terminal,
}

impl RustVSCodeTask {
    pub fn new() -> Self {
        let path: WPath = WPath::from_string("./.vscode/tasks.json");
        let terminal: Terminal = Terminal::new();
        RustVSCodeTask { path, terminal }
    }

    pub fn generate_run_task(&self) {
        let json: String = self.get_run_task_json();
        let path_dir: WPath = WPath::from_string("./.vscode/");

        if !path_dir.exists() {
            self.terminal
                .writeln_color("[Creating Directory Structure]", &CyanANSI);
            let result: Result<(), io::Error> = fs::create_dir_all(&path_dir);

            if let Err(error) = result {
                let parts: [&str; 2] = ["Error: ", &error.to_string()];
                self.terminal.writeln_parameter(&parts, &RedANSI);
                return;
            }
        }

        if !self.path.exists() {
            let result: Result<(), io::Error> = fs::write(&self.path, json);
            if let Err(error) = result {
                let parts: [&str; 2] = ["Error: ", &error.to_string()];
                self.terminal.writeln_parameter(&parts, &RedANSI);
            } else {
                let string: &str = "New tasks file has been created.";
                self.terminal.writeln_color(string, &GreenANSI);

                let path_string = format!("{:?}", &self.path);
                let parts: [&str; 2] = ["File: ", &path_string];
                self.terminal.writeln_parameter(&parts, &GreenANSI);
            }
        } else {
            let label: &str = "rust: cargo run";
            let is_present: Result<bool, io::Error> = self.is_task_label_present(label);
            if let Ok(is_present) = is_present {
                if !is_present {
                    let result: Result<(), io::Error> = self.write_run_task();
                    if let Err(error) = result {
                        let parts: [&str; 2] = ["Error: ", &error.to_string()];
                        self.terminal.writeln_parameter(&parts, &RedANSI);
                    } else {
                        let string: &str = "Run task has been added to [tasks.json]";
                        self.terminal.writeln_color(string, &GreenANSI);
                    }
                } else {
                    let string: &str = "Run task already exists in [tasks.json].";
                    self.terminal.writeln_color(string, &RedANSI);
                }
            } else {
                let error: io::Error = is_present.unwrap_err();
                let parts: [&str; 2] = ["Error: ", &error.to_string()];
                self.terminal.writeln_parameter(&parts, &RedANSI);
            }
        }
    }

    fn get_run_task_json(&self) -> String {
        let config: DefaultConfig = DefaultConfig {
            version: "2.0.0".to_string(),
            tasks: vec![DefaultTask {
                task_type: "shell".to_string(),
                command: "cargo run --release".to_string(),
                problem_matcher: vec!["$rustc".to_string()],
                label: "rust: cargo run".to_string(),
                group: TaskGroup {
                    kind: "build".to_string(),
                    is_default: true,
                },
            }],
        };

        let json: String = serde_json::to_string_pretty(&config).unwrap();
        json
    }

    fn get_run_task(&self) -> Value {
        let task: Task = Task {
            task_type: "shell".to_string(),
            command: "cargo run --release".to_string(),
            problem_matcher: vec!["$rustc".to_string()],
            label: "rust: cargo run".to_string(),
            group: "build".to_string(),
        };
        let value: Value = serde_json::to_value(&task).unwrap();
        value
    }

    fn write_run_task(&self) -> Result<(), io::Error> {
        let file: fs::File = fs::File::open(&self.path)?;
        let reader: io::BufReader<fs::File> = io::BufReader::new(file);
        let mut json: serde_json::Value = serde_json::from_reader(reader)?;
        let tasks: &mut Value = &mut json["tasks"];
        if tasks.is_array() {
            let tasks_array: &mut Vec<Value> = tasks.as_array_mut().unwrap();
            let run_task: Value = self.get_run_task();
            tasks_array.push(run_task);
        }
        let json_string: Result<String, serde_json::Error> = serde_json::to_string_pretty(&json);
        if let Ok(json_string) = json_string {
            let result: Result<(), io::Error> = fs::write(&self.path, json_string);
            return result;
        }
        Ok(())
    }

    fn is_task_label_present(&self, label: &str) -> Result<bool, io::Error> {
        let file: fs::File = fs::File::open(&self.path)?;
        let reader: io::BufReader<fs::File> = io::BufReader::new(file);
        let json: serde_json::Value = serde_json::from_reader(reader)?;
        let tasks: &Value = &json["tasks"];
        if tasks.is_array() {
            let tasks_array = tasks.as_array().unwrap();

            for task in tasks_array {
                let task_label: &Value = &task["label"];
                if task_label.is_string() && task_label == label {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}
