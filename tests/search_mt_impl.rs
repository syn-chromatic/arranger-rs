use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::fs::DirEntry;
use std::fs::{Metadata, ReadDir};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use arranger::search::info::FileInfo;
use arranger::search::mt_search::mt_progress::{ProgressMetrics, SearchMetricsThreaded};
use arranger::search::mt_search::mt_structs::QueueChannel;

pub struct FileSearch {
    root: Option<PathBuf>,
}

impl FileSearch {
    pub fn new() -> Self {
        let root: Option<PathBuf> = None;

        FileSearch { root }
    }

    pub fn set_root<T: AsRef<Path>>(&mut self, root: T) {
        self.root = Some(PathBuf::from(root.as_ref()));
    }
}

impl FileSearch {
    fn get_root_path(&self) -> Result<PathBuf, io::Error> {
        let root: PathBuf = if let Some(root) = &self.root {
            root.to_path_buf()
        } else {
            env::current_dir().unwrap()
        };

        let root: Result<PathBuf, io::Error> = root.canonicalize();
        root
    }

    fn handle_file(
        &self,
        metadata: Metadata,
        file: PathBuf,
        files: &mut HashSet<FileInfo>,
        search_progress: &Arc<SearchMetricsThreaded>,
    ) {
        let progress_metrics: Arc<ProgressMetrics> = search_progress.get_metrics();
        progress_metrics.increment_search_count();
        progress_metrics.add_search_bytes(&metadata);

        if !files.contains(&file) {
            let file_info: FileInfo = FileInfo::new(file, metadata);
            files.insert(file_info);
            progress_metrics.increment_match_count();
        }
    }

    fn handle_entry(
        &self,
        entry: &DirEntry,
        files: &mut HashSet<FileInfo>,
        queue: &mut LinkedList<PathBuf>,
        search_progress: &Arc<SearchMetricsThreaded>,
    ) {
        if let Ok(metadata) = entry.metadata() {
            let path: PathBuf = entry.path();

            if metadata.is_dir() {
                queue.push_back(path);
            } else if metadata.is_file() {
                self.handle_file(metadata, path, files, search_progress);
            }
        }
    }

    fn walker(
        self: &Arc<Self>,
        root: &PathBuf,
        queue_channel: Arc<QueueChannel>,
        search_progress: &Arc<SearchMetricsThreaded>,
    ) {
        let mut files: HashSet<FileInfo> = HashSet::new();

        let entries: ReadDir = match root.read_dir() {
            Ok(entries) => entries,
            Err(_) => return,
        };

        let mut queue: LinkedList<PathBuf> = LinkedList::new();
        for entry in entries {
            if let Ok(entry) = entry.as_ref() {
                self.handle_entry(entry, &mut files, &mut queue, search_progress);
            }
        }

        let _ = queue_channel.send(queue);
    }

    fn batch_walker(
        self: &Arc<Self>,
        batch: Vec<PathBuf>,
        queue_channel: Arc<QueueChannel>,
        search_progress: &Arc<SearchMetricsThreaded>,
    ) {
        for root in batch.iter() {
            self.walker(&root, queue_channel.clone(), search_progress);
        }
    }
}

pub struct SearchThreadManager {
    max_threads: usize,
    batch_size: usize,
    file_search: Arc<FileSearch>,
    active_threads: Arc<AtomicUsize>,
    queue_channel: Arc<QueueChannel>,
    search_progress: Arc<SearchMetricsThreaded>,
    halt: Arc<AtomicBool>,
}

impl SearchThreadManager {
    pub fn new(max_threads: usize, batch_size: usize, file_search: FileSearch) -> Self {
        let file_search: Arc<FileSearch> = Arc::new(file_search);
        let active_threads: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let queue_channel: Arc<QueueChannel> = Arc::new(QueueChannel::new());

        let search_progress: Arc<SearchMetricsThreaded> =
            Arc::new(SearchMetricsThreaded::new(Duration::from_millis(100)));
        let halt: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        SearchThreadManager {
            max_threads,
            batch_size,
            file_search,
            active_threads,
            queue_channel,
            search_progress,
            halt,
        }
    }

    pub fn search_files(&self) {
        let root: Result<PathBuf, io::Error> = self.file_search.get_root_path();

        if let Ok(root) = root {
            let mut queue: LinkedList<PathBuf> = LinkedList::new();
            queue.push_back(root);

            self.spawn_walkers(&mut queue);
        }

        self.search_progress.display_progress_finalize();
    }

    fn set_progress_metrics(
        &self,
        active_threads: usize,
        receive_buffer: usize,
        queue: &LinkedList<PathBuf>,
    ) {
        let progress_metrics: Arc<ProgressMetrics> = self.search_progress.get_metrics();
        progress_metrics.set_threads(active_threads);
        progress_metrics.set_receive_buffer(receive_buffer);
        progress_metrics.set_queue(queue.len());
    }

    fn spawn_walkers(&self, queue: &mut LinkedList<PathBuf>) {
        self.display_progress_loop();

        loop {
            let batch: Vec<PathBuf> = self.get_queue_batch(queue);
            let _ = self.add_batched_thread(batch);

            let active_threads: usize = self.active_threads.load(Ordering::SeqCst);
            let receive_buffer: usize = self.queue_channel.get_receive_buffer();
            self.set_progress_metrics(active_threads, receive_buffer, queue);
            // self.search_progress.display_progress();
            // self.print_activity(&queue, active_threads, receive_buffer);

            for _ in 0..receive_buffer {
                if let Ok(received) = self.queue_channel.recv() {
                    queue.extend(received);
                }
            }

            if active_threads >= self.max_threads {
                self.wait_for_threads();
            }

            if self.get_halt_condition(queue, active_threads, receive_buffer) {
                break;
            }
        }
    }

    fn display_progress_loop(&self) {
        let search_progress: Arc<SearchMetricsThreaded> = self.search_progress.clone();
        let halt: Arc<AtomicBool> = self.halt.clone();

        let _ = thread::spawn(move || loop {
            search_progress.blocking_display_progress();
            if halt.load(Ordering::SeqCst) {
                break;
            }
        });
    }

    fn get_halt_condition(
        &self,
        queue: &LinkedList<PathBuf>,
        active_threads: usize,
        receive_buffer: usize,
    ) -> bool {
        let halt_condition: bool =
            self.get_current_halt_condition(queue, active_threads, receive_buffer);
        if !halt_condition {
            return false;
        }
        thread::sleep(Duration::from_micros(100));
        let halt_condition: bool = self.get_updated_halt_condition(queue);
        halt_condition
    }

    fn get_current_halt_condition(
        &self,
        queue: &LinkedList<PathBuf>,
        active_threads: usize,
        receive_buffer: usize,
    ) -> bool {
        if queue.len() == 0 && active_threads == 0 && receive_buffer == 0 {
            return true;
        }
        false
    }

    fn get_updated_halt_condition(&self, queue: &LinkedList<PathBuf>) -> bool {
        let active_threads: usize = self.active_threads.load(Ordering::SeqCst);
        let receive_buffer: usize = self.queue_channel.get_receive_buffer();
        if queue.len() == 0 && active_threads == 0 && receive_buffer == 0 {
            return true;
        }
        false
    }

    fn wait_for_threads(&self) {
        loop {
            let active_threads: usize = self.active_threads.load(Ordering::SeqCst);
            if active_threads < self.max_threads {
                break;
            }
            thread::sleep(Duration::from_micros(10));
        }
    }

    fn add_batched_thread<'a>(&'a self, batch: Vec<PathBuf>) -> thread::JoinHandle<()> {
        let search_clone: Arc<FileSearch> = self.file_search.clone();
        let active_threads: Arc<AtomicUsize> = self.active_threads.clone();
        let queue_channel: Arc<QueueChannel> = self.queue_channel.clone();
        let search_progress: Arc<SearchMetricsThreaded> = self.search_progress.clone();

        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            active_threads.fetch_add(1, Ordering::SeqCst);
            search_clone.batch_walker(batch, queue_channel, &search_progress);
            active_threads.fetch_sub(1, Ordering::SeqCst);
        });
        handle
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

    fn print_activity(
        &self,
        queue: &LinkedList<PathBuf>,
        active_threads: usize,
        receive_buffer: usize,
    ) {
        print!(
            "Active Threads: {} | Receive Buffer: {} | Queue: {}\x1B[K\r",
            active_threads,
            receive_buffer,
            queue.len(),
        );
    }
}

#[test]
fn test_multithread() {
    println!("\n\n");
    let mut file_search = FileSearch::new();
    file_search.set_root("C:/");

    let max_threads: usize = 12;
    let batch_size: usize = 500;
    let thread_manager: SearchThreadManager =
        SearchThreadManager::new(max_threads, batch_size, file_search);
    thread_manager.search_files();
    println!("\n\n");
}
