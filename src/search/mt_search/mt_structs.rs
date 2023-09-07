use std::collections::{HashSet, LinkedList};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};

use crate::search::info::FileInfo;

pub struct RwUsize {
    value: RwLock<usize>,
}

impl RwUsize {
    pub fn new() -> Self {
        let value: RwLock<usize> = RwLock::new(0);
        RwUsize { value }
    }

    pub fn increment(&self) -> Result<(), &'static str> {
        match self.value.write() {
            Ok(mut write_guard) => {
                *write_guard += 1;
                Ok(())
            }
            Err(_) => Err("Failed to acquire write lock"),
        }
    }

    pub fn decrement(&self) -> Result<(), &'static str> {
        match self.value.write() {
            Ok(mut write_guard) => {
                if *write_guard == 0 {
                    return Err("Value is already 0");
                }
                *write_guard -= 1;
                Ok(())
            }
            Err(_) => Err("Failed to acquire write lock"),
        }
    }

    pub fn add(&self, value: usize) -> Result<(), &'static str> {
        match self.value.write() {
            Ok(mut write_guard) => {
                *write_guard += value;
                Ok(())
            }
            Err(_) => Err("Failed to acquire write lock"),
        }
    }

    pub fn get_value(&self) -> Result<usize, &'static str> {
        match self.value.read() {
            Ok(read_guard) => Ok(*read_guard),
            Err(_) => Err("Failed to acquire read lock"),
        }
    }
}

pub struct QueueChannel {
    sender: Arc<Mutex<mpsc::Sender<LinkedList<PathBuf>>>>,
    receiver: Arc<Mutex<mpsc::Receiver<LinkedList<PathBuf>>>>,
    send_buffer: AtomicUsize,
    receive_buffer: AtomicUsize,
}

impl QueueChannel {
    pub fn new() -> Self {
        let (sender, receiver): (
            mpsc::Sender<LinkedList<PathBuf>>,
            mpsc::Receiver<LinkedList<PathBuf>>,
        ) = mpsc::channel();

        let sender: Arc<Mutex<mpsc::Sender<LinkedList<PathBuf>>>> = Arc::new(Mutex::new(sender));
        let receiver: Arc<Mutex<mpsc::Receiver<LinkedList<PathBuf>>>> =
            Arc::new(Mutex::new(receiver));

        QueueChannel {
            sender,
            receiver,
            send_buffer: AtomicUsize::new(0),
            receive_buffer: AtomicUsize::new(0),
        }
    }

    pub fn send(
        &self,
        value: LinkedList<PathBuf>,
    ) -> Result<(), mpsc::SendError<LinkedList<PathBuf>>> {
        self.send_buffer.fetch_add(1, Ordering::SeqCst);
        let result: Result<(), mpsc::SendError<LinkedList<PathBuf>>> =
            self.sender.lock().unwrap().send(value);
        if result.is_ok() {
            self.receive_buffer.fetch_add(1, Ordering::SeqCst);
        } else {
            self.send_buffer.fetch_sub(1, Ordering::SeqCst);
        }
        result
    }

    pub fn recv(&self) -> Result<LinkedList<PathBuf>, mpsc::RecvError> {
        let received: Result<LinkedList<PathBuf>, mpsc::RecvError> =
            self.receiver.lock().unwrap().recv();
        if received.is_ok() {
            self.receive_buffer.fetch_sub(1, Ordering::SeqCst);
        }
        received
    }

    pub fn try_recv(&self) -> Result<LinkedList<PathBuf>, mpsc::TryRecvError> {
        let received: Result<LinkedList<PathBuf>, mpsc::TryRecvError> =
            self.receiver.lock().unwrap().try_recv();
        if received.is_ok() {
            self.receive_buffer.fetch_sub(1, Ordering::SeqCst);
        }
        received
    }

    pub fn get_send_buffer(&self) -> usize {
        let send_buffer: usize = self.send_buffer.load(Ordering::SeqCst);
        send_buffer
    }

    pub fn get_receive_buffer(&self) -> usize {
        let receive_buffer: usize = self.receive_buffer.load(Ordering::SeqCst);
        receive_buffer
    }
}

pub struct FilesChannel {
    sender: Arc<Mutex<mpsc::Sender<HashSet<FileInfo>>>>,
    receiver: Arc<Mutex<mpsc::Receiver<HashSet<FileInfo>>>>,
    send_buffer: AtomicUsize,
    receive_buffer: AtomicUsize,
}

impl FilesChannel {
    pub fn new() -> Self {
        let (sender, receiver): (
            mpsc::Sender<HashSet<FileInfo>>,
            mpsc::Receiver<HashSet<FileInfo>>,
        ) = mpsc::channel();

        let sender: Arc<Mutex<mpsc::Sender<HashSet<FileInfo>>>> = Arc::new(Mutex::new(sender));
        let receiver: Arc<Mutex<mpsc::Receiver<HashSet<FileInfo>>>> =
            Arc::new(Mutex::new(receiver));

        FilesChannel {
            sender,
            receiver,
            send_buffer: AtomicUsize::new(0),
            receive_buffer: AtomicUsize::new(0),
        }
    }

    pub fn send(&self, value: HashSet<FileInfo>) -> Result<(), mpsc::SendError<HashSet<FileInfo>>> {
        self.send_buffer.fetch_add(1, Ordering::SeqCst);
        let result: Result<(), mpsc::SendError<HashSet<FileInfo>>> =
            self.sender.lock().unwrap().send(value);
        if result.is_ok() {
            self.receive_buffer.fetch_add(1, Ordering::SeqCst);
        } else {
            self.send_buffer.fetch_sub(1, Ordering::SeqCst);
        }
        result
    }

    pub fn recv(&self) -> Result<HashSet<FileInfo>, mpsc::RecvError> {
        let received: Result<HashSet<FileInfo>, mpsc::RecvError> =
            self.receiver.lock().unwrap().recv();
        if received.is_ok() {
            self.receive_buffer.fetch_sub(1, Ordering::SeqCst);
        }
        received
    }

    pub fn try_recv(&self) -> Result<HashSet<FileInfo>, mpsc::TryRecvError> {
        let received: Result<HashSet<FileInfo>, mpsc::TryRecvError> =
            self.receiver.lock().unwrap().try_recv();
        if received.is_ok() {
            self.receive_buffer.fetch_sub(1, Ordering::SeqCst);
        }
        received
    }

    pub fn get_send_buffer(&self) -> usize {
        let send_buffer: usize = self.send_buffer.load(Ordering::SeqCst);
        send_buffer
    }

    pub fn get_receive_buffer(&self) -> usize {
        let receive_buffer: usize = self.receive_buffer.load(Ordering::SeqCst);
        receive_buffer
    }
}
