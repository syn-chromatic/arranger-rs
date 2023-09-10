use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::fs::{Metadata, ReadDir};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use regex::Regex;

use crate::search::info::FileInfo;
use crate::search::progress::{ProgressMetrics, SearchMetrics};
use crate::threading::{AtomicChannel, ThreadLoop, ThreadManager};

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
        search_metrics: &Arc<SearchMetrics>,
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
                let is_match: bool = self.handle_file(metadata, path, files, search_metrics);
                return is_match;
            }
        }
        false
    }

    fn send_to_queue_channel(
        &self,
        queue: LinkedList<PathBuf>,
        queue_channel: &Arc<AtomicChannel<LinkedList<PathBuf>>>,
    ) {
        if !queue.is_empty() {
            let timeout: Duration = Duration::from_millis(1);
            let _ = queue_channel.send_timeout(queue, timeout);
        }
    }

    fn send_to_files_channel(
        &self,
        files: HashSet<FileInfo>,
        files_channel: &Arc<AtomicChannel<HashSet<FileInfo>>>,
    ) {
        if !files.is_empty() {
            let timeout: Duration = Duration::from_millis(1);
            let _ = files_channel.send_timeout(files, timeout);
        }
    }

    fn walker(
        self: &Arc<Self>,
        root: &PathBuf,
        files: &mut HashSet<FileInfo>,
        queue_channel: &Arc<AtomicChannel<LinkedList<PathBuf>>>,
        search_metrics: &Arc<SearchMetrics>,
    ) {
        let entries: ReadDir = match root.read_dir() {
            Ok(entries) => entries,
            Err(_) => return,
        };
        let mut queue: LinkedList<PathBuf> = LinkedList::new();

        for entry in entries {
            if let Ok(entry) = entry.as_ref() {
                let is_match = self.handle_entry(entry, files, &mut queue, search_metrics);
                if is_match && self.quit_directory_on_match {
                    return;
                }
            }
        }

        self.send_to_queue_channel(queue, queue_channel);
    }

    fn batch_walker(
        self: &Arc<Self>,
        batch: Vec<PathBuf>,
        files_channel: &Arc<AtomicChannel<HashSet<FileInfo>>>,
        queue_channel: &Arc<AtomicChannel<LinkedList<PathBuf>>>,
        search_metrics: &Arc<SearchMetrics>,
    ) {
        let mut files_batch: HashSet<FileInfo> = HashSet::new();

        for root in batch.iter() {
            if !self.is_excluded_directory(&root) {
                self.walker(&root, &mut files_batch, queue_channel, search_metrics);
            }
        }
        self.send_to_files_channel(files_batch, files_channel);
    }
}

struct SearchActivity {
    busy_threads: usize,
    job_queue: usize,
    queue_buffer: usize,
    queue: usize,
}

impl SearchActivity {
    fn new(busy_threads: usize, job_queue: usize, queue_buffer: usize, queue: usize) -> Self {
        SearchActivity {
            busy_threads,
            job_queue,
            queue_buffer,
            queue,
        }
    }

    fn all_empty(&self) -> bool {
        if self.busy_threads == 0
            && self.job_queue == 0
            && self.queue_buffer == 0
            && self.queue == 0
        {
            return true;
        }
        false
    }
}

pub struct SearchThreadScheduler {
    batch_size: usize,
    threads: usize,
    file_search: Arc<FileSearch>,
    files_channel: Arc<AtomicChannel<HashSet<FileInfo>>>,
    queue_channel: Arc<AtomicChannel<LinkedList<PathBuf>>>,
    thread_manager: ThreadManager,
    metrics_thread_worker: ThreadLoop,
}

impl SearchThreadScheduler {
    pub fn new(threads: usize, batch_size: usize, file_search: FileSearch) -> Self {
        let file_search: Arc<FileSearch> = Arc::new(file_search);
        let files_channel: Arc<AtomicChannel<HashSet<FileInfo>>> = Arc::new(AtomicChannel::new());
        let queue_channel: Arc<AtomicChannel<LinkedList<PathBuf>>> = Arc::new(AtomicChannel::new());
        let thread_manager: ThreadManager = ThreadManager::new(threads);
        let metrics_thread_worker: ThreadLoop = ThreadLoop::new();

        SearchThreadScheduler {
            batch_size,
            threads,
            file_search,
            files_channel,
            queue_channel,
            thread_manager,
            metrics_thread_worker,
        }
    }

    pub fn search_files(&self, update_rate: Duration) -> HashSet<FileInfo> {
        let root: Result<PathBuf, io::Error> = self.file_search.get_root_path();

        if let Ok(root) = root {
            let search_metrics: Arc<SearchMetrics> = Arc::new(SearchMetrics::new(update_rate));
            let mut queue: LinkedList<PathBuf> = LinkedList::new();
            queue.push_back(root);

            self.metrics_display_thread(&search_metrics);
            self.spawn_walkers(&mut queue, &search_metrics);
        }
        let files: HashSet<FileInfo> = self.get_received_files();
        self.clean_receiver_channels();
        files
    }

    pub fn search_files_benchmark(&self, update_rate: Duration) -> Arc<SearchMetrics> {
        let root: Result<PathBuf, io::Error> = self.file_search.get_root_path();
        let search_metrics: Arc<SearchMetrics> = Arc::new(SearchMetrics::new(update_rate));
        let mut queue: LinkedList<PathBuf> = LinkedList::new();

        if let Ok(root) = root {
            queue.push_back(root);

            self.metrics_display_thread(&search_metrics);
            self.spawn_walkers(&mut queue, &search_metrics);
        }

        let _ = self.get_received_files();
        self.clean_receiver_channels();
        search_metrics
    }
}

impl SearchThreadScheduler {
    fn get_received_files(&self) -> HashSet<FileInfo> {
        let mut files: HashSet<FileInfo> = HashSet::new();
        while let Ok(mut files_receive) = self.files_channel.try_recv() {
            for file_info in files_receive.drain() {
                files.insert(file_info);
            }
        }
        files
    }

    fn spawn_walkers(&self, queue: &mut LinkedList<PathBuf>, search_metrics: &Arc<SearchMetrics>) {
        let progress_metrics: Arc<ProgressMetrics> = search_metrics.get_metrics();

        loop {
            let batch: Vec<PathBuf> = self.get_queue_batch(queue);
            let _ = self.add_batched_thread(batch, search_metrics);

            let search_activity: SearchActivity = self.get_search_activity(queue.len());
            progress_metrics.set_threads(search_activity.busy_threads);

            self.extend_queue(queue, &search_activity);
            self.wait_for_job_queue(&search_activity);

            if self.get_halt_condition(&search_activity, queue) {
                break;
            }
        }

        self.finalize(search_metrics, &progress_metrics);
    }

    fn finalize(
        &self,
        search_metrics: &Arc<SearchMetrics>,
        progress_metrics: &Arc<ProgressMetrics>,
    ) {
        search_metrics.terminate();
        self.metrics_thread_worker.terminate();

        let busy_threads: usize = self.thread_manager.get_busy_threads();
        progress_metrics.set_threads(busy_threads);
        search_metrics.finalize();
    }

    fn wait_for_job_queue(&self, search_activity: &SearchActivity) {
        let job_queue: usize = search_activity.job_queue;
        if job_queue >= self.threads * 2 {
            thread::sleep(Duration::from_micros(1));
        }
    }

    fn metrics_display_thread(&self, search_metrics: &Arc<SearchMetrics>) {
        let search_metrics: Arc<SearchMetrics> = search_metrics.clone();
        let closure = move || {
            search_metrics.blocking_display_progress();
        };
        self.metrics_thread_worker.start(closure);
    }

    fn get_search_activity(&self, queue: usize) -> SearchActivity {
        let busy_threads: usize = self.thread_manager.get_busy_threads();
        let job_queue: usize = self.thread_manager.get_job_queue();
        let queue_buffer: usize = self.queue_channel.get_buffer();

        let search_activity: SearchActivity =
            SearchActivity::new(busy_threads, job_queue, queue_buffer, queue);
        search_activity
    }

    fn get_halt_condition(
        &self,
        search_activity: &SearchActivity,
        queue: &mut LinkedList<PathBuf>,
    ) -> bool {
        if search_activity.all_empty() {
            thread::sleep(Duration::from_millis(1));
            let search_activity: SearchActivity = self.get_search_activity(queue.len());
            if search_activity.all_empty() {
                self.thread_manager.terminate_all();
                return true;
            }
        }
        false
    }

    fn clean_receiver_channels(&self) {
        self.files_channel.clear_receiver();
        self.queue_channel.clear_receiver();
    }

    fn extend_queue(&self, queue: &mut LinkedList<PathBuf>, search_activity: &SearchActivity) {
        let queue_buffer: usize = search_activity.queue_buffer;
        for _ in 0..queue_buffer {
            if let Ok(received) = self.queue_channel.recv() {
                queue.extend(received);
            }
        }
    }

    fn add_batched_thread(&self, batch: Vec<PathBuf>, search_metrics: &Arc<SearchMetrics>) {
        if batch.len() > 0 {
            let search_clone: Arc<FileSearch> = self.file_search.clone();
            let files_channel: Arc<AtomicChannel<HashSet<FileInfo>>> = self.files_channel.clone();
            let queue_channel: Arc<AtomicChannel<LinkedList<PathBuf>>> = self.queue_channel.clone();
            let search_metrics: Arc<SearchMetrics> = search_metrics.clone();

            let closure = move || {
                search_clone.batch_walker(batch, &files_channel, &queue_channel, &search_metrics);
            };

            self.thread_manager.execute(closure);
        }
    }

    fn get_queue_batch(&self, queue: &mut LinkedList<PathBuf>) -> Vec<PathBuf> {
        let batch_size: usize = self.batch_size;
        let mut batch: Vec<PathBuf> = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            if let Some(root) = queue.pop_front() {
                batch.push(root);
            } else {
                break;
            }
        }
        batch
    }
}
