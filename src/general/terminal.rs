use std::io;
use std::io::Write;

pub trait ANSICode: Send + Sync {
    fn value(&self) -> &'static str;
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

#[derive(Copy, Clone)]
pub struct ResetANSI;
impl ANSICode for ResetANSI {
    fn value(&self) -> &'static str {
        "\x1B[0m"
    }
}
#[derive(Copy, Clone)]
pub struct RedANSI;
impl ANSICode for RedANSI {
    fn value(&self) -> &'static str {
        "\x1B[31m"
    }
}
#[derive(Copy, Clone)]
pub struct GreenANSI;
impl ANSICode for GreenANSI {
    fn value(&self) -> &'static str {
        "\x1B[32m"
    }
}
#[derive(Copy, Clone)]
pub struct YellowANSI;
impl ANSICode for YellowANSI {
    fn value(&self) -> &'static str {
        "\x1B[33m"
    }
}
#[derive(Copy, Clone)]
pub struct BlueANSI;
impl ANSICode for BlueANSI {
    fn value(&self) -> &'static str {
        "\x1B[34m"
    }
}
#[derive(Copy, Clone)]
pub struct MagentaANSI;
impl ANSICode for MagentaANSI {
    fn value(&self) -> &'static str {
        "\x1B[35m"
    }
}
#[derive(Copy, Clone)]
pub struct CyanANSI;
impl ANSICode for CyanANSI {
    fn value(&self) -> &'static str {
        "\x1B[36m"
    }
}
#[derive(Copy, Clone)]
pub struct WhiteANSI;
impl ANSICode for WhiteANSI {
    fn value(&self) -> &'static str {
        "\x1B[37m"
    }
}

pub struct Terminal {
    ansi_color: Box<dyn ANSICode>,
    ansi_reset: ResetANSI,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            ansi_color: Box::new(WhiteANSI),
            ansi_reset: ResetANSI,
        }
    }

    pub fn write(&self, text: &str) {
        let ansi_color_v: &str = self.ansi_color.value();
        let ansi_reset_v: &str = self.ansi_reset.value();
        let string: String = format!("{}{}{}", ansi_color_v, text, ansi_reset_v);
        print!("{}", string);
        io::stdout().flush().unwrap();
    }

    pub fn writeln(&self, text: &str) {
        let ansi_color_v: &str = self.ansi_color.value();
        let ansi_reset_v: &str = self.ansi_reset.value();
        let string: String = format!("{}{}{}", ansi_color_v, text, ansi_reset_v);
        println!("{}", string);
    }

    pub fn write_color<T: ANSICode + 'static>(&self, text: &str, ansi_color: &T) {
        let ansi_color_v: &str = ansi_color.value();
        let ansi_reset_v: &str = self.ansi_reset.value();
        let string: String = format!("{}{}{}", ansi_color_v, text, ansi_reset_v);
        print!("{}", string);
        io::stdout().flush().unwrap();
    }

    pub fn writeln_color<T: ANSICode + 'static>(&self, text: &str, ansi_color: &T) {
        let ansi_color_v: &str = ansi_color.value();
        let ansi_reset_v: &str = self.ansi_reset.value();
        let string: String = format!("{}{}{}", ansi_color_v, text, ansi_reset_v);
        println!("{}", string);
    }

    pub fn write_parameter<T: ANSICode + 'static + std::marker::Copy>(
        &self,
        parts: &[&str; 2],
        ansi_color: &T,
    ) {
        for (idx, part) in parts.iter().enumerate() {
            if idx % 2 == 0 {
                self.write_color(part, ansi_color);
            } else {
                self.write_color(part, &WhiteANSI);
            }
        }
    }

    pub fn writeln_parameter<T: ANSICode + 'static + std::marker::Copy>(
        &self,
        parts: &[&str; 2],
        ansi_color: &T,
    ) {
        self.write_parameter(parts, ansi_color);
        println!();
    }

    pub fn write_separated_parameters<T: ANSICode + 'static>(
        &self,
        parts: &[&str],
        ansi_color: &T,
        separator: &str,
    ) -> usize {
        let mut length: usize = 0;
        for (idx, part) in parts.iter().enumerate() {
            if idx % 2 == 0 {
                self.write_color(part, ansi_color);
                length += part.len();
            } else {
                self.write_color(part, &WhiteANSI);
                length += part.len();
                if idx != (parts.len() - 1) {
                    self.write_color(separator, &WhiteANSI);
                    length += separator.len();
                }
            }
        }
        length
    }

    pub fn writeln_separated_parameters<T: ANSICode + 'static>(
        &self,
        parts: &[&str],
        ansi_color: &T,
        separator: &str,
    ) -> usize {
        let length: usize = self.write_separated_parameters(parts, ansi_color, separator);
        println!();
        length
    }

    pub fn set_ansi_color<T: ANSICode + 'static>(&mut self, ansi_color: T) {
        self.ansi_color = Box::new(ansi_color);
    }

    pub fn write_reset(&self) {
        let ansi_reset_v: &str = self.ansi_reset.value();
        print!("{}", ansi_reset_v);
        io::stdout().flush().unwrap();
    }
}
