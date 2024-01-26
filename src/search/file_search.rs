use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::fs::{Metadata, ReadDir};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use regex::Regex;

use thread_manager::ThreadLooper;
use thread_manager::ThreadManager;

use crate::search::file_info::FileInfo;
use crate::search::metrics::{ProgressMetrics, SearchMetrics};

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

    fn get_root_path(&self) -> Result<PathBuf, io::Error> {
        let root: PathBuf = if let Some(root) = &self.root {
            root.to_path_buf()
        } else {
            env::current_dir().unwrap()
        };

        let root: Result<PathBuf, io::Error> = root.canonicalize();
        root
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

        if let Some(regex) = self.exclusive_filename_regex.as_ref() {
            let is_match: bool = regex.is_match(&filename);
            return is_match;
        }
        false
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
        search_metrics: Arc<SearchMetrics>,
    ) -> bool {
        let entry_criteria: bool = self.evaluate_entry_criteria(&file);

        let progress_metrics: Arc<ProgressMetrics> = search_metrics.get_metrics();
        progress_metrics.increment_search_count();
        progress_metrics.add_search_bytes(&metadata);

        if !files.contains(&file) && entry_criteria {
            let file_info: FileInfo = FileInfo::new(file, metadata);
            files.insert(file_info);
            progress_metrics.increment_match_count();
            return true;
        }
        false
    }

    fn handle_entry(
        &self,
        entry: &DirEntry,
        files: &mut HashSet<FileInfo>,
        queue: &mut LinkedList<PathBuf>,
        search_metrics: &Arc<SearchMetrics>,
    ) -> bool {
        if let Ok(metadata) = entry.metadata() {
            let path: PathBuf = entry.path();

            if metadata.is_dir() {
                queue.push_back(path);
            } else if metadata.is_file() {
                let is_match: bool =
                    self.handle_file(metadata, path, files, search_metrics.clone());
                return is_match;
            }
        }
        false
    }

    fn walker(
        self: &Arc<Self>,
        root: &PathBuf,
        files: &mut HashSet<FileInfo>,
        search_metrics: &Arc<SearchMetrics>,
    ) -> LinkedList<PathBuf> {
        let mut queue: LinkedList<PathBuf> = LinkedList::new();

        let entries: ReadDir = match root.read_dir() {
            Ok(entries) => entries,
            Err(_) => return queue,
        };

        for entry in entries {
            if let Ok(entry) = entry.as_ref() {
                let is_match = self.handle_entry(entry, files, &mut queue, search_metrics);
                if is_match && self.quit_directory_on_match {
                    return queue;
                }
            }
        }
        queue
    }

    fn batch_walker(
        self: &Arc<Self>,
        batch: &Vec<PathBuf>,
        search_metrics: &Arc<SearchMetrics>,
    ) -> (HashSet<FileInfo>, LinkedList<PathBuf>) {
        let mut files_batch: HashSet<FileInfo> = HashSet::new();
        let mut queue_batch: LinkedList<PathBuf> = LinkedList::new();

        for root in batch.iter() {
            if !self.is_excluded_directory(&root) {
                let queue: LinkedList<PathBuf> =
                    self.walker(&root, &mut files_batch, search_metrics);
                queue_batch.extend(queue);
            }
        }

        (files_batch, queue_batch)
    }
}

pub struct SearchThreadScheduler {
    batch_size: usize,
    file_search: Arc<FileSearch>,
    metrics_display: ThreadLooper,
    thread_manager: ThreadManager<(HashSet<FileInfo>, LinkedList<PathBuf>)>,
}

impl SearchThreadScheduler {
    pub fn new(threads: usize, batch_size: usize, file_search: FileSearch) -> Self {
        let file_search: Arc<FileSearch> = Arc::new(file_search);
        let metrics_display: ThreadLooper = ThreadLooper::new();
        let thread_manager: ThreadManager<(HashSet<FileInfo>, LinkedList<PathBuf>)> =
            ThreadManager::new(threads);

        Self {
            batch_size,
            file_search,
            thread_manager,
            metrics_display,
        }
    }

    pub fn search_files(&self, update_rate: Duration) -> HashSet<FileInfo> {
        let root: Result<PathBuf, io::Error> = self.file_search.get_root_path();
        let mut files: HashSet<FileInfo> = HashSet::new();
        let mut queue: LinkedList<PathBuf> = LinkedList::new();

        if let Ok(root) = root {
            let search_metrics: Arc<SearchMetrics> = Arc::new(SearchMetrics::new(update_rate));
            queue.push_back(root);

            self.metrics_display_thread(&search_metrics);
            self.spawn_walkers(&mut files, &mut queue, &search_metrics);
        }

        files
    }

    pub fn search_files_benchmark(&self, update_rate: Duration) -> Arc<SearchMetrics> {
        let root: Result<PathBuf, io::Error> = self.file_search.get_root_path();
        let search_metrics: Arc<SearchMetrics> = Arc::new(SearchMetrics::new(update_rate));

        let mut files: HashSet<FileInfo> = HashSet::new();
        let mut queue: LinkedList<PathBuf> = LinkedList::new();

        if let Ok(root) = root {
            queue.push_back(root);

            self.metrics_display_thread(&search_metrics);
            self.spawn_walkers(&mut files, &mut queue, &search_metrics);
        }

        search_metrics
    }
}

impl SearchThreadScheduler {
    fn spawn_walkers(
        &self,
        files: &mut HashSet<FileInfo>,
        queue: &mut LinkedList<PathBuf>,
        search_metrics: &Arc<SearchMetrics>,
    ) {
        let progress_metrics: Arc<ProgressMetrics> = search_metrics.get_metrics();
        self.add_batched_threads(queue, search_metrics);

        for (r_files, r_queue) in self.thread_manager.yield_results() {
            files.extend(r_files);
            queue.extend(r_queue);

            self.add_batched_threads(queue, search_metrics);
            progress_metrics.set_busy_threads(self.thread_manager.busy_threads());
        }

        self.finalize(search_metrics, &progress_metrics);
    }

    fn finalize(
        &self,
        search_metrics: &Arc<SearchMetrics>,
        progress_metrics: &Arc<ProgressMetrics>,
    ) {
        self.thread_manager.join();
        search_metrics.terminate();
        self.metrics_display.terminate();

        progress_metrics.set_busy_threads(self.thread_manager.busy_threads());
        search_metrics.finalize();
    }

    fn metrics_display_thread(&self, search_metrics: &Arc<SearchMetrics>) {
        let search_metrics: Arc<SearchMetrics> = search_metrics.clone();
        self.metrics_display.start(move || {
            search_metrics.blocking_display_progress();
        });
    }

    fn get_queue_batch(&self, queue: &mut LinkedList<PathBuf>) -> Option<Vec<PathBuf>> {
        if queue.is_empty() {
            return None;
        }

        let batch_size: usize = usize::min(self.batch_size, queue.len());
        let mut batch: Vec<PathBuf> = Vec::with_capacity(batch_size);

        for _ in 0..batch_size {
            if let Some(path) = queue.pop_front() {
                batch.push(path);
            }
        }

        Some(batch)
    }

    fn add_batched_threads(
        &self,
        queue: &mut LinkedList<PathBuf>,
        search_metrics: &Arc<SearchMetrics>,
    ) {
        while let Some(batch) = self.get_queue_batch(queue) {
            let search_clone: Arc<FileSearch> = self.file_search.clone();
            let search_metrics: Arc<SearchMetrics> = search_metrics.clone();

            self.thread_manager
                .execute(move || search_clone.batch_walker(&batch, &search_metrics));
        }
    }
}
