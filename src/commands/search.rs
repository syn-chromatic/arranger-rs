use std::collections::HashSet;
use std::env;
use std::io;
use std::path::PathBuf;

use regex::Regex;

use crate::general::terminal::Terminal;
use crate::general::terminal::{CyanANSI, GreenANSI, RedANSI, YellowANSI};

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
            let exclusive_exts: &Vec<String> = &self.option.extensions;

            file_search.set_root(root);
            file_search.set_exclusive_extensions(exclusive_exts);

            self.set_file_search_file_stem(&mut file_search);
            let files: HashSet<FileInfo> = file_search.search_files();
            if !files.is_empty() {
                self.terminal.writeln_color("\nFiles:", &GreenANSI);

                for file_info in files {
                    self.print_file_info_path(&file_info);
                    self.print_file_info_metadata(&file_info);
                    println!();
                }
            } else {
                self.terminal
                    .writeln_color("\nNo files were found.", &RedANSI);
            }
        }
    }
}

impl SearchCommand {
    fn set_file_search_file_stem(&self, file_search: &mut FileSearch) {
        let filename: &Option<String> = &self.option.filename;

        if let Some(filename) = filename {
            if self.option.regex {
                let regex: Result<Regex, regex::Error> = Regex::new(&filename);
                if let Ok(regex) = regex {
                    file_search.set_exclusive_file_stem_regex(&regex);
                } else {
                    let error: String = regex.unwrap_err().to_string();
                    self.terminal.writeln_color(&error, &RedANSI);
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

    fn print_search_parameters(&self) {
        let filename: &str = self.get_filename_or_default();
        let extensions: &Vec<String> = &self.option.extensions;
        let regex: bool = self.option.regex;

        let parameters: String = format!(
            "Filename: [{}] | Extensions: {:?} | Regex: [{}]\n",
            filename, extensions, regex
        );

        self.terminal
            .write_color("Search Parameters: ", &YellowANSI);
        self.terminal.write(&parameters);
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
        let creation: String = file_info.get_formatted_creation_time();
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
