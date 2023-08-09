use std::io;
use std::str::Lines;

use crate::general::terminal::Terminal;
use crate::general::terminal::{CyanANSI, GreenANSI, RedANSI};
use crate::general::terminal::{WhiteANSI, YellowANSI};

pub fn confirm_and_continue() -> bool {
    let terminal: Terminal = Terminal::new();
    let mut input: String = String::new();

    let string: &str = "\nDo you want to continue? [y/N]: ";
    terminal.write_color(string, CyanANSI);

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim() == "y" || input.trim() == "Y" {
                let string: &str = "Continuing...\n";
                terminal.writeln_color(string, GreenANSI);
                return true;
            } else {
                let string: &str = "Not continuing...\n";
                terminal.writeln_color(string, RedANSI);
                return false;
            }
        }
        Err(e) => {
            let string: String = format!("Failed to read line: {}\n", e);
            terminal.writeln_color(&string, RedANSI);
            return false;
        }
    }
}

pub fn split_once_retain_delim_left<'a>(
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

pub fn print_options(opt_string: &str) {
    let terminal: Terminal = Terminal::new();
    let lines: Lines = opt_string.lines();

    for (idx, line) in lines.enumerate() {
        if !line.is_empty() {
            let first_char: Option<char> = line.chars().next();
            if first_char.map_or(false, |c| c.is_alphabetic()) {
                let split: Option<(&str, &str)> = split_once_retain_delim_left(&line, ":");
                if let Some(split) = split {
                    let parts: [&str; 2] = [split.0, split.1];
                    if split.1.is_empty() {
                        terminal.writeln_2p_primary(&parts, YellowANSI);
                    } else {
                        terminal.writeln_2p_primary(&parts, RedANSI);
                    }
                    continue;
                }
                if idx == 0 {
                    terminal.writeln_color(&line, GreenANSI);
                    continue;
                }
            }
        }
        terminal.writeln_color(&line, WhiteANSI);
    }
}
