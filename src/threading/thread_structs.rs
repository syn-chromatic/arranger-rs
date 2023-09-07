use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

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

pub struct AtomicChannel<T> {
    sender: Arc<Mutex<mpsc::Sender<T>>>,
    receiver: Arc<Mutex<mpsc::Receiver<T>>>,
    buffer: AtomicUsize,
}

impl<T> AtomicChannel<T> {
    pub fn new() -> Self {
        let (sender, receiver): (mpsc::Sender<T>, mpsc::Receiver<T>) = mpsc::channel();
        let sender: Arc<Mutex<mpsc::Sender<T>>> = Arc::new(Mutex::new(sender));
        let receiver: Arc<Mutex<mpsc::Receiver<T>>> = Arc::new(Mutex::new(receiver));
        let buffer: AtomicUsize = AtomicUsize::new(0);

        AtomicChannel {
            sender,
            receiver,
            buffer,
        }
    }

    pub fn send(&self, value: T) -> Result<(), mpsc::SendError<T>> {
        if let Ok(sender_guard) = self.sender.lock() {
            let sent_result: Result<(), mpsc::SendError<T>> = sender_guard.send(value);
            if sent_result.is_ok() {
                self.buffer.fetch_add(1, Ordering::SeqCst);
            }
            return sent_result;
        }
        Err(mpsc::SendError(value))
    }

    pub fn send_timeout(&self, mut value: T, timeout: Duration) -> Result<(), mpsc::SendError<T>> {
        let now: Instant = Instant::now();
        while let Err(error) = self.send(value) {
            if now.elapsed() == timeout {
                return Err(error);
            }
            value = error.0;
        }
        Ok(())
    }

    pub fn recv(&self) -> Result<T, mpsc::RecvError> {
        if let Ok(receiver_guard) = self.receiver.lock() {
            let received_result: Result<T, mpsc::RecvError> = receiver_guard.recv();
            if received_result.is_ok() {
                self.buffer.fetch_sub(1, Ordering::SeqCst);
            }
            return received_result;
        }
        Err(mpsc::RecvError)
    }

    pub fn try_recv(&self) -> Result<T, mpsc::TryRecvError> {
        if let Ok(receiver_guard) = self.receiver.lock() {
            let received_result: Result<T, mpsc::TryRecvError> = receiver_guard.try_recv();
            if received_result.is_ok() {
                self.buffer.fetch_sub(1, Ordering::SeqCst);
            }
            return received_result;
        }
        Err(mpsc::TryRecvError::Disconnected)
    }

    pub fn get_buffer(&self) -> usize {
        let receive_buffer: usize = self.buffer.load(Ordering::SeqCst);
        receive_buffer
    }

    pub fn clean_receiver(&self) {
        while let Ok(value) = self.try_recv() {
            drop(value);
        }
    }
}
