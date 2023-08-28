use std::fs::Metadata;
use std::path::PathBuf;
use std::time::Instant;

use crate::general::terminal::Terminal;
use crate::general::terminal::{WhiteANSI, YellowANSI};

use crate::search::formatters::format_size;
use crate::search::formatters::format_time;

pub struct SearchProgress {
    terminal: Terminal,
    search_counter: usize,
    match_counter: usize,
    search_bytes: usize,
    previous_length: usize,
    time: Instant,
    elapsed_time_ns: u128,
}

impl SearchProgress {
    pub fn new() -> Self {
        let terminal: Terminal = Terminal::new();
        let search_counter: usize = 0;
        let match_counter: usize = 0;
        let search_bytes: usize = 0;
        let previous_length: usize = 0;
        let time: Instant = Instant::now();
        let elapsed_time_ns: u128 = 0;

        SearchProgress {
            terminal,
            search_counter,
            match_counter,
            search_bytes,
            previous_length,
            time,
            elapsed_time_ns,
        }
    }

    pub fn get_elapsed_time_ns(&self) -> u128 {
        self.elapsed_time_ns
    }

    pub fn increment_search_count(&mut self) {
        self.search_counter += 1;
    }

    pub fn increment_match_count(&mut self) {
        self.match_counter += 1;
    }

    pub fn add_search_bytes(&mut self, metadata: &Metadata) {
        let bytes: usize = metadata.len() as usize;
        self.search_bytes += bytes;
    }

    pub fn display_search_path(&self, root: &PathBuf) {
        let path_string: String = self.get_path_string(root);
        let formatted_path: String = format!("[{}]", path_string);
        let parts: [&str; 2] = ["Searching In: ", &formatted_path];
        self.terminal.writeln_parameter(&parts, &YellowANSI);
    }

    pub fn display_progress(&mut self) {
        if self.search_counter % 500 == 0 {
            self.write_progress();
        }
    }

    pub fn display_progress_finalize(&mut self) {
        self.write_progress();
        println!();
    }
}

impl SearchProgress {
    fn update_elapsed_time_ns(&mut self) {
        let elapsed_time_ns: u128 = self.time.elapsed().as_nanos();
        self.elapsed_time_ns = elapsed_time_ns;
    }

    fn write_progress(&mut self) {
        let match_string: String = self.match_counter.to_string();
        let search_string: String = self.search_counter.to_string();
        let size_string: String = format_size(self.search_bytes);

        self.update_elapsed_time_ns();
        let time_string: String = format_time(self.get_elapsed_time_ns());

        let parts: [&str; 8] = [
            "\rMatches: ",
            &match_string,
            "Searches: ",
            &search_string,
            "Size: ",
            &size_string,
            "Time: ",
            &time_string,
        ];

        let color: &YellowANSI = &YellowANSI;
        let sep: &str = " | ";
        let length: usize = self.terminal.write_separated_parameters(&parts, color, sep);
        self.write_space_fill(length);
        self.previous_length = length;
    }

    fn write_space_fill(&self, length: usize) {
        let fill_length: usize = self.get_fill_length(length);
        let fill_string: String = " ".repeat(fill_length);
        self.terminal.write_ansi(&fill_string, &WhiteANSI);
    }

    fn get_fill_length(&self, length: usize) -> usize {
        let length: isize = length as isize;
        let previous_length: isize = self.previous_length as isize;
        let fill: isize = previous_length - length;
        if fill >= 0 {
            return fill as usize;
        }
        0
    }

    fn get_path_string(&self, path: &PathBuf) -> String {
        let mut path_string: String = path.to_string_lossy().to_string();

        let stripped_path: Option<&str> = path_string.strip_prefix(r"\\?\");
        if let Some(stripped_path) = stripped_path {
            path_string = stripped_path.to_string();
        }
        path_string
    }
}
