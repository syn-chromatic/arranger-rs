use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;
use std::time::{Duration, SystemTime};

use crate::terminal::RedANSI;
use crate::terminal::Terminal;

use crate::commands::configuration::SearchOption;
use crate::commands::configuration::SearchSort;
use crate::general::table_display::DynamicTable;
use crate::general::table_display::FileInfoTable;

use crate::search::file_search::{FileSearch, SearchThreadScheduler};
use crate::search::info::FileInfo;

pub struct SearchCommand {
    option: SearchOption,
    terminal: Terminal,
}

impl SearchCommand {
    pub fn new(option: SearchOption) -> Self {
        let terminal: Terminal = Terminal::new();
        SearchCommand { option, terminal }
    }

    pub fn execute_command(&self) {
        let mut file_search: FileSearch = FileSearch::new();

        self.print_search_parameters();
        let current_dir: Result<PathBuf, io::Error> = env::current_dir();

        if let Ok(root) = current_dir {
            match self.set_file_search_parameters(&root, &mut file_search) {
                Ok(_) => {}
                Err(_) => return,
            };

            let threads: usize = self.option.threads;
            let batch_size: usize = 100;
            let search_scheduler: SearchThreadScheduler =
                SearchThreadScheduler::new(threads, batch_size, file_search);

            let update_rate: Duration = Duration::from_millis(50);
            let mut files_hashset: HashSet<FileInfo> = search_scheduler.search_files(update_rate);
            let mut files: Vec<FileInfo> = files_hashset.drain().collect();
            self.sort_files(&mut files);
            self.print_files(&files);
        }
    }
}

impl SearchCommand {
    fn set_file_search_parameters(
        &self,
        root: &PathBuf,
        file_search: &mut FileSearch,
    ) -> Result<(), Box<dyn Error>> {
        let exclusive_exts: &Vec<String> = &self.option.extensions;
        let excluded_dirs: &Vec<String> = &self.option.excluded_dirs;

        file_search.set_root(root);
        file_search.set_exclusive_extensions(exclusive_exts);
        let exclusion_result: Result<(), io::Error> =
            file_search.set_exclude_directories(excluded_dirs);

        if let Err(error) = exclusion_result {
            let parts: [&str; 2] = ["Directory Exclusion Error: ", &error.to_string()];
            self.terminal.writeln_parameter(&parts, &RedANSI);
            return Err(Box::new(error));
        }

        self.set_file_search_filename(file_search)?;
        Ok(())
    }

    fn set_file_search_filename(&self, file_search: &mut FileSearch) -> Result<(), Box<dyn Error>> {
        let filename: &Option<String> = &self.option.filename;

        if let Some(filename) = filename {
            if self.option.regex {
                let result: Result<(), regex::Error> =
                    file_search.set_exclusive_filename_regex(filename);
                if let Err(error) = result {
                    self.terminal.writeln_ansi(&error.to_string(), &RedANSI);
                    return Err(Box::new(error));
                }
            } else {
                file_search.set_exclusive_filename(filename);
            }
        }
        Ok(())
    }

    fn get_filename_or_default(&self) -> &str {
        if let Some(filename) = &self.option.filename {
            return filename;
        }
        ""
    }

    fn sort_files(&self, files: &mut Vec<FileInfo>) {
        if let Some(sort) = &self.option.sort {
            match sort {
                SearchSort::SizeAscending => self.sort_ascending_by_size(files),
                SearchSort::SizeDescending => self.sort_descending_by_size(files),
                SearchSort::CreatedAscending => self.sort_ascending_by_created(files),
                SearchSort::CreatedDescending => self.sort_descending_by_created(files),
                SearchSort::ModifiedAscending => self.sort_ascending_by_modified(files),
                SearchSort::ModifiedDescending => self.sort_descending_by_modified(files),
            }
        }
    }

    fn sort_ascending_by_size(&self, files: &mut Vec<FileInfo>) {
        files.sort_by(|a, b| a.get_size().cmp(&b.get_size()));
    }

    fn sort_descending_by_size(&self, files: &mut Vec<FileInfo>) {
        files.sort_by(|a, b| b.get_size().cmp(&a.get_size()));
    }

    fn sort_ascending_by_created(&self, files: &mut Vec<FileInfo>) {
        files.sort_by(|a, b| {
            let time_a: SystemTime = a.get_created_time().unwrap_or(UNIX_EPOCH);
            let time_b: SystemTime = b.get_created_time().unwrap_or(UNIX_EPOCH);
            time_a.cmp(&time_b)
        });
    }

    fn sort_descending_by_created(&self, files: &mut Vec<FileInfo>) {
        files.sort_by(|a, b| {
            let time_a: SystemTime = a.get_created_time().unwrap_or(UNIX_EPOCH);
            let time_b: SystemTime = b.get_created_time().unwrap_or(UNIX_EPOCH);
            time_b.cmp(&time_a)
        });
    }

    fn sort_ascending_by_modified(&self, files: &mut Vec<FileInfo>) {
        files.sort_by(|a, b| {
            let time_a: SystemTime = a.get_created_time().unwrap_or(UNIX_EPOCH);
            let time_b: SystemTime = b.get_created_time().unwrap_or(UNIX_EPOCH);
            time_a.cmp(&time_b)
        });
    }

    fn sort_descending_by_modified(&self, files: &mut Vec<FileInfo>) {
        files.sort_by(|a, b| {
            let time_a: SystemTime = a.get_modified_time().unwrap_or(UNIX_EPOCH);
            let time_b: SystemTime = b.get_modified_time().unwrap_or(UNIX_EPOCH);
            time_b.cmp(&time_a)
        });
    }

    fn get_files_iterator<'a>(
        &self,
        files: &'a Vec<FileInfo>,
    ) -> Box<dyn Iterator<Item = &'a FileInfo> + 'a> {
        if let Some(limit) = self.option.limit {
            return Box::new(files.iter().take(limit));
        } else {
            return Box::new(files.iter());
        }
    }

    fn print_files(&self, files: &Vec<FileInfo>) {
        if !files.is_empty() {
            let files_iterator: Box<dyn Iterator<Item = &FileInfo>> =
                self.get_files_iterator(files);
            let file_info_printer: FileInfoTable = FileInfoTable::new(2, 0.9);

            file_info_printer.print_header("FILES");
            for file_info in files_iterator {
                file_info_printer.print(file_info);
                println!();
            }
        } else {
            self.terminal.writeln_ansi("No files were found.", &RedANSI);
        }
    }

    fn print_search_parameters(&self) {
        let filename: &str = self.get_filename_or_default();
        let extensions: &Vec<String> = &self.option.extensions;
        let excluded_dirs: &Vec<String> = &self.option.excluded_dirs;
        let sort: &Option<SearchSort> = &self.option.sort;
        let limit: &Option<usize> = &self.option.limit;
        let regex: bool = self.option.regex;

        let mut table: DynamicTable = DynamicTable::new(0.6, 1);
        table.set_header("Search Parameters");
        table.add_parameter("Filename", filename);
        table.add_parameter("Extensions", extensions);
        table.add_parameter("Excluded Dirs", excluded_dirs);

        if let Some(sort) = sort {
            table.add_parameter("Sorting", sort);
        }

        if let Some(limit) = limit {
            table.add_parameter("Limit", limit);
        }

        table.add_parameter("Regex", regex);
        table.print();
        println!();
    }
}
