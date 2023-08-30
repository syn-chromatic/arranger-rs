use std::io;
use std::io::Write;

use crate::terminal::ANSICode;
use crate::terminal::ResetANSI;
use crate::terminal::WhiteANSI;

pub struct Terminal {
    ansi_code: Box<dyn ANSICode>,
    ansi_reset: ResetANSI,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            ansi_code: Box::new(WhiteANSI),
            ansi_reset: ResetANSI,
        }
    }

    #[allow(dead_code)]
    pub fn write(&self, text: &str) {
        let ansi_code_v: String = self.ansi_code.value();
        let ansi_reset_v: String = self.ansi_reset.value();
        let string: String = format!("{}{}{}", ansi_code_v, text, ansi_reset_v);
        print!("{}", string);
        io::stdout().flush().unwrap();
    }

    #[allow(dead_code)]
    pub fn writeln(&self, text: &str) {
        let ansi_code_v: String = self.ansi_code.value();
        let ansi_reset_v: String = self.ansi_reset.value();
        let string: String = format!("{}{}{}", ansi_code_v, text, ansi_reset_v);
        println!("{}", string);
    }

    pub fn write_ansi<T: ANSICode + 'static>(&self, text: &str, ansi_code: &T) {
        let ansi_code_v: String = ansi_code.value();
        let ansi_reset_v: String = self.ansi_reset.value();
        let string: String = format!("{}{}{}", ansi_code_v, text, ansi_reset_v);
        print!("{}", string);
        io::stdout().flush().unwrap();
    }

    pub fn writeln_ansi<T: ANSICode + 'static>(&self, text: &str, ansi_code: &T) {
        let ansi_code_v: String = ansi_code.value();
        let ansi_reset_v: String = self.ansi_reset.value();
        let string: String = format!("{}{}{}", ansi_code_v, text, ansi_reset_v);
        println!("{}", string);
    }

    pub fn write_parameter<T: ANSICode + 'static + std::marker::Copy>(
        &self,
        parts: &[&str; 2],
        ansi_code: &T,
    ) {
        for (idx, part) in parts.iter().enumerate() {
            if idx % 2 == 0 {
                self.write_ansi(part, ansi_code);
            } else {
                self.write_ansi(part, &WhiteANSI);
            }
        }
    }

    pub fn writeln_parameter<T: ANSICode + 'static + std::marker::Copy>(
        &self,
        parts: &[&str; 2],
        ansi_code: &T,
    ) {
        self.write_parameter(parts, ansi_code);
        println!();
    }

    pub fn write_separated_parameters<T: ANSICode + 'static>(
        &self,
        parts: &[&str],
        ansi_code: &T,
        separator: &str,
    ) -> usize {
        let mut length: usize = 0;
        for (idx, part) in parts.iter().enumerate() {
            if idx % 2 == 0 {
                self.write_ansi(part, ansi_code);
                length += part.len();
            } else {
                self.write_ansi(part, &WhiteANSI);
                length += part.len();
                if idx != (parts.len() - 1) {
                    self.write_ansi(separator, &WhiteANSI);
                    length += separator.len();
                }
            }
        }
        length
    }

    #[allow(dead_code)]
    pub fn writeln_separated_parameters<T: ANSICode + 'static>(
        &self,
        parts: &[&str],
        ansi_code: &T,
        separator: &str,
    ) -> usize {
        let length: usize = self.write_separated_parameters(parts, ansi_code, separator);
        println!();
        length
    }

    #[allow(dead_code)]
    pub fn set_ansi_code<T: ANSICode + 'static>(&mut self, ansi_code: T) {
        self.ansi_code = Box::new(ansi_code);
    }

    #[allow(dead_code)]
    pub fn write_reset(&self) {
        let ansi_reset_v: String = self.ansi_reset.value();
        print!("{}", ansi_reset_v);
        io::stdout().flush().unwrap();
    }
}
