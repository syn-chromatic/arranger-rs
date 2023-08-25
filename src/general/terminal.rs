use std::io;
use std::io::Write;

pub trait ANSICode: Send + Sync {
    fn value(&self) -> String;
    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI;
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

#[derive(Clone)]
pub struct CombinedANSI(String);

impl ANSICode for CombinedANSI {
    fn value(&self) -> String {
        self.0.clone()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct ResetANSI;
impl ANSICode for ResetANSI {
    fn value(&self) -> String {
        "\x1B[0m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BlackANSI;
impl ANSICode for BlackANSI {
    fn value(&self) -> String {
        "\x1B[30m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct RedANSI;
impl ANSICode for RedANSI {
    fn value(&self) -> String {
        "\x1B[31m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct GreenANSI;
impl ANSICode for GreenANSI {
    fn value(&self) -> String {
        "\x1B[32m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct YellowANSI;
impl ANSICode for YellowANSI {
    fn value(&self) -> String {
        "\x1B[33m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BlueANSI;
impl ANSICode for BlueANSI {
    fn value(&self) -> String {
        "\x1B[34m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct MagentaANSI;
impl ANSICode for MagentaANSI {
    fn value(&self) -> String {
        "\x1B[35m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct CyanANSI;
impl ANSICode for CyanANSI {
    fn value(&self) -> String {
        "\x1B[36m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct WhiteANSI;
impl ANSICode for WhiteANSI {
    fn value(&self) -> String {
        "\x1B[37m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BlackBackgroundANSI;
impl ANSICode for BlackBackgroundANSI {
    fn value(&self) -> String {
        "\x1B[40m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct RedBackgroundANSI;
impl ANSICode for RedBackgroundANSI {
    fn value(&self) -> String {
        "\x1B[41m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct GreenBackgroundANSI;
impl ANSICode for GreenBackgroundANSI {
    fn value(&self) -> String {
        "\x1B[42m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct YellowBackgroundANSI;
impl ANSICode for YellowBackgroundANSI {
    fn value(&self) -> String {
        "\x1B[43m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BlueBackgroundANSI;
impl ANSICode for BlueBackgroundANSI {
    fn value(&self) -> String {
        "\x1B[44m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct MagentaBackgroundANSI;
impl ANSICode for MagentaBackgroundANSI {
    fn value(&self) -> String {
        "\x1B[45m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct CyanBackgroundANSI;
impl ANSICode for CyanBackgroundANSI {
    fn value(&self) -> String {
        "\x1B[46m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct WhiteBackgroundANSI;
impl ANSICode for WhiteBackgroundANSI {
    fn value(&self) -> String {
        "\x1B[47m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct UnderlineANSI;
impl ANSICode for UnderlineANSI {
    fn value(&self) -> String {
        "\x1B[4m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct ItalicANSI;
impl ANSICode for ItalicANSI {
    fn value(&self) -> String {
        "\x1B[3m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BoldANSI;
impl ANSICode for BoldANSI {
    fn value(&self) -> String {
        "\x1B[1m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BlinkANSI;
impl ANSICode for BlinkANSI {
    fn value(&self) -> String {
        "\x1B[5m".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

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
