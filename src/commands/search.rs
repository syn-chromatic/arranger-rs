use std::collections::HashSet;
use std::env;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use regex::Regex;

use crate::general::terminal::Terminal;
use crate::general::terminal::{CyanANSI, GreenANSI, RedANSI};

use crate::commands::configuration::SearchSort;
use crate::general::grid_printer::DynamicTable;
use crate::general::grid_printer::FileInfoPrinter;
use crate::general::path::WPath;
use crate::search::file::FileSearch;
use crate::search::info::FileInfo;
use crate::SearchOption;

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

            let mut files_hashset: HashSet<FileInfo> = file_search.search_files();
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
    ) -> Result<(), io::Error> {
        let exclusive_exts: &Vec<String> = &self.option.extensions;
        let excluded_dirs: &Vec<String> = &self.option.excluded_dirs;

        file_search.set_root(root);
        file_search.set_exclusive_extensions(exclusive_exts);
        let exclusion_result: Result<(), io::Error> =
            file_search.set_exclude_directories(excluded_dirs);

        if let Err(exclusion_error) = exclusion_result {
            let parts: [&str; 2] = ["Directory Exclusion Error: ", &exclusion_error.to_string()];
            self.terminal.writeln_parameter(&parts, &RedANSI);
            return Err(exclusion_error);
        }

        self.set_file_search_file_stem(file_search);
        Ok(())
    }

    fn set_file_search_file_stem(&self, file_search: &mut FileSearch) {
        let filename: &Option<String> = &self.option.filename;

        if let Some(filename) = filename {
            if self.option.regex {
                let regex: Result<Regex, regex::Error> = Regex::new(&filename);
                if let Ok(regex) = regex {
                    file_search.set_exclusive_file_stem_regex(&regex);
                } else {
                    let error: String = regex.unwrap_err().to_string();
                    self.terminal.writeln_ansi(&error, &RedANSI);
                    return;
                }
            } else {
                let exclusive_file_stems: Vec<&String> = vec![filename];
                file_search.set_exclusive_file_stems(exclusive_file_stems);
            }
        }
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
                SearchSort::SizeAsc => self.sort_ascending_by_size(files),
                SearchSort::SizeDesc => self.sort_descending_by_size(files),
                SearchSort::CreatedAsc => self.sort_ascending_by_created(files),
                SearchSort::CreatedDesc => self.sort_descending_by_created(files),
                SearchSort::ModifiedAsc => self.sort_ascending_by_modified(files),
                SearchSort::ModifiedDesc => self.sort_descending_by_modified(files),
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
            let file_info_printer: FileInfoPrinter = FileInfoPrinter::new(2, 0.9);
            println!();
            file_info_printer.print_header("FILES");
            // self.terminal.writeln_ansi("\nFiles:", &GreenANSI);
            for file_info in files_iterator {
                file_info_printer.print(file_info);
                // self.print_file_info_path(&file_info);
                // self.print_file_info_metadata(&file_info);
                println!();
            }
        } else {
            self.terminal
                .writeln_ansi("\nNo files were found.", &RedANSI);
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

    fn print_file_info_path(&self, file_info: &FileInfo) {
        let file: WPath = file_info.get_path().into();
        let path_str: String = format!("[{:?}]", file);
        let parts: [&str; 2] = ["Path: ", &path_str];
        self.terminal.writeln_parameter(&parts, &CyanANSI);
    }

    fn print_file_info_metadata(&self, file_info: &FileInfo) {
        let size: String = file_info.get_formatted_size();
        let size_str: String = format!("[{}]", size);
        let creation: String = file_info.get_formatted_created_time();
        let creation_str: String = format!("[{}]", creation);
        let modified: String = file_info.get_formatted_modified_time();
        let modified_str: String = format!("[{}]", modified);

        let parts: [&str; 6] = [
            "Size: ",
            &size_str,
            "Created: ",
            &creation_str,
            "Modified: ",
            &modified_str,
        ];

        let color: &CyanANSI = &CyanANSI;
        let sep: &str = " | ";

        self.terminal
            .writeln_separated_parameters(&parts, color, sep);
    }
}
