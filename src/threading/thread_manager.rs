use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::threading::thread_structs::AtomicChannel;

pub struct ThreadManager {
    channel: Arc<AtomicChannel<Box<dyn FnOnce() + Send>>>,
    workers: Vec<ThreadWorker>,
    terminated: AtomicBool,
}

impl ThreadManager {
    pub fn new(size: usize) -> ThreadManager {
        let channel: Arc<AtomicChannel<Box<dyn FnOnce() + Send>>> = Arc::new(AtomicChannel::new());
        let workers: Vec<ThreadWorker> = Self::get_workers(size, channel.clone());
        let terminated: AtomicBool = AtomicBool::new(true);

        ThreadManager {
            channel,
            workers,
            terminated,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Box<dyn FnOnce() + Send + 'static> = Box::new(f);
        let _ = self.channel.send(job);
        if self.is_terminated() {
            self.start_workers();
        }
    }

    pub fn get_active_threads(&self) -> usize {
        let mut active_threads: usize = 0;
        for worker in self.workers.iter() {
            if worker.is_active() {
                active_threads += 1;
            }
        }
        active_threads
    }

    pub fn get_job_queue(&self) -> usize {
        let receive_buffer: usize = self.channel.get_buffer();
        receive_buffer
    }

    pub fn terminate_all(&self) {
        for worker in self.workers.iter() {
            worker.terminate();
        }
        self.terminated.store(true, Ordering::SeqCst);
        self.clear_receiver_channel();
    }

    pub fn is_terminated(&self) -> bool {
        let is_terminated: bool = self.terminated.load(Ordering::SeqCst);
        is_terminated
    }

    pub fn clear_receiver_channel(&self) {
        while let Ok(received) = self.channel.try_recv() {
            drop(received);
        }
    }
}

impl ThreadManager {
    fn get_workers(
        size: usize,
        channel: Arc<AtomicChannel<Box<dyn FnOnce() + Send>>>,
    ) -> Vec<ThreadWorker> {
        let mut workers: Vec<ThreadWorker> = Vec::with_capacity(size);

        for id in 0..size {
            let worker: ThreadWorker = ThreadWorker::new(id, channel.clone());
            workers.push(worker);
        }
        workers
    }

    fn start_workers(&self) {
        self.terminated.store(false, Ordering::SeqCst);
        for worker in self.workers.iter() {
            worker.start();
        }
    }
}

pub struct ThreadWorker {
    id: usize,
    thread: Mutex<Option<thread::JoinHandle<()>>>,
    channel: Arc<AtomicChannel<Box<dyn FnOnce() + Send>>>,
    is_active: Arc<AtomicBool>,
    terminate_signal: Arc<AtomicBool>,
}

impl ThreadWorker {
    pub fn new(id: usize, channel: Arc<AtomicChannel<Box<dyn FnOnce() + Send>>>) -> ThreadWorker {
        let thread: Mutex<Option<thread::JoinHandle<()>>> = Mutex::new(None);
        let is_active: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let terminate_signal: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        ThreadWorker {
            id,
            thread,
            channel,
            is_active,
            terminate_signal,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn start(&self) {
        if !self.is_active() {
            let worker_loop = self.get_worker_loop();
            let thread: thread::JoinHandle<()> = thread::spawn(move || worker_loop());
            if let Ok(mut thread_guard) = self.thread.lock() {
                *thread_guard = Some(thread);
            }
        }
    }

    pub fn terminate(&self) {
        self.terminate_signal.store(true, Ordering::SeqCst);
        if let Ok(mut thread_option) = self.thread.lock() {
            if let Some(thread) = thread_option.take() {
                let _ = thread.join();
            }
        }
        self.terminate_signal.store(false, Ordering::SeqCst);
    }

    pub fn is_active(&self) -> bool {
        let is_active: bool = self.is_active.load(Ordering::SeqCst);
        is_active
    }
}

impl ThreadWorker {
    fn get_worker_loop(&self) -> impl Fn() {
        let channel: Arc<AtomicChannel<Box<dyn FnOnce() + Send>>> = self.channel.clone();
        let is_active: Arc<AtomicBool> = self.is_active.clone();
        let terminate_signal: Arc<AtomicBool> = self.terminate_signal.clone();

        let worker_loop = move || {
            is_active.store(true, Ordering::SeqCst);
            loop {
                if terminate_signal.load(Ordering::SeqCst) {
                    break;
                }

                let job: Result<Box<dyn FnOnce() + Send>, mpsc::TryRecvError> = channel.try_recv();
                if let Ok(job) = job {
                    job();
                } else {
                    thread::sleep(Duration::from_micros(1));
                }
            }
            is_active.store(false, Ordering::SeqCst);
        };
        worker_loop
    }
}

pub struct ThreadLoopWorker {
    thread: Mutex<Option<thread::JoinHandle<()>>>,
    is_active: Arc<AtomicBool>,
    terminate_signal: Arc<AtomicBool>,
}

impl ThreadLoopWorker {
    pub fn new() -> ThreadLoopWorker {
        let thread: Mutex<Option<thread::JoinHandle<()>>> = Mutex::new(None);
        let is_active: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let terminate_signal: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        ThreadLoopWorker {
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
            let worker_loop = self.get_worker_loop(f);
            let thread: thread::JoinHandle<()> = thread::spawn(move || worker_loop());
            if let Ok(mut thread_guard) = self.thread.lock() {
                *thread_guard = Some(thread);
            }
        }
    }

    pub fn terminate(&self) {
        self.terminate_signal.store(true, Ordering::SeqCst);
        if let Ok(mut thread_option) = self.thread.lock() {
            if let Some(thread) = thread_option.take() {
                let _ = thread.join();
            }
        }
        self.terminate_signal.store(false, Ordering::SeqCst);
    }

    pub fn is_active(&self) -> bool {
        let is_active: bool = self.is_active.load(Ordering::SeqCst);
        is_active
    }
}

impl ThreadLoopWorker {
    pub fn get_worker_loop<F>(&self, job: F) -> impl Fn()
    where
        F: Fn() + Send + 'static,
    {
        let is_active: Arc<AtomicBool> = self.is_active.clone();
        let terminate_signal: Arc<AtomicBool> = self.terminate_signal.clone();

        let worker_loop = move || {
            is_active.store(true, Ordering::SeqCst);
            loop {
                if terminate_signal.load(Ordering::SeqCst) {
                    break;
                }
                job();
            }
            is_active.store(false, Ordering::SeqCst);
        };
        worker_loop
    }
}
