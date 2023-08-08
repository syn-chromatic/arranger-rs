use std::io;
use std::io::Write;

pub trait ANSICode {
    fn value(&self) -> &'static str;
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub struct ResetANSI;
impl ANSICode for ResetANSI {
    fn value(&self) -> &'static str {
        "\x1B[0m"
    }
}

pub struct RedANSI;
impl ANSICode for RedANSI {
    fn value(&self) -> &'static str {
        "\x1B[31m"
    }
}

pub struct GreenANSI;
impl ANSICode for GreenANSI {
    fn value(&self) -> &'static str {
        "\x1B[32m"
    }
}

pub struct YellowANSI;
impl ANSICode for YellowANSI {
    fn value(&self) -> &'static str {
        "\x1B[33m"
    }
}

pub struct BlueANSI;
impl ANSICode for BlueANSI {
    fn value(&self) -> &'static str {
        "\x1B[34m"
    }
}

pub struct MagentaANSI;
impl ANSICode for MagentaANSI {
    fn value(&self) -> &'static str {
        "\x1B[35m"
    }
}

pub struct CyanANSI;
impl ANSICode for CyanANSI {
    fn value(&self) -> &'static str {
        "\x1B[36m"
    }
}

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

    pub fn write_color<T: ANSICode + 'static>(&self, text: &str, ansi_color: T) {
        let ansi_color_v: &str = ansi_color.value();
        let ansi_reset_v: &str = self.ansi_reset.value();
        let string: String = format!("{}{}{}", ansi_color_v, text, ansi_reset_v);
        print!("{}", string);
        io::stdout().flush().unwrap();
    }

    pub fn writeln_color<T: ANSICode + 'static>(&self, text: &str, ansi_color: T) {
        let ansi_color_v: &str = ansi_color.value();
        let ansi_reset_v: &str = self.ansi_reset.value();
        let string: String = format!("{}{}{}", ansi_color_v, text, ansi_reset_v);
        println!("{}", string);
    }

    pub fn write_color_p(&self, parts: &[&str], ansi_colors: &[Box<dyn ANSICode>]) {
        for (text, ansi_color) in parts.iter().zip(ansi_colors.iter()) {
            let ansi_color_v: &str = ansi_color.value();
            let ansi_reset_v: &str = self.ansi_reset.value();
            let string: String = format!("{}{}{}", ansi_color_v, text, ansi_reset_v);
            print!("{}", string);
            io::stdout().flush().unwrap();
        }
    }

    pub fn writeln_color_p(&self, parts: &[&str], ansi_colors: &[Box<dyn ANSICode>]) {
        self.write_color_p(parts, ansi_colors);
        println!();
    }

    pub fn write_2p_primary<T: ANSICode + 'static>(&self, parts: &[&str; 2], ansi_color: T) {
        let colors: [Box<dyn ANSICode>; 2] = [ansi_color.boxed(), WhiteANSI.boxed()];
        self.write_color_p(parts, &colors);
    }

    pub fn writeln_2p_primary<T: ANSICode + 'static>(&self, parts: &[&str; 2], ansi_color: T) {
        let colors: [Box<dyn ANSICode>; 2] = [ansi_color.boxed(), WhiteANSI.boxed()];
        self.writeln_color_p(parts, &colors);
    }

    pub fn set_ansi_color<T: ANSICode + 'static>(&mut self, ansi_color: T) {
        self.ansi_color = Box::new(ansi_color);
    }
}
