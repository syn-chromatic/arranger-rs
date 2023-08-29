use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::fs::{Metadata, ReadDir};
use std::io;
use std::path::{Path, PathBuf};

use regex::Regex;

use crate::search::info::FileInfo;
use crate::search::progress::SearchProgress;

pub struct FileSearch {
    root: Option<PathBuf>,
    exclusive_filename: String,
    exclusive_filename_regex: Option<Regex>,
    exclusive_exts: HashSet<String>,
    exclude_dirs: HashSet<PathBuf>,
    quit_directory_on_match: bool,
}

impl FileSearch {
    pub fn new() -> Self {
        let root: Option<PathBuf> = None;
        let exclusive_filename: String = String::new();
        let exclusive_filename_regex: Option<Regex> = None;
        let exclusive_exts: HashSet<String> = HashSet::new();
        let exclude_dirs: HashSet<PathBuf> = HashSet::new();
        let quit_directory_on_match: bool = false;

        FileSearch {
            root,
            exclusive_filename,
            exclusive_filename_regex,
            exclusive_exts,
            exclude_dirs,
            quit_directory_on_match,
        }
    }

    pub fn set_root<T: AsRef<Path>>(&mut self, root: T) {
        self.root = Some(PathBuf::from(root.as_ref()));
    }

    pub fn set_exclusive_filename(&mut self, filename: &str) {
        self.exclusive_filename = filename.to_string();
    }

    pub fn set_exclusive_filename_regex(&mut self, filename: &str) -> Result<(), regex::Error> {
        let regex: Regex = Regex::new(&filename)?;
        self.exclusive_filename_regex = Some(regex);
        return Ok(());
    }

    pub fn set_exclusive_extensions<I, S>(&mut self, exts: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut exclusive_exts: HashSet<String> = HashSet::new();

        for ext in exts {
            let ext: String = ext.as_ref().trim().to_lowercase();
            exclusive_exts.insert(ext);
        }

        self.exclusive_exts = exclusive_exts;
    }

    pub fn set_exclude_directories<I, S>(&mut self, dirs: I) -> Result<(), io::Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        let mut exclude_dirs: HashSet<PathBuf> = HashSet::new();

        for dir in dirs {
            let directory: PathBuf = PathBuf::from(dir.as_ref());
            let canonical_directory: PathBuf = self.canonicalize_directory(&directory)?;
            exclude_dirs.insert(canonical_directory);
        }

        self.exclude_dirs = exclude_dirs;
        Ok(())
    }

    pub fn set_quit_directory_on_match(&mut self, state: bool) {
        self.quit_directory_on_match = state;
    }

    pub fn clear_root(&mut self) {
        self.root = None;
    }

    pub fn clear_exclusive_filename(&mut self) {
        self.exclusive_filename = String::new();
    }

    pub fn clear_exclusive_filename_regex(&mut self) {
        self.exclusive_filename_regex = None;
    }

    pub fn clear_exclusive_extensions(&mut self) {
        self.exclusive_exts = HashSet::new();
    }

    pub fn clear_exclude_directories(&mut self) {
        self.exclude_dirs = HashSet::new();
    }

    pub fn search_files(&self) -> HashSet<FileInfo> {
        let mut files: HashSet<FileInfo> = HashSet::new();
        let mut queue: LinkedList<PathBuf> = LinkedList::new();
        let mut search_progress: SearchProgress = SearchProgress::new();

        let root: Result<PathBuf, io::Error> = self.get_root_path();
        if let Ok(root) = root {
            search_progress.set_search_path(&root);
            queue.push_back(root);

            while let Some(current_dir) = queue.pop_front() {
                self.walker(&current_dir, &mut files, &mut queue, &mut search_progress);
            }
            search_progress.display_progress_finalize();
        }

        files
    }

    pub fn search_files_benchmark(&self) -> SearchProgress {
        let mut files: HashSet<FileInfo> = HashSet::new();
        let mut queue: LinkedList<PathBuf> = LinkedList::new();
        let mut search_progress: SearchProgress = SearchProgress::new();

        let root: Result<PathBuf, io::Error> = self.get_root_path();
        if let Ok(root) = root {
            search_progress.set_search_path(&root);
            queue.push_back(root);

            while let Some(current_dir) = queue.pop_front() {
                self.walker(&current_dir, &mut files, &mut queue, &mut search_progress);
            }

            search_progress.display_progress_finalize();
        }
        search_progress
    }
}

impl FileSearch {
    fn canonicalize_directory(&self, directory: &PathBuf) -> Result<PathBuf, io::Error> {
        let canonical_directory: Result<PathBuf, io::Error> = directory.canonicalize();
        if let Ok(canonical_directory) = canonical_directory {
            if canonical_directory.is_file() {
                let error: io::Error = self.get_path_is_file_error();
                return Err(error);
            }
            return Ok(canonical_directory);
        }

        let error: io::Error = self.get_invalid_directory_error(directory);
        return Err(error);
    }

    fn get_path_is_file_error(&self) -> io::Error {
        let path_is_file: String = format!("Path provided is a file, not a directory.");
        let error: io::Error = io::Error::new(io::ErrorKind::Other, path_is_file);
        error
    }

    fn get_invalid_directory_error(&self, directory: &PathBuf) -> io::Error {
        let unavailable_dir: String = format!(
            "Path provided [{}] cannot be found.",
            directory.to_string_lossy()
        );
        let error: io::Error = io::Error::new(io::ErrorKind::Other, unavailable_dir);
        error
    }

    fn evaluate_entry_criteria(&self, path: &PathBuf) -> bool {
        let filename_regex: bool = self.exclusive_filename_regex.is_some();
        let is_exclusive_filename: bool = if filename_regex {
            self.is_exclusive_filename_regex(path)
        } else {
            self.is_exclusive_filename(path)
        };

        let is_exclusive_extension: bool = self.is_exclusive_extension(path);
        let entry_criteria: bool = is_exclusive_filename && is_exclusive_extension;

        entry_criteria
    }

    fn get_root_path(&self) -> Result<PathBuf, io::Error> {
        let root: PathBuf = if let Some(root) = &self.root {
            root.to_path_buf()
        } else {
            env::current_dir().unwrap()
        };

        let root: Result<PathBuf, io::Error> = root.canonicalize();
        root
    }

    fn is_same_directory(&self, path: &PathBuf, dir: &PathBuf) -> bool {
        if path.is_file() {
            let path_parent: Option<&Path> = path.parent();
            if let Some(path_parent) = path_parent {
                return dir == path_parent;
            }
        }
        dir == path
    }

    fn is_exclusive_filename(&self, path: &PathBuf) -> bool {
        if self.exclusive_filename.is_empty() {
            return false;
        }

        let filename: &OsStr = path.file_name().unwrap_or_default();
        let filename: String = filename.to_string_lossy().to_lowercase();
        let filename_exists: bool = filename.starts_with(&self.exclusive_filename);
        filename_exists
    }

    fn is_exclusive_filename_regex(&self, path: &PathBuf) -> bool {
        if self.exclusive_filename_regex.is_none() {
            return false;
        }

        let filename: &OsStr = path.file_name().unwrap_or_default();
        let filename: String = filename.to_string_lossy().to_lowercase();

        let regex: &Regex = self.exclusive_filename_regex.as_ref().unwrap();
        let is_match: bool = regex.is_match(&filename);
        is_match
    }

    fn is_exclusive_extension(&self, path: &PathBuf) -> bool {
        if self.exclusive_exts.is_empty() {
            return true;
        }

        let file_ext: &OsStr = path.extension().unwrap_or_default();
        let file_ext: String = file_ext.to_string_lossy().to_lowercase();

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
            if self.is_same_directory(path, dir) {
                return true;
            }
        }
        false
    }

    fn handle_file(
        &self,
        metadata: Metadata,
        file: PathBuf,
        files: &mut HashSet<FileInfo>,
        search_progress: &mut SearchProgress,
    ) -> bool {
        let entry_criteria: bool = self.evaluate_entry_criteria(&file);

        search_progress.increment_search_count();
        search_progress.add_search_bytes(&metadata);

        if !files.contains(&file) && entry_criteria {
            let file_info: FileInfo = FileInfo::new(file, metadata);
            files.insert(file_info);
            search_progress.increment_match_count();
            return true;
        }
        false
    }

    fn handle_entry(
        &self,
        entry: &DirEntry,
        files: &mut HashSet<FileInfo>,
        sub_directories: &mut LinkedList<PathBuf>,
        search_progress: &mut SearchProgress,
    ) {
        if let Ok(metadata) = entry.metadata() {
            search_progress.display_progress();
            let path: PathBuf = entry.path();

            if metadata.is_file() {
                let is_match: bool = self.handle_file(metadata, path, files, search_progress);
                if is_match && self.quit_directory_on_match {
                    return;
                }
            } else if metadata.is_dir() {
                if !metadata.is_symlink() {
                    sub_directories.push_back(path);
                }
            }
        }
    }

    fn walker(
        &self,
        root: &PathBuf,
        files: &mut HashSet<FileInfo>,
        queue: &mut LinkedList<PathBuf>,
        search_progress: &mut SearchProgress,
    ) {
        if self.is_excluded_directory(&root) {
            return;
        }

        let entries: ReadDir = match root.read_dir() {
            Ok(entries) => entries,
            Err(_) => return,
        };

        let mut sub_directories: LinkedList<PathBuf> = LinkedList::new();

        for entry in entries {
            if let Ok(entry) = entry.as_ref() {
                self.handle_entry(entry, files, &mut sub_directories, search_progress);
            }
        }

        queue.append(&mut sub_directories);
    }
}
