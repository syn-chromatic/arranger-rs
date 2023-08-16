use std::error::Error;
use std::io;
use std::io::Read;
use std::process::{Child, ChildStderr, ChildStdout};
use std::process::{Command, ExitStatus};
use std::process::{Output, Stdio};
use std::str;
use std::string::FromUtf8Error;
use std::thread;

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
                let string: String = format!("{}", self.stdout.trim());
                self.terminal.writeln_color(&string, &GreenANSI);
                self.terminal.writeln_color(&separator, &YellowANSI);
                return;
            }
        }

        let string: String = format!("{}", self.stderr.trim());
        self.terminal.writeln_color(&string, &RedANSI);
        self.terminal.writeln_color(&separator, &YellowANSI);
    }
}

pub struct CommandExecute;

impl CommandExecute {
    pub fn new() -> Self {
        CommandExecute
    }

    pub fn execute_command(&self, program: &WPath, args: &[&str]) -> Option<CommandResponse> {
        let output: Result<Output, io::Error> = Command::new(program).args(args).output();
        if let Ok(output) = output {
            let response: Option<CommandResponse> = CommandResponse::new(output);
            return response;
        }
        None
    }

    pub fn execute_spawn_command(&self, program: &WPath, args: &[&str]) {
        let spawn: Result<Child, io::Error> = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let terminal = Terminal::new();

        if let Ok(mut spawn) = spawn {
            let stdout_func = move |stdout| Self::write_stdout_buffer(stdout);
            let stderr_func = move |stderr| Self::write_stderr_buffer(stderr);

            let stdout_handle: Option<thread::JoinHandle<()>> =
                self.spawn_thread_for_io(spawn.stdout.take(), stdout_func);

            let stderr_handle: Option<thread::JoinHandle<()>> =
                self.spawn_thread_for_io(spawn.stderr.take(), stderr_func);

            stdout_handle.map(|handle| handle.join().unwrap());
            stderr_handle.map(|handle| handle.join().unwrap());

            let separator: String = "-".repeat(10);
            terminal.writeln_color(&separator, &YellowANSI);
        }
    }
}

impl CommandExecute {
    fn spawn_thread_for_io<T, F>(
        &self,
        stream: Option<T>,
        func: F,
    ) -> Option<thread::JoinHandle<()>>
    where
        F: Fn(T) + Send + 'static,
        T: Send + 'static,
    {
        stream.map(|s| {
            thread::spawn(move || {
                func(s);
            })
        })
    }

    fn write_stdout_buffer(mut stdout: ChildStdout) {
        let terminal: Terminal = Terminal::new();
        let mut buffer: [u8; 256] = [0; 256];

        while let Ok(size) = stdout.read(&mut buffer) {
            if size == 0 {
                break;
            }

            let string: Result<&str, str::Utf8Error> = str::from_utf8(&buffer[..size]);
            if let Ok(string) = string {
                terminal.write_color(string, &GreenANSI);
            } else {
                let error: str::Utf8Error = string.unwrap_err();
                Self::write_spawn_error(Box::new(error), &terminal);
                break;
            }
        }
    }

    fn write_stderr_buffer(mut stderr: ChildStderr) {
        let terminal: Terminal = Terminal::new();
        let mut buffer: [u8; 256] = [0; 256];

        while let Ok(size) = stderr.read(&mut buffer) {
            if size == 0 {
                break;
            }

            let string: Result<&str, str::Utf8Error> = str::from_utf8(&buffer[..size]);
            if let Ok(string) = string {
                terminal.write_color(string, &RedANSI);
            } else {
                let error: str::Utf8Error = string.unwrap_err();
                Self::write_spawn_error(Box::new(error), &terminal);
                break;
            }
        }
    }

    fn write_spawn_error(error: Box<dyn Error>, terminal: &Terminal) {
        let error_string: String = error.to_string();
        terminal.writeln_color(&error_string, &RedANSI);
    }
}
