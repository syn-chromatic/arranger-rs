use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;

use std::fs::DirEntry;
use std::fs::{Metadata, ReadDir};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::thread;

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

    fn handle_entry(&self, entry: &DirEntry, queue: &mut LinkedList<PathBuf>) {
        if let Ok(metadata) = entry.metadata() {
            // search_progress.display_progress();
            let path: PathBuf = entry.path();

            if metadata.is_dir() {
                queue.push_back(path);
            } else if metadata.is_file() {
                // self.handle_file(metadata, path,  search_progress);
            }
        }
    }

    fn walker(self: &Arc<Self>, root: &PathBuf, sender: mpsc::Sender<LinkedList<PathBuf>>) {
        let entries: ReadDir = match root.read_dir() {
            Ok(entries) => entries,
            Err(_) => return,
        };

        // entries.

        let mut queue: LinkedList<PathBuf> = LinkedList::new();

        for entry in entries {
            if let Ok(entry) = entry.as_ref() {
                self.handle_entry(entry, &mut queue);
            }
        }

        let _ = sender.send(queue);
    }
}

// struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Job>,
// }

// type Job = Box<dyn FnOnce() + Send + 'static>;

// impl ThreadPool {
//     fn new(size: usize) -> ThreadPool {
//         let (sender, receiver) = mpsc::channel();
//         let receiver = Arc::new(Mutex::new(receiver));

//         let mut workers = Vec::with_capacity(size);

//         for id in 0..size {
//             workers.push(Worker::new(id, Arc::clone(&receiver)));
//         }

//         ThreadPool { workers, sender }
//     }

//     fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//         let job: Box<F> = Box::new(f);
//         self.sender.send(job).unwrap();
//     }
// }

// struct Worker {
//     id: usize,
//     thread: Option<thread::JoinHandle<()>>,
// }

// impl Worker {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
//         let thread: thread::JoinHandle<()> = thread::spawn(move || loop {
//             let job: Box<dyn FnOnce() + Send> = receiver.lock().unwrap().recv().unwrap();
//             job();
//         });

//         Worker {
//             id,
//             thread: Some(thread),
//         }
//     }
// }

pub struct MultithreadFileSearch {
    file_search: Arc<FileSearch>,
    max_threads: usize,
}

impl MultithreadFileSearch {
    pub fn new() -> Self {
        let mut file_search: FileSearch = FileSearch::new();
        file_search.set_root("C:/");
        let file_search: Arc<FileSearch> = Arc::new(file_search);
        let max_threads: usize = 128;

        MultithreadFileSearch {
            file_search,
            max_threads,
        }
    }

    pub fn search_files<'a>(&'a self) {
        let root: PathBuf = self.file_search.get_root_path().unwrap();

        self.spawn_walker(root);
    }

    fn wait_for_threads(
        &self,
        rx: &mpsc::Receiver<LinkedList<PathBuf>>,
        thread_handles: &mut Vec<std::thread::JoinHandle<()>>,
        queue: &mut LinkedList<PathBuf>,
    ) {
        loop {
            if thread_handles.len() < self.max_threads {
                break;
            }

            if let Ok(received) = rx.recv_timeout(std::time::Duration::from_millis(100)) {
                queue.extend(received);
            }

            thread_handles.retain(|x| !x.is_finished());
        }
    }

    fn update_thread_handles(&self, thread_handles: &mut Vec<std::thread::JoinHandle<()>>) {
        thread_handles.retain(|x| !x.is_finished());
    }

    fn update_queue_first_pass(
        &self,
        queue: &mut LinkedList<PathBuf>,
        tx: &mpsc::Sender<LinkedList<PathBuf>>,
        rx: &mpsc::Receiver<LinkedList<PathBuf>>,
    ) {
        let current_dir: Option<PathBuf> = queue.pop_front();
        if let Some(current_dir) = current_dir {
            self.file_search.walker(&current_dir, tx.clone());
            if let Ok(received) = rx.recv() {
                queue.extend(received);
            }
        } else {
            return;
        }
    }

    fn spawn_walker<'a>(&'a self, root: PathBuf) {
        let (tx, rx): (
            mpsc::Sender<LinkedList<PathBuf>>,
            mpsc::Receiver<LinkedList<PathBuf>>,
        ) = mpsc::channel();

        let mut queue: LinkedList<PathBuf> = LinkedList::new();
        let mut thread_handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
        queue.push_back(root);

        self.update_queue_first_pass(&mut queue, &tx, &rx);

        while let Some(current_dir) = queue.pop_front() {
            let sender_clone: mpsc::Sender<LinkedList<PathBuf>> = tx.clone();
            let shared_self: Arc<FileSearch> = self.file_search.clone();

            self.update_thread_handles(&mut thread_handles);

            if thread_handles.len() >= self.max_threads {
                self.wait_for_threads(&rx, &mut thread_handles, &mut queue);
            }

            let handle: thread::JoinHandle<()> = thread::spawn(move || {
                shared_self.walker(&current_dir, sender_clone);
            });

            thread_handles.push(handle);

            self.print_active_threads(&thread_handles, &queue);
            if let Ok(received) = rx.recv_timeout(std::time::Duration::from_micros(1)) {
                queue.extend(received);
            }
        }
    }

    fn print_active_threads(
        &self,
        thread_handles: &Vec<std::thread::JoinHandle<()>>,
        queue: &LinkedList<PathBuf>,
    ) {
        print!(
            "Active Threads: {} | Queue: {}  |\r",
            thread_handles.len(),
            queue.len(),
        );
    }
}

#[test]
fn test_multithread() {
    let multithread_search: MultithreadFileSearch = MultithreadFileSearch::new();
    let _ = multithread_search.search_files();
}
