use std::io::Error;
use std::process::Output;
use std::process::{Command, ExitStatus};
use std::string::FromUtf8Error;

use crate::general::path::WPath;
use crate::general::terminal::Terminal;
use crate::general::terminal::{GreenANSI, RedANSI, YellowANSI};

pub struct CommandResponse {
    stdout: String,
    stderr: String,
    status: ExitStatus,
    terminal: Terminal,
}

impl CommandResponse {
    pub fn new(output: Output) -> Option<Self> {
        let stdout: Result<String, FromUtf8Error> = String::from_utf8(output.stdout);
        let stderr: Result<String, FromUtf8Error> = String::from_utf8(output.stderr);
        let status: ExitStatus = output.status;

        if let (Ok(stdout), Ok(stderr)) = (stdout, stderr) {
            let stdout: String = stdout.trim().to_string();
            let stderr: String = stderr.trim().to_string();
            let terminal: Terminal = Terminal::new();

            let response: CommandResponse = CommandResponse {
                stdout,
                stderr,
                status,
                terminal,
            };

            return Some(response);
        }
        None
    }

    pub fn get_stdout(&self) -> &str {
        &self.stdout
    }

    pub fn get_stderr(&self) -> &str {
        &self.stderr
    }

    pub fn get_status(&self) -> &ExitStatus {
        &self.status
    }

    pub fn print(&self) {
        let exit_code: Option<i32> = self.status.code();
        let separator: String = "-".repeat(10);
        if let Some(exit_code) = exit_code {
            if exit_code == 0 && self.stderr.is_empty() {
                let string: String = format!("{}", self.stdout);
                self.terminal.writeln_color(&string, GreenANSI);
                self.terminal.writeln_color(&separator, YellowANSI);
                println!();
                return;
            }
        }

        let string: String = format!("{}", self.stderr);
        self.terminal.writeln_color(&string, RedANSI);
        self.terminal.writeln_color(&separator, YellowANSI);
        println!();
    }
}

pub struct CommandExecute;

impl CommandExecute {
    pub fn new() -> Self {
        CommandExecute
    }

    pub fn execute_command(&self, program: &WPath, args: &[&str]) -> Option<CommandResponse> {
        let output: Result<Output, Error> = Command::new(program).args(args).output();
        if let Ok(output) = output {
            let response: Option<CommandResponse> = CommandResponse::new(output);
            return response;
        }
        None
    }
}
