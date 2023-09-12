use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadLoop {
    thread: Mutex<Option<thread::JoinHandle<()>>>,
    is_active: Arc<AtomicBool>,
    is_busy: Arc<AtomicBool>,
    termination_signal: Arc<AtomicBool>,
}

impl ThreadLoop {
    pub fn new() -> Self {
        let thread: Mutex<Option<thread::JoinHandle<()>>> = Mutex::new(None);
        let is_active: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let is_busy: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let termination_signal: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        ThreadLoop {
            thread,
            is_active,
            is_busy,
            termination_signal,
        }
    }

    pub fn start<F>(&self, function: F)
    where
        F: Fn() + Send + 'static,
    {
        if !self.is_active() {
            let worker_loop = self.create_worker_loop(function);
            let thread: thread::JoinHandle<()> = thread::spawn(worker_loop);
            if let Ok(mut thread_guard) = self.thread.lock() {
                *thread_guard = Some(thread);
            }
        }
    }

    pub fn terminate(&self) {
        self.send_termination_signal();
        self.join();
    }

    pub fn is_active(&self) -> bool {
        let is_active: bool = self.is_active.load(Ordering::Acquire);
        is_active
    }

    pub fn is_busy(&self) -> bool {
        let is_busy = self.is_busy.load(Ordering::Acquire);
        is_busy
    }
}

impl ThreadLoop {
    fn send_termination_signal(&self) {
        self.termination_signal.store(true, Ordering::Release);
    }

    fn join(&self) {
        if let Ok(mut thread_option) = self.thread.lock() {
            if let Some(thread) = thread_option.take() {
                let _ = thread.join();
            }
        }
    }

    fn create_worker_loop<F>(&self, function: F) -> impl Fn()
    where
        F: Fn() + Send + 'static,
    {
        let is_active: Arc<AtomicBool> = self.is_active.clone();
        let is_busy: Arc<AtomicBool> = self.is_busy.clone();
        let termination_signal: Arc<AtomicBool> = self.termination_signal.clone();

        let worker_loop = move || {
            is_active.store(true, Ordering::Release);
            while !termination_signal.load(Ordering::Acquire) {
                is_busy.store(true, Ordering::Release);
                function();
                is_busy.store(false, Ordering::Release);
            }
            is_active.store(false, Ordering::Release);
            termination_signal.store(false, Ordering::Release);
        };
        worker_loop
    }
}
