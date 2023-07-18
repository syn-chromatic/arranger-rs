use std::io::Error;
use std::path::PathBuf;
use std::process::Output;
use std::process::{Command, ExitStatus};
use std::string::FromUtf8Error;

pub struct CommandR {
    stdout: String,
    stderr: String,
    status: ExitStatus,
}

impl CommandR {
    pub fn new(output: Output) -> Option<Self> {
        let stdout: Result<String, FromUtf8Error> = String::from_utf8(output.stdout);
        let stderr: Result<String, FromUtf8Error> = String::from_utf8(output.stderr);
        let status: ExitStatus = output.status;

        if let (Ok(stdout), Ok(stderr)) = (stdout, stderr) {
            let response: CommandR = CommandR {
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

impl core::fmt::Debug for CommandR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = format!(
            "# STDOUT #\n{}\n# STDERR #\n{}\n# STATUS #\n{}",
            self.stdout, self.stderr, self.status
        );

        write!(f, "{}", string)
    }
}

pub struct CommandE;

impl CommandE {
    pub fn new() -> Self {
        CommandE
    }

    pub fn execute_command(&self, program: &PathBuf, args: &[&str]) -> Option<CommandR> {
        let output: Result<Output, Error> = Command::new(program).args(args).output();
        if let Ok(output) = output {
            let response: Option<CommandR> = CommandR::new(output);
            return response;
        }
        None
    }
}
