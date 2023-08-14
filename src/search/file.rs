use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::ffi::OsStr;
use std::fs::{Metadata, ReadDir};
use std::io;
use std::path::{Path, PathBuf};

use regex::Regex;

use crate::general::terminal::Terminal;
use crate::general::terminal::{ANSICode, WhiteANSI, YellowANSI};

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
            self.print_progress();
        }
    }

    pub fn finalize(&mut self) {
        self.print_progress();
        println!();
    }

    pub fn reset(&mut self) {
        self.search_counter = 0;
        self.match_counter = 0;
        self.previous_length = 0;
    }
}

impl SearchProgress {
    fn print_progress(&mut self) {
        let match_string: String = self.match_counter.to_string();
        let search_string: String = self.search_counter.to_string();
        let size_string: String = self.get_formatted_size();
        let parts: [&str; 8] = [
            "\rMatch: ",
            &match_string,
            " | ",
            "Searched Items: ",
            &search_string,
            " | ",
            "Searched Size: ",
            &size_string,
        ];

        let colors: [Box<dyn ANSICode>; 8] = [
            YellowANSI.boxed(),
            WhiteANSI.boxed(),
            WhiteANSI.boxed(),
            YellowANSI.boxed(),
            WhiteANSI.boxed(),
            WhiteANSI.boxed(),
            YellowANSI.boxed(),
            WhiteANSI.boxed(),
        ];

        let string: String = parts.join("");
        let fill: usize = self.get_fill(&string);
        let fill_string: String = " ".repeat(fill);
        self.previous_length = string.len();

        self.terminal.write_color_p(&parts, &colors);
        self.terminal.write_color(&fill_string, WhiteANSI);
    }

    fn get_fill(&self, string: &str) -> usize {
        let previous_length: isize = self.previous_length as isize;
        let current_length: isize = string.len() as isize;
        let fill: isize = previous_length - current_length;
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

pub struct FileSearch {
    root: Option<PathBuf>,
    exclusive_filenames: HashSet<String>,
    exclusive_file_stems: HashSet<String>,
    exclusive_file_stem_regex: Option<Regex>,
    exclusive_exts: HashSet<String>,
    exclude_dirs: HashSet<PathBuf>,
    quit_directory_on_match: bool,
}

impl FileSearch {
    pub fn new() -> Self {
        let root: Option<PathBuf> = None;
        let exclusive_filenames: HashSet<String> = HashSet::new();
        let exclusive_file_stems: HashSet<String> = HashSet::new();
        let exclusive_file_stem_regex: Option<Regex> = None;
        let exclusive_exts: HashSet<String> = HashSet::new();
        let exclude_dirs: HashSet<PathBuf> = HashSet::new();
        let quit_directory_on_match: bool = false;

        FileSearch {
            root,
            exclusive_filenames,
            exclusive_file_stems,
            exclusive_file_stem_regex,
            exclusive_exts,
            exclude_dirs,
            quit_directory_on_match,
        }
    }

    pub fn set_root<T: AsRef<Path>>(&mut self, root: T) {
        self.root = Some(PathBuf::from(root.as_ref()));
    }

    pub fn set_exclusive_filenames(&mut self, filenames: Vec<&str>) {
        let mut exclusive_filenames: HashSet<String> = HashSet::new();
        for filename in filenames {
            exclusive_filenames.insert(filename.to_string());
        }
        self.exclusive_filenames = exclusive_filenames;
    }

    pub fn set_exclusive_file_stems<I, S>(&mut self, file_stems: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let exclusive_file_stems: HashSet<String> = file_stems
            .into_iter()
            .map(|file_stem| file_stem.as_ref().to_string())
            .collect();

        self.exclusive_file_stems = exclusive_file_stems;
    }

    pub fn set_exclusive_file_stem_regex(&mut self, file_stem_regex: &Regex) {
        self.exclusive_file_stem_regex = Some(file_stem_regex.clone());
    }

    pub fn set_exclusive_extensions<I, S>(&mut self, exts: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let exclusive_exts: HashSet<String> = exts
            .into_iter()
            .map(|ext| self.format_extension(ext.as_ref()))
            .collect();

        self.exclusive_exts = exclusive_exts;
    }

    pub fn set_exclude_directories(&mut self, dirs: Vec<&str>) {
        let mut exclude_dirs: HashSet<PathBuf> = HashSet::new();
        for dir in dirs {
            exclude_dirs.insert(PathBuf::from(dir));
        }
        self.exclude_dirs = exclude_dirs;
    }

    pub fn set_quit_directory_on_match(&mut self, state: bool) {
        self.quit_directory_on_match = state;
    }

    pub fn search_files(&self) -> HashSet<PathBuf> {
        let mut files: HashSet<PathBuf> = HashSet::new();
        let mut queue: LinkedList<PathBuf> = LinkedList::new();
        let mut search_progress: SearchProgress = SearchProgress::new();

        let root: PathBuf = self.get_root_path();
        self.print_search_initialize(&root);
        queue.push_back(root);

        while let Some(current_dir) = queue.pop_front() {
            self.search(&current_dir, &mut files, &mut queue, &mut search_progress);
        }

        search_progress.finalize();
        files
    }
}

impl FileSearch {
    fn print_search_initialize(&self, root: &PathBuf) {
        let mut string: String = root.to_string_lossy().to_string();

        let stripped_string: Option<&str> = string.strip_prefix(r"\\?\");
        if let Some(stripped_string) = stripped_string {
            string = stripped_string.to_string();
        }

        let terminal: Terminal = Terminal::new();
        let path_string: String = format!("[{}]", string);
        let parts: [&str; 2] = ["Searching In: ", &path_string];
        let colors: [Box<dyn ANSICode>; 2] = [YellowANSI.boxed(), WhiteANSI.boxed()];
        terminal.writeln_color_p(&parts, &colors);
    }

    fn format_extension(&self, ext: &str) -> String {
        let mut ext: String = ext.trim().to_lowercase();
        if !ext.is_empty() && !ext.starts_with('.') {
            ext.insert(0, '.');
        }
        ext
    }

    fn get_filter_validation(&self, path: &PathBuf) -> bool {
        let is_exclusive_filename: bool = self.is_exclusive_filename(path);
        let is_exclusive_file_stem: bool = self.is_exclusive_file_stem(path);
        let is_exclusive_file_stem_regex: bool = self.is_exclusive_file_stem_regex(path);
        let is_exclusive_extension: bool = self.is_exclusive_extension(path);
        let filter_validation: bool = is_exclusive_filename
            && is_exclusive_file_stem
            && is_exclusive_file_stem_regex
            && is_exclusive_extension;

        filter_validation
    }

    fn get_abs_path(&self) -> PathBuf {
        env::current_dir().unwrap()
    }

    fn get_root_path(&self) -> PathBuf {
        if let Some(root) = &self.root {
            return root.clone();
        }
        self.get_abs_path()
    }

    fn is_same_directory(&self, file: &PathBuf, dir: &PathBuf) -> bool {
        if dir.exists() {
            for ancestor in file.ancestors() {
                if ancestor == dir {
                    return true;
                }
            }
        }
        false
    }

    fn is_exclusive_filename(&self, path: &PathBuf) -> bool {
        if self.exclusive_filenames.is_empty() {
            return true;
        }

        let p_filename: &OsStr = path.file_name().unwrap_or_default();
        let p_filename: String = p_filename.to_string_lossy().to_lowercase();
        if self.exclusive_filenames.contains(&p_filename) {
            return true;
        }
        false
    }

    fn is_exclusive_file_stem(&self, path: &PathBuf) -> bool {
        if self.exclusive_file_stems.is_empty() {
            return true;
        }

        let file_stem: &OsStr = path.file_stem().unwrap_or_default();
        let file_stem: String = file_stem.to_string_lossy().to_lowercase();
        if self.exclusive_file_stems.contains(&file_stem) {
            return true;
        }
        false
    }

    fn is_exclusive_file_stem_regex(&self, path: &PathBuf) -> bool {
        if self.exclusive_file_stem_regex.is_none() {
            return true;
        }

        let file_stem: &OsStr = path.file_stem().unwrap_or_default();
        let file_stem: String = file_stem.to_string_lossy().to_lowercase();

        let regex: &Regex = self.exclusive_file_stem_regex.as_ref().unwrap();
        let is_match: bool = regex.is_match(&file_stem);
        is_match
    }

    fn is_exclusive_extension(&self, path: &PathBuf) -> bool {
        if self.exclusive_exts.is_empty() {
            return true;
        }

        let file_ext: &OsStr = path.extension().unwrap_or_default();
        let file_ext: String = file_ext.to_string_lossy().to_lowercase();
        let file_ext: String = self.format_extension(&file_ext);

        if self.exclusive_exts.contains(&file_ext) {
            return true;
        }

        false
    }

    fn is_excluded_directory(&self, path: &PathBuf) -> bool {
        if self.exclude_dirs.is_empty() {
            return false;
        }

        for dir in &self.exclude_dirs {
            let same_directory: bool = self.is_same_directory(path, dir);
            if same_directory {
                return true;
            }
        }
        false
    }

    fn handle_file(
        &self,
        metadata: &Metadata,
        file: &PathBuf,
        files: &mut HashSet<PathBuf>,
        search_progress: &mut SearchProgress,
    ) -> bool {
        let filter_validation: bool = self.get_filter_validation(file);

        search_progress.increment_search();
        search_progress.add_search_bytes(metadata);

        if !files.contains(file) && filter_validation {
            files.insert(file.clone());
            search_progress.increment_match();
            return true;
        }
        false
    }

    fn walker(
        &self,
        entries: ReadDir,
        files: &mut HashSet<PathBuf>,
        queue: &mut LinkedList<PathBuf>,
        search_progress: &mut SearchProgress,
    ) {
        let mut additional_directories: LinkedList<PathBuf> = LinkedList::new();

        for entry in entries {
            if let Ok(entry) = entry.as_ref() {
                if let Ok(metadata) = entry.metadata() {
                    search_progress.show_progress();
                    let path: PathBuf = entry.path();

                    if metadata.is_file() {
                        let is_match: bool =
                            self.handle_file(&metadata, &path, files, search_progress);
                        if is_match && self.quit_directory_on_match {
                            return;
                        }
                    } else if metadata.is_dir() {
                        if !metadata.is_symlink() {
                            additional_directories.push_back(path);
                        }
                    }
                }
            }
        }

        queue.append(&mut additional_directories);
    }

    fn search(
        &self,
        root: &PathBuf,
        files: &mut HashSet<PathBuf>,
        queue: &mut LinkedList<PathBuf>,
        search_progress: &mut SearchProgress,
    ) {
        if let Ok(root) = root.canonicalize() {
            if self.is_excluded_directory(&root) {
                return;
            }

            let entries: Result<ReadDir, io::Error> = root.read_dir();
            if let Ok(entries) = entries {
                self.walker(entries, files, queue, search_progress);
            }
        }
    }
}
