use std::collections::HashSet;
use std::env;

use std::io;
use std::path::PathBuf;

use regex::Regex;

use crate::general::terminal::Terminal;
use crate::general::terminal::{ANSICode, CyanANSI, WhiteANSI};
use crate::general::terminal::{GreenANSI, RedANSI, YellowANSI};

use crate::general::path::WPath;
use crate::search::file::FileSearch;
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
            let exclude_dirs: Vec<&str> = vec![];
            let quit_directory_on_match: bool = false;

            file_search.set_root(root);
            file_search.set_exclusive_extensions(exclusive_exts);
            file_search.set_exclude_directories(exclude_dirs);
            file_search.set_quit_directory_on_match(quit_directory_on_match);

            self.set_file_search_file_stem(&mut file_search);

            let files: HashSet<PathBuf> = file_search.search_files();
            if !files.is_empty() {
                self.terminal.writeln_color("\nFiles:", GreenANSI);

                for file in files {
                    let file: WPath = file.into();
                    let path_str: String = format!("[{:?}]", file);
                    let parts: [&str; 2] = ["Path: ", &path_str];
                    let colors: [Box<dyn ANSICode>; 2] = [CyanANSI.boxed(), WhiteANSI.boxed()];
                    self.terminal.writeln_color_p(&parts, &colors);
                }
            } else {
                self.terminal
                    .writeln_color("\nNo files were found.", RedANSI);
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
                    self.terminal.writeln_color(&error, RedANSI);
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
        self.terminal.write_color("Search Parameters: ", YellowANSI);
        self.terminal.write(&parameters);
        println!();
    }
}
