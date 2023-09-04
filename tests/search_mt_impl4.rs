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
        // sender: mpsc::Sender<LinkedList<PathBuf>>,
        file_counter: Arc<RwUsize>,
        queue_channel: Arc<Mutex<QueueChannel>>,
    ) {
        let entries: ReadDir = match root.read_dir() {
            Ok(entries) => entries,
            Err(_) => return,
        };

        let mut queue: LinkedList<PathBuf> = LinkedList::new();
        for entry in entries {
            if let Ok(entry) = entry.as_ref() {
                let file_count: usize = self.handle_entry(entry, &mut queue);
                let _ = file_counter.add(file_count);
            }
        }

        // let _ = sender.send(queue);
        let _ = queue_channel.lock().unwrap().send(queue);
    }

    fn batch_walker(
        self: &Arc<Self>,
        batch: Vec<PathBuf>,
        // sender: mpsc::Sender<LinkedList<PathBuf>>,
        file_counter: Arc<RwUsize>,
        queue_channel: Arc<Mutex<QueueChannel>>,
    ) {
        for root in batch.iter() {
            self.walker(&root, file_counter.clone(), queue_channel.clone());
        }
    }
}

pub struct QueueChannel {
    tx: mpsc::Sender<LinkedList<PathBuf>>,
    rx: mpsc::Receiver<LinkedList<PathBuf>>,
    receive_buffer: usize,
}

impl QueueChannel {
    pub fn new() -> Self {
        let (tx, rx): (
            mpsc::Sender<LinkedList<PathBuf>>,
            mpsc::Receiver<LinkedList<PathBuf>>,
        ) = mpsc::channel();

        // let tx: Arc<Mutex<mpsc::Sender<LinkedList<PathBuf>>>> = Arc::new(Mutex::new(tx));
        // let rx: Arc<Mutex<mpsc::Receiver<LinkedList<PathBuf>>>> = Arc::new(Mutex::new(rx));
        let receive_buffer: usize = 0;

        QueueChannel {
            tx,
            rx,
            receive_buffer,
        }
    }

    pub fn send(&mut self, mut value: LinkedList<PathBuf>) -> Result<(), ()> {
        loop {
            let result = self.tx.send(value);
            if let Err(error) = result {
                value = error.0;
            } else {
                break;
            }
        }

        self.receive_buffer += 1;
        // // while let Err(sent) = self.tx.send(value) {

        // // }s
        // let sent: Result<(), mpsc::SendError<LinkedList<PathBuf>>> = self.tx.send(value);
        // if sent.is_ok() {
        //     self.receive_buffer += 1;
        // }
        Ok(())
    }

    pub fn try_recv(&mut self) -> Result<LinkedList<PathBuf>, mpsc::TryRecvError> {
        let received: Result<LinkedList<PathBuf>, mpsc::TryRecvError> = self.rx.try_recv();

        if received.is_ok() {
            if self.receive_buffer > 0 {
                self.receive_buffer -= 1;
            }
        }
        received
    }

    pub fn get_receive_buffer(&self) -> usize {
        self.receive_buffer
    }
}

pub struct ThreadedWalker {
    max_threads: usize,
    batch_size: usize,
    file_search: Arc<FileSearch>,
    active_threads: Arc<RwUsize>,
    file_counter: Arc<RwUsize>,
    queue_channel: Arc<Mutex<QueueChannel>>,
}

impl ThreadedWalker {
    pub fn new(max_threads: usize, batch_size: usize) -> Self {
        let mut file_search: FileSearch = FileSearch::new();
        file_search.set_root("C:/");
        let file_search: Arc<FileSearch> = Arc::new(file_search);
        let active_threads: Arc<RwUsize> = Arc::new(RwUsize::new());
        let file_counter: Arc<RwUsize> = Arc::new(RwUsize::new());
        let queue_channel: Arc<Mutex<QueueChannel>> = Arc::new(Mutex::new(QueueChannel::new()));

        ThreadedWalker {
            max_threads,
            batch_size,
            file_search,
            active_threads,
            file_counter,
            queue_channel,
        }
    }

    fn wait_for_threads(&self) {
        loop {
            let active_threads: usize = self.active_threads.get_count().unwrap();
            if active_threads < self.max_threads {
                break;
            }
            thread::sleep(Duration::from_micros(10));
        }
    }

    fn update_queue_pass(
        &self,
        queue: &mut LinkedList<PathBuf>,
        sender: &mpsc::Sender<LinkedList<PathBuf>>,
        receiver: &mpsc::Receiver<LinkedList<PathBuf>>,
    ) {
        let current_dir: Option<PathBuf> = queue.pop_front();
        let file_counter: Arc<RwUsize> = self.file_counter.clone();
        let queue_channel = self.queue_channel.clone();

        if let Some(current_dir) = current_dir {
            self.file_search
                .walker(&current_dir, file_counter, queue_channel);

            if let Ok(received) = self.queue_channel.lock().unwrap().try_recv() {
                queue.extend(received);
            }
        } else {
            return;
        }
    }

    fn add_batched_thread<'a>(
        &'a self,
        batch: Vec<PathBuf>,
        sender: &mpsc::Sender<LinkedList<PathBuf>>,
    ) -> thread::JoinHandle<()> {
        let queue_channel: Arc<Mutex<QueueChannel>> = self.queue_channel.clone();
        // let sender_clone: mpsc::Sender<LinkedList<PathBuf>> = sender.clone();
        let search_clone: Arc<FileSearch> = self.file_search.clone();
        let active_threads: Arc<RwUsize> = self.active_threads.clone();
        let file_counter: Arc<RwUsize> = self.file_counter.clone();

        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            let _ = active_threads.increment().unwrap();
            search_clone.batch_walker(batch, file_counter, queue_channel);
            let _ = active_threads.decrement().unwrap();
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

    fn spawn_walker<'a>(&'a self, root: PathBuf) {
        let (sender, receiver): (
            mpsc::Sender<LinkedList<PathBuf>>,
            mpsc::Receiver<LinkedList<PathBuf>>,
        ) = mpsc::channel();

        let mut queue: LinkedList<PathBuf> = LinkedList::new();
        queue.push_back(root);
        self.update_queue_pass(&mut queue, &sender, &receiver);

        let mut receive_tries: usize = 0;
        let max_tries = 5;
        loop {
            let batch: Vec<PathBuf> = self.get_queue_batch(&mut queue);
            let active_threads: usize = self.active_threads.get_count().unwrap();

            if active_threads >= self.max_threads {
                self.wait_for_threads();
            }

            let _ = self.add_batched_thread(batch, &sender);

            // while let Ok(received) = receiver.recv_timeout(Duration::from_micros(1)) {
            //     queue.extend(received);
            // }

            let active_threads: usize = self.active_threads.get_count().unwrap();
            let receive_buffer: usize = self.queue_channel.lock().unwrap().get_receive_buffer();
            self.print_activity(&queue, active_threads, receive_buffer);

            if receive_buffer == 0 && active_threads == 0 {
                if receive_tries == max_tries {
                    break;
                }
                receive_tries += 1;
            } else {
                receive_tries = 0;
            }

            for _ in 0..receive_buffer {
                if let Ok(receive) = self.queue_channel.lock().unwrap().try_recv() {
                    queue.extend(receive);
                }
            }

            // println!(
            //     "Receive Buffer: {}",
            //     self.queue_channel.lock().unwrap().get_receive_buffer()
            // );
            // let active_threads: usize = self.active_threads.get_count().unwrap();
            // if queue.len() == 0 && active_threads == 0 {
            //     break;
            // }
        }
    }

    fn print_activity(
        &self,
        queue: &LinkedList<PathBuf>,
        active_threads: usize,
        receive_buffer: usize,
    ) {
        let file_count: usize = self.file_counter.get_count().unwrap();
        print!(
            "Active Threads: {} | Receive Buffer: {} | Queue: {} | File Count: {} | \r",
            active_threads,
            receive_buffer,
            queue.len(),
            file_count,
        );
    }
}

struct ThreadedSearch {
    walker: Arc<ThreadedWalker>,
}

impl ThreadedSearch {
    pub fn new(max_threads: usize, batch_size: usize) -> Self {
        let walker: ThreadedWalker = ThreadedWalker::new(max_threads, batch_size);
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
    let threaded_search: ThreadedSearch = ThreadedSearch::new(12, 100);
    let _ = threaded_search.search_files();
    println!();
    // }
}
