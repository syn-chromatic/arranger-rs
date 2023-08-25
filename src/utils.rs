use std::fmt;
use std::io;
use std::str::Lines;

use crate::general::terminal::ANSICode;
use crate::general::terminal::Terminal;
use crate::general::terminal::{BlackANSI, WhiteANSI, YellowANSI};
use crate::general::terminal::{CombinedANSI, YellowBackgroundANSI};
use crate::general::terminal::{CyanANSI, GreenANSI, RedANSI};

pub struct ConfirmationPrompt;

impl ConfirmationPrompt {
    pub fn prompt(terminal: &Terminal) -> bool {
        let mut input: String = String::new();

        let string: &str = "\nDo you want to continue? [y/N]: ";
        terminal.write_ansi(string, &CyanANSI);

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
            terminal.writeln_ansi(string, &GreenANSI);
            return true;
        } else {
            let string: &str = "Not continuing...\n";
            terminal.writeln_ansi(string, &RedANSI);
            return false;
        }
    }

    fn handle_error(terminal: &Terminal, error: io::Error) -> bool {
        let string: String = format!("Failed to read line: {}\n", error);
        terminal.writeln_ansi(&string, &RedANSI);
        false
    }
}

pub struct StringOp;

impl StringOp {
    #[allow(dead_code)]
    pub fn substring(s: &str, start: usize, end: usize) -> Option<&str> {
        let start_byte: usize = s.char_indices().nth(start)?.0;
        let end_byte: usize = s.char_indices().nth(end)?.0;
        Some(&s[start_byte..end_byte])
    }

    pub fn trim_with_ellipsis(s: &str, length: usize, ellipsis: usize) -> String {
        if s.len() <= length {
            return s.to_string();
        }
        let trimmed = &s[..s
            .char_indices()
            .nth(length - ellipsis)
            .map_or(s.len(), |(i, _)| i)];

        let s: String = format!("{}{}", trimmed, ".".repeat(ellipsis));
        s
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
                self.terminal.writeln_ansi(&line, &WhiteANSI);
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
            self.terminal.writeln_ansi(&line, &GreenANSI);
        }
    }
}

pub struct ParametersPrinter {
    terminal: Terminal,
    header: String,
    parameters: Vec<(String, String)>,
    delimiter_spacing: usize,
}

impl ParametersPrinter {
    pub fn new(delimiter_spacing: usize) -> Self {
        let terminal: Terminal = Terminal::new();
        let header: String = String::new();
        let parameters: Vec<(String, String)> = Vec::new();
        ParametersPrinter {
            terminal,
            header,
            parameters,
            delimiter_spacing,
        }
    }

    pub fn set_header(&mut self, header: &str) {
        self.header = header.to_string();
    }

    pub fn add_parameter<T: fmt::Debug>(&mut self, parameter: (&str, T)) {
        let parameter_a: String = parameter.0.to_string();
        let parameter_b: String = format!("{:?}", parameter.1);
        self.parameters.push((parameter_a, parameter_b));
    }

    pub fn print_parameters(&self) {
        let attribute_length: usize = self.get_max_attribute_length() + self.delimiter_spacing;

        let padded_header: String = self.get_padded_header(attribute_length);
        let header_ansi: CombinedANSI = self.get_header_ansi();
        self.terminal.writeln_ansi(&padded_header, &header_ansi);

        for parameter in &self.parameters {
            let padded_attribute: String = self.pad_right_to_length(&parameter.0, attribute_length);
            self.terminal.write_ansi(&padded_attribute, &YellowANSI);
            let delimiter: String = self.get_delimiter();
            let padded_value: String = delimiter + &parameter.1;
            self.terminal.write(&padded_value);
            println!();
        }
        println!();
    }
}

impl ParametersPrinter {
    fn get_max_attribute_length(&self) -> usize {
        let max_length: usize = self
            .parameters
            .iter()
            .map(|(s, _)| s.len())
            .max()
            .unwrap_or(0);
        max_length
    }

    fn get_header_padding_length(&self, max_length: usize) -> usize {
        if self.header.len() > 0 {
            let halved_header: usize = (self.header.len() as f32 / 2.0).ceil() as usize;
            if (max_length + 1) > halved_header {
                let padding_length: usize = (max_length + 1) - halved_header;
                return padding_length;
            }
        }
        return 1;
    }

    fn get_padded_header(&self, attribute_length: usize) -> String {
        let padding_length: usize = self.get_header_padding_length(attribute_length);
        let padding: String = " ".repeat(padding_length);
        let padded_header: String = padding.to_string() + &self.header + &padding;
        padded_header
    }

    fn get_header_ansi(&self) -> CombinedANSI {
        let ansi: CombinedANSI = YellowBackgroundANSI.combine(&BlackANSI);
        ansi
    }

    fn get_delimiter(&self) -> String {
        let delimiter: String = "|".to_string() + &" ".repeat(self.delimiter_spacing);
        delimiter
    }

    fn pad_right_to_length(&self, string: &str, length: usize) -> String {
        format!("{: <width$}", string, width = length)
    }
}
