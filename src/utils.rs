use std::io;

use crate::general::terminal::Terminal;
use crate::general::terminal::{CyanANSI, GreenANSI, RedANSI};

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
