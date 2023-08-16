use std::fs::Metadata;

use crate::general::terminal::Terminal;
use crate::general::terminal::{WhiteANSI, YellowANSI};

pub struct SearchProgress {
    terminal: Terminal,
    search_counter: usize,
    match_counter: usize,
    search_bytes: u64,
    previous_length: usize,
}

impl SearchProgress {
    pub fn new() -> Self {
        let terminal: Terminal = Terminal::new();
        let search_counter: usize = 0;
        let match_counter: usize = 0;
        let search_bytes: u64 = 0;
        let previous_length: usize = 0;

        SearchProgress {
            terminal,
            search_counter,
            match_counter,
            search_bytes,
            previous_length,
        }
    }

    pub fn increment_search(&mut self) {
        self.search_counter += 1;
    }

    pub fn add_search_bytes(&mut self, metadata: &Metadata) {
        let bytes: u64 = metadata.len();
        self.search_bytes += bytes;
    }

    pub fn increment_match(&mut self) {
        self.match_counter += 1;
    }

    pub fn show_progress(&mut self) {
        if self.search_counter % 500 == 0 {
            self.write_progress();
        }
    }

    pub fn finalize(&mut self) {
        self.write_progress();
        println!();
    }

    pub fn reset(&mut self) {
        self.search_counter = 0;
        self.match_counter = 0;
        self.previous_length = 0;
    }
}

impl SearchProgress {
    fn write_progress(&mut self) {
        let match_string: String = self.match_counter.to_string();
        let search_string: String = self.search_counter.to_string();
        let size_string: String = self.get_formatted_size();
        let parts: [&str; 6] = [
            "\rMatch: ",
            &match_string,
            "Searched Items: ",
            &search_string,
            "Searched Size: ",
            &size_string,
        ];

        let color: &YellowANSI = &YellowANSI;
        let sep: &str = " | ";
        let length: usize = self.terminal.write_separated_parameters(&parts, color, sep);
        self.write_fill_string(length);
        self.previous_length = length;
    }

    fn write_fill_string(&self, length: usize) {
        let fill: usize = self.get_fill(length);
        let fill_string: String = " ".repeat(fill);
        self.terminal.write_color(&fill_string, &WhiteANSI);
    }

    fn get_fill(&self, length: usize) -> usize {
        let length: isize = length as isize;
        let previous_length: isize = self.previous_length as isize;
        let fill: isize = previous_length - length;
        if fill >= 0 {
            return fill as usize;
        }
        0
    }

    fn get_formatted_size(&self) -> String {
        const KB: f64 = (1u64 << 10) as f64;
        const MB: f64 = (1u64 << 20) as f64;
        const GB: f64 = (1u64 << 30) as f64;
        const TB: f64 = (1u64 << 40) as f64;

        let search_bytes: f64 = self.search_bytes as f64;
        match search_bytes {
            _ if search_bytes <= KB => format!("{:.2} B", search_bytes),
            _ if search_bytes < MB => format!("{:.2} KB", search_bytes / KB),
            _ if search_bytes < GB => format!("{:.2} MB", search_bytes / MB),
            _ if search_bytes < TB => format!("{:.2} GB", search_bytes / GB),
            _ => format!("{:.2} TB", search_bytes / TB),
        }
    }
}
