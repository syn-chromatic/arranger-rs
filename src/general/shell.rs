use std::io::Error;
use std::process::Output;
use std::process::{Command, ExitStatus};
use std::string::FromUtf8Error;

use core::fmt::{Debug, Formatter};

use crate::general::path::AbPath;

pub struct CommandResponse {
    stdout: String,
    stderr: String,
    status: ExitStatus,
}

impl CommandResponse {
    pub fn new(output: Output) -> Option<Self> {
        let stdout: Result<String, FromUtf8Error> = String::from_utf8(output.stdout);
        let stderr: Result<String, FromUtf8Error> = String::from_utf8(output.stderr);
        let status: ExitStatus = output.status;

        if let (Ok(stdout), Ok(stderr)) = (stdout, stderr) {
            let stdout: String = stdout.trim().to_string();
            let stderr: String = stderr.trim().to_string();

            let response: CommandResponse = CommandResponse {
                stdout,
                stderr,
                status,
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
}

impl Debug for CommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let separator: String = "=".repeat(30);
        let string: String = format!(
            "{}\n{}\n{}\n{}\n{}\n",
            separator, self.stdout, self.stderr, self.status, separator,
        );
        f.write_str(&string)
    }
}

pub struct CommandExecute;

impl CommandExecute {
    pub fn new() -> Self {
        CommandExecute
    }

    pub fn execute_command(&self, program: &AbPath, args: &[&str]) -> Option<CommandResponse> {
        let output: Result<Output, Error> = Command::new(program).args(args).output();
        if let Ok(output) = output {
            let response: Option<CommandResponse> = CommandResponse::new(output);
            return response;
        }
        None
    }
}
