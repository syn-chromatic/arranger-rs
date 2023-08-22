use std::io;
use std::str::Lines;

use crate::general::terminal::Terminal;
use crate::general::terminal::{CyanANSI, GreenANSI, RedANSI};
use crate::general::terminal::{WhiteANSI, YellowANSI};

pub struct ConfirmationPrompt;

impl ConfirmationPrompt {
    pub fn prompt(terminal: &Terminal) -> bool {
        let mut input: String = String::new();

        let string: &str = "\nDo you want to continue? [y/N]: ";
        terminal.write_color(string, &CyanANSI);

        match io::stdin().read_line(&mut input) {
            Ok(_) => Self::process_input(terminal, &input),
            Err(error) => Self::handle_error(terminal, error),
        }
    }
}

impl ConfirmationPrompt {
    fn process_input(terminal: &Terminal, input: &str) -> bool {
        if input.trim() == "y" || input.trim() == "Y" {
            let string: &str = "Continuing...\n\n";
            terminal.writeln_color(string, &GreenANSI);
            return true;
        } else {
            let string: &str = "Not continuing...\n";
            terminal.writeln_color(string, &RedANSI);
            return false;
        }
    }

    fn handle_error(terminal: &Terminal, error: io::Error) -> bool {
        let string: String = format!("Failed to read line: {}\n", error);
        terminal.writeln_color(&string, &RedANSI);
        false
    }
}

pub struct StringOp;

impl StringOp {
    pub fn substring(s: &str, start: usize, end: usize) -> Option<&str> {
        let start_byte: usize = s.char_indices().nth(start)?.0;
        let end_byte: usize = s.char_indices().nth(end)?.0;
        Some(&s[start_byte..end_byte])
    }

    pub fn split_retain_delim_left<'a>(
        s: &'a str,
        delimiter: &'a str,
    ) -> Option<(&'a str, &'a str)> {
        if !s.is_empty() && !delimiter.is_empty() {
            return s.find(delimiter).map(|index| {
                let first: &str = &s[..index + delimiter.len()];
                let second: &str = &s[index + delimiter.len()..];
                (first, second)
            });
        }
        None
    }
}

pub struct OptionsPrinter {
    terminal: Terminal,
}

impl OptionsPrinter {
    pub fn new() -> Self {
        OptionsPrinter {
            terminal: Terminal::new(),
        }
    }

    pub fn print(&self, opt_string: &str) {
        let lines: Lines = opt_string.lines();
        for (idx, line) in lines.enumerate() {
            if self.is_non_empty_alphabetic_line(&line) {
                self.handle_alphabetic_line(&line, idx);
            } else {
                self.terminal.writeln_color(&line, &WhiteANSI);
            }
        }
    }
}

impl OptionsPrinter {
    fn is_non_empty_alphabetic_line(&self, line: &str) -> bool {
        if !line.is_empty() {
            if line.chars().next().map_or(false, |c| c.is_alphabetic()) {
                return true;
            }
        }
        false
    }

    fn handle_alphabetic_line(&self, line: &str, idx: usize) {
        let split_line: Option<(&str, &str)> = StringOp::split_retain_delim_left(&line, ":");

        if let Some(split) = split_line {
            let parts: [&str; 2] = [split.0, split.1];
            if split.1.is_empty() {
                self.terminal.writeln_parameter(&parts, &YellowANSI);
            } else {
                self.terminal.writeln_parameter(&parts, &RedANSI);
            }
        } else if idx == 0 {
            self.terminal.writeln_color(&line, &GreenANSI);
        }
    }
}
