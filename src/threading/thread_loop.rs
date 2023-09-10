use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadLoop {
    thread: Mutex<Option<thread::JoinHandle<()>>>,
    is_active: Arc<AtomicBool>,
    terminate_signal: Arc<AtomicBool>,
}

impl ThreadLoop {
    pub fn new() -> Self {
        let thread: Mutex<Option<thread::JoinHandle<()>>> = Mutex::new(None);
        let is_active: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let terminate_signal: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        ThreadLoop {
            thread,
            is_active,
            terminate_signal,
        }
    }

    pub fn start<F>(&self, f: F)
    where
        F: Fn() + Send + 'static,
    {
        if !self.is_active() {
            let thread_loop = self.get_thread_loop(f);
            let thread: thread::JoinHandle<()> = thread::spawn(thread_loop);
            if let Ok(mut thread_guard) = self.thread.lock() {
                *thread_guard = Some(thread);
            }
        }
    }

    pub fn terminate(&self) {
        self.terminate_signal.store(true, Ordering::Release);
        if let Ok(mut thread_option) = self.thread.lock() {
            if let Some(thread) = thread_option.take() {
                let _ = thread.join();
            }
        }
        self.terminate_signal.store(false, Ordering::Release);
    }

    pub fn is_active(&self) -> bool {
        let is_active: bool = self.is_active.load(Ordering::Acquire);
        is_active
    }
}

impl ThreadLoop {
    fn get_thread_loop<F>(&self, job: F) -> impl Fn()
    where
        F: Fn() + Send + 'static,
    {
        let is_active: Arc<AtomicBool> = self.is_active.clone();
        let terminate_signal: Arc<AtomicBool> = self.terminate_signal.clone();

        let worker_loop = move || {
            is_active.store(true, Ordering::Release);
            loop {
                if terminate_signal.load(Ordering::Acquire) {
                    break;
                }
                job();
            }
            is_active.store(false, Ordering::Release);
        };
        worker_loop
    }
}
