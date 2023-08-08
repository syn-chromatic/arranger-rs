use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::fs::{DirEntry, ReadDir};
use std::io;
use std::path::{Path, PathBuf};

use crate::general::terminal::Terminal;
use crate::general::terminal::{ANSICode, WhiteANSI, YellowANSI};

pub struct SearchProgress {
    terminal: Terminal,
    search_counter: usize,
    match_counter: usize,
    previous_length: usize,
}

impl SearchProgress {
    pub fn new() -> Self {
        let terminal: Terminal = Terminal::new();
        let search_counter: usize = 0;
        let match_counter: usize = 0;
        let previous_length: usize = 0;

        SearchProgress {
            terminal,
            search_counter,
            match_counter,
            previous_length,
        }
    }

    pub fn increment_search(&mut self) {
        self.search_counter += 1;
    }

    pub fn increment_match(&mut self) {
        self.match_counter += 1;
    }

    pub fn print_progress(&mut self) {
        let match_string: String = self.match_counter.to_string();
        let search_string: String = self.search_counter.to_string();
        let parts: [&str; 5] = [
            "\rMatch: ",
            &match_string,
            " | ",
            "Searched: ",
            &search_string,
        ];
        let colors: [Box<dyn ANSICode>; 5] = [
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

    pub fn print_finalize(&mut self) {
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
    fn get_fill(&self, string: &str) -> usize {
        let previous_length: isize = self.previous_length as isize;
        let current_length: isize = string.len() as isize;
        let fill: isize = previous_length - current_length;
        if fill >= 0 {
            return fill as usize;
        }
        0
    }
}

pub struct FileSearch {
    root: Option<PathBuf>,
    exclusive_filenames: HashSet<String>,
    exclusive_file_stems: HashSet<String>,
    exclusive_exts: HashSet<String>,
    exclude_dirs: HashSet<PathBuf>,
    quit_directory_on_match: bool,
}

impl FileSearch {
    pub fn new() -> Self {
        let root: Option<PathBuf> = None;
        let exclusive_filenames: HashSet<String> = HashSet::new();
        let exclusive_file_stems: HashSet<String> = HashSet::new();
        let exclusive_exts: HashSet<String> = HashSet::new();
        let exclude_dirs: HashSet<PathBuf> = HashSet::new();
        let quit_directory_on_match: bool = false;

        FileSearch {
            root,
            exclusive_filenames,
            exclusive_file_stems,
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

    pub fn set_exclusive_file_stems(&mut self, file_stems: Vec<&str>) {
        let mut exclusive_file_stems: HashSet<String> = HashSet::new();
        for file_stem in file_stems {
            exclusive_file_stems.insert(file_stem.to_string());
        }
        self.exclusive_file_stems = exclusive_file_stems;
    }

    pub fn set_exclusive_extensions(&mut self, exts: Vec<&str>) {
        let mut exclusive_exts: HashSet<String> = HashSet::new();
        for ext in exts {
            let ext: String = self.format_extension(ext);
            exclusive_exts.insert(ext.to_string());
        }
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
        let mut roots: HashSet<PathBuf> = HashSet::new();
        let mut files: HashSet<PathBuf> = HashSet::new();
        let root: PathBuf = self.get_root_path();
        self.print_search_initialize(&root);
        let mut search_progress: SearchProgress = SearchProgress::new();
        self.search(&root, &mut roots, &mut files, &mut search_progress);
        search_progress.print_finalize();
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
        let is_exclusive_extension: bool = self.is_exclusive_extension(path);
        let filter_validation: bool =
            is_exclusive_filename && is_exclusive_file_stem && is_exclusive_extension;
        filter_validation
    }

    fn get_entry_path(&self, entry: &Result<DirEntry, io::Error>) -> Option<PathBuf> {
        if entry.is_ok() {
            let path_buf: PathBuf = entry.as_ref().unwrap().path();
            let path_canonical: Option<PathBuf> = self.get_canonical_path(&path_buf);
            return path_canonical;
        }
        None
    }

    fn get_canonical_path(&self, path: &PathBuf) -> Option<PathBuf> {
        let path_canonical: Result<PathBuf, io::Error> = path.canonicalize();
        if path_canonical.is_ok() {
            return Some(path_canonical.unwrap());
        }

        println!("\nPath Inaccessible: {:?}\n", path);
        None
    }

    fn get_directory_entries(&self, root: &PathBuf) -> Option<ReadDir> {
        let entries: Result<ReadDir, io::Error> = root.read_dir();
        if entries.is_ok() {
            return Some(entries.unwrap());
        }
        println!("\nPath Inaccessible: {:?}\n", root);
        None
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
        path: &PathBuf,
        files: &mut HashSet<PathBuf>,
        search_progress: &mut SearchProgress,
    ) -> bool {
        let filter_validation: bool = self.get_filter_validation(&path);

        search_progress.increment_search();
        if !files.contains(path) && filter_validation {
            files.insert(path.clone());
            search_progress.increment_match();
            return true;
        }
        false
    }

    fn recurse_additional_directories(
        &self,
        additional_directories: HashSet<PathBuf>,
        roots: &mut HashSet<PathBuf>,
        files: &mut HashSet<PathBuf>,
        search_progress: &mut SearchProgress,
    ) {
        for path in additional_directories {
            roots.insert(path.clone());
            self.search(&path, roots, files, search_progress);
        }
    }

    fn walker(
        &self,
        entries: ReadDir,
        roots: &mut HashSet<PathBuf>,
        files: &mut HashSet<PathBuf>,
        search_progress: &mut SearchProgress,
    ) {
        let mut additional_directories: HashSet<PathBuf> = HashSet::new();

        for entry in entries {
            let entry_path: Option<PathBuf> = self.get_entry_path(&entry);

            if let Some(path) = entry_path {
                search_progress.print_progress();

                if path.is_file() {
                    let is_match: bool = self.handle_file(&path, files, search_progress);
                    if is_match && self.quit_directory_on_match {
                        return;
                    }
                } else if path.is_dir() {
                    additional_directories.insert(path);
                }
            }
        }

        self.recurse_additional_directories(additional_directories, roots, files, search_progress);
    }

    fn search(
        &self,
        root: &PathBuf,
        roots: &mut HashSet<PathBuf>,
        files: &mut HashSet<PathBuf>,
        search_progress: &mut SearchProgress,
    ) {
        let root_op: Option<PathBuf> = self.get_canonical_path(root);
        if let Some(root) = root_op {
            if self.is_excluded_directory(&root) {
                return;
            }

            let entries: Option<ReadDir> = self.get_directory_entries(&root);
            if let Some(entries) = entries {
                self.walker(entries, roots, files, search_progress);
            }
        }
    }
}
