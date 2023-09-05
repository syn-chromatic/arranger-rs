use std::collections::LinkedList;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};

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
                    return Err("Thread count is already 0");
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

pub struct AtomicCounter {
    value: AtomicUsize,
}

impl AtomicCounter {
    pub fn new(value: usize) -> Self {
        let value: AtomicUsize = AtomicUsize::new(value);
        AtomicCounter { value }
    }

    pub fn add_sequential(&self, value: usize) {
        self.value.fetch_add(value, Ordering::SeqCst);
    }

    pub fn sub_sequential(&self, value: usize) {
        self.value.fetch_sub(value, Ordering::SeqCst);
    }

    pub fn add_relaxed(&self, value: usize) {
        self.value.fetch_add(value, Ordering::Relaxed);
    }

    pub fn sub_relaxed(&self, value: usize) {
        self.value.fetch_sub(value, Ordering::Relaxed);
    }

    pub fn load_sequential(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    pub fn load_relaxed(&self) -> usize {
        self.value.load(Ordering::Relaxed)
    }

    pub fn store_value_sequential(&self, value: usize) {
        self.value.store(value, Ordering::SeqCst);
    }

    pub fn store_value_relaxed(&self, value: usize) {
        self.value.store(value, Ordering::Relaxed);
    }
}

pub struct QueueChannel {
    sender: Arc<Mutex<mpsc::Sender<LinkedList<PathBuf>>>>,
    receiver: Arc<Mutex<mpsc::Receiver<LinkedList<PathBuf>>>>,
    send_buffer: AtomicCounter,
    receive_buffer: AtomicCounter,
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
            send_buffer: AtomicCounter::new(0),
            receive_buffer: AtomicCounter::new(0),
        }
    }

    pub fn send(
        &self,
        value: LinkedList<PathBuf>,
    ) -> Result<(), mpsc::SendError<LinkedList<PathBuf>>> {
        self.send_buffer.add_sequential(1);
        let result: Result<(), mpsc::SendError<LinkedList<PathBuf>>> =
            self.sender.lock().unwrap().send(value);
        if result.is_ok() {
            self.receive_buffer.add_sequential(1);
        } else {
            self.send_buffer.sub_sequential(1);
        }
        result
    }

    pub fn recv(&self) -> Result<LinkedList<PathBuf>, mpsc::RecvError> {
        let received: Result<LinkedList<PathBuf>, mpsc::RecvError> =
            self.receiver.lock().unwrap().recv();
        if received.is_ok() {
            self.receive_buffer.sub_sequential(1);
        }
        received
    }

    pub fn try_recv(&self) -> Result<LinkedList<PathBuf>, mpsc::TryRecvError> {
        let received: Result<LinkedList<PathBuf>, mpsc::TryRecvError> =
            self.receiver.lock().unwrap().try_recv();
        if received.is_ok() {
            self.receive_buffer.sub_sequential(1);
        }
        received
    }

    pub fn get_send_buffer(&self) -> usize {
        let send_buffer: usize = self.send_buffer.load_sequential();
        send_buffer
    }

    pub fn get_receive_buffer(&self) -> usize {
        let receive_buffer: usize = self.receive_buffer.load_sequential();
        receive_buffer
    }
}
