use std::fs::Metadata;
use std::path::PathBuf;
use std::time::Instant;

use crate::search::formatters::format_size;
use crate::search::formatters::format_time;

use crate::general::table_display::DynamicTable;
use crate::general::writer::ConsoleWriter;

pub struct SearchProgress {
    table: DynamicTable,
    writer: ConsoleWriter,
    search_counter: usize,
    match_counter: usize,
    search_bytes: usize,
    time: Instant,
    elapsed_time_ns: u128,
}

impl SearchProgress {
    pub fn new() -> Self {
        let table: DynamicTable = DynamicTable::new(0.8, 1);
        let writer: ConsoleWriter = ConsoleWriter::new();
        let search_counter: usize = 0;
        let match_counter: usize = 0;
        let search_bytes: usize = 0;
        let time: Instant = Instant::now();
        let elapsed_time_ns: u128 = 0;

        SearchProgress {
            table,
            writer,
            search_counter,
            match_counter,
            search_bytes,
            time,
            elapsed_time_ns,
        }
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

    pub fn display_progress(&mut self) {
        if self.search_counter % 500 == 0 {
            self.table.update_terminal_width();
            self.write_progress();
        }
    }

    pub fn display_progress_finalize(&mut self) {
        self.write_progress();
        println!();
    }

    pub fn set_search_path(&mut self, path: &PathBuf) {
        let path_string = self.get_path_string(&path);
        self.table.add_parameter_string("Path", &path_string);
    }

    pub fn get_elapsed_time_ns(&self) -> u128 {
        self.elapsed_time_ns
    }
}

impl SearchProgress {
    fn update_elapsed_time_ns(&mut self) {
        let elapsed_time_ns: u128 = self.time.elapsed().as_nanos();
        self.elapsed_time_ns = elapsed_time_ns;
    }

    fn write_progress(&mut self) {
        let size_string: String = format_size(self.search_bytes);

        self.update_elapsed_time_ns();
        let time_string: String = format_time(self.get_elapsed_time_ns());

        self.table.add_parameter("Match", self.match_counter);
        self.table.add_parameter("Search", self.search_counter);
        self.table.add_parameter_string("Size", &size_string);
        self.table.add_parameter_string("Time", &time_string);

        let table_string: String = self.table.get_table_string();
        self.writer.write(&table_string);
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
