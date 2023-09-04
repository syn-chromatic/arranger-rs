pub mod rw_usize;
use crate::rw_usize::RwUsize;

use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::fs::DirEntry;
use std::fs::{Metadata, ReadDir};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use arranger::search::info::FileInfo;
use arranger::search::progress::SearchProgress;

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
        search_progress: &mut SearchProgress,
    ) {
        search_progress.increment_search_count();
        search_progress.add_search_bytes(&metadata);

        if !files.contains(&file) {
            let file_info: FileInfo = FileInfo::new(file, metadata);
            files.insert(file_info);
            search_progress.increment_match_count();
        }
    }

    fn handle_entry(&self, entry: &DirEntry, queue: &mut LinkedList<PathBuf>) -> usize {
        let mut file_count: usize = 0;
        if let Ok(metadata) = entry.metadata() {
            // search_progress.display_progress();
            let path: PathBuf = entry.path();

            if metadata.is_dir() {
                queue.push_back(path);
            } else if metadata.is_file() {
                file_count += 1;
                // self.handle_file(metadata, path,  search_progress);
            }
        }
        file_count
    }

    fn walker(
        self: &Arc<Self>,
        root: &PathBuf,
        sender: mpsc::Sender<LinkedList<PathBuf>>,
        file_counter: Arc<RwUsize>,
    ) {
        let entries: ReadDir = match root.read_dir() {
            Ok(entries) => entries,
            Err(_) => return,
        };

        let mut queue: LinkedList<PathBuf> = LinkedList::new();
        for entry in entries {
            if let Ok(entry) = entry.as_ref() {
                let file_count: usize = self.handle_entry(entry, &mut queue);
                file_counter.add(file_count);
            }
        }

        let _ = sender.send(queue);
    }
}

pub struct ThreadedWalker {
    file_search: Arc<FileSearch>,
    max_threads: usize,
    active_threads: Arc<RwUsize>,
    file_counter: Arc<RwUsize>,
}

impl ThreadedWalker {
    pub fn new(max_threads: usize) -> Self {
        let mut file_search: FileSearch = FileSearch::new();
        file_search.set_root("C:/");
        let file_search: Arc<FileSearch> = Arc::new(file_search);
        let active_threads: Arc<RwUsize> = Arc::new(RwUsize::new());
        let file_counter: Arc<RwUsize> = Arc::new(RwUsize::new());

        ThreadedWalker {
            file_search,
            max_threads,
            active_threads,
            file_counter,
        }
    }

    fn wait_for_threads(&self) {
        loop {
            let active_threads: usize = self.active_threads.get_count().unwrap();
            if active_threads < self.max_threads {
                break;
            }
            thread::sleep(Duration::from_nanos(1));
        }
    }

    fn update_thread_handles(&self, thread_handles: &mut Vec<thread::JoinHandle<()>>) {
        thread_handles.retain(|x| !x.is_finished());
    }

    fn update_queue_pass(
        &self,
        queue: &mut LinkedList<PathBuf>,
        sender: &mpsc::Sender<LinkedList<PathBuf>>,
        receiver: &mpsc::Receiver<LinkedList<PathBuf>>,
    ) {
        let current_dir: Option<PathBuf> = queue.pop_front();
        let file_counter: Arc<RwUsize> = self.file_counter.clone();

        if let Some(current_dir) = current_dir {
            self.file_search
                .walker(&current_dir, sender.clone(), file_counter);
            if let Ok(received) = receiver.recv() {
                queue.extend(received);
            }
        } else {
            return;
        }
    }

    fn add_thread<'a>(
        &'a self,
        current_dir: PathBuf,
        sender: &mpsc::Sender<LinkedList<PathBuf>>,
    ) -> thread::JoinHandle<()> {
        let sender_clone: mpsc::Sender<LinkedList<PathBuf>> = sender.clone();
        let search_clone: Arc<FileSearch> = self.file_search.clone();
        let active_threads: Arc<RwUsize> = self.active_threads.clone();
        let file_counter: Arc<RwUsize> = self.file_counter.clone();

        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            let _ = active_threads.increment().unwrap();
            search_clone.walker(&current_dir, sender_clone, file_counter);
            let _ = active_threads.decrement().unwrap();
        });
        handle
    }

    fn spawn_walker<'a>(&'a self, root: PathBuf) {
        let (sender, receiver): (
            mpsc::Sender<LinkedList<PathBuf>>,
            mpsc::Receiver<LinkedList<PathBuf>>,
        ) = mpsc::channel();

        let mut queue: LinkedList<PathBuf> = LinkedList::new();

        queue.push_back(root);

        self.update_queue_pass(&mut queue, &sender, &receiver);

        while let Some(current_dir) = queue.pop_front() {
            let active_threads: usize = self.active_threads.get_count().unwrap();

            if active_threads >= self.max_threads {
                self.wait_for_threads();
            }

            let _ = self.add_thread(current_dir, &sender);
            self.print_activity(&queue, active_threads);

            while let Ok(received) = receiver.recv_timeout(Duration::from_micros(1)) {
                queue.extend(received);
            }
        }
    }

    fn print_activity(&self, queue: &LinkedList<PathBuf>, active_threads: usize) {
        let file_count = self.file_counter.get_count().unwrap();
        print!(
            "Active Threads: {} | Queue: {} | File Count: {} | \r",
            active_threads,
            queue.len(),
            file_count,
        );
    }
}

struct ThreadedSearch {
    walker: Arc<ThreadedWalker>,
}

impl ThreadedSearch {
    pub fn new(max_threads: usize) -> Self {
        let walker: ThreadedWalker = ThreadedWalker::new(max_threads);
        let walker: Arc<ThreadedWalker> = Arc::new(walker);
        ThreadedSearch { walker }
    }

    pub fn search_files<'a>(&'a self) {
        let walker: Arc<ThreadedWalker> = self.walker.clone();
        let root: PathBuf = walker.file_search.get_root_path().unwrap();
        walker.spawn_walker(root);
    }
}

#[test]
fn test_multithread() {
    // loop {
    let threaded_search: ThreadedSearch = ThreadedSearch::new(12);
    let _ = threaded_search.search_files();
    // }
}
