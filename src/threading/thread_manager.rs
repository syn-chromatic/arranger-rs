use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::threading::AtomicChannel;

pub struct ThreadManager {
    channel: Arc<AtomicChannel<Box<dyn FnOnce() + Send>>>,
    workers: Vec<ThreadWorker>,
    is_terminated: AtomicBool,
}

impl ThreadManager {
    pub fn new(size: usize) -> Self {
        let channel: Arc<AtomicChannel<Box<dyn FnOnce() + Send>>> = Arc::new(AtomicChannel::new());
        let workers: Vec<ThreadWorker> = Self::get_workers(size, channel.clone());
        let is_terminated: AtomicBool = AtomicBool::new(true);

        ThreadManager {
            channel,
            workers,
            is_terminated,
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

    pub fn get_busy_threads(&self) -> usize {
        let mut busy_threads: usize = 0;
        for worker in self.workers.iter() {
            if worker.is_busy() {
                busy_threads += 1;
            }
        }
        busy_threads
    }

    pub fn get_job_queue(&self) -> usize {
        let job_queue: usize = self.channel.get_buffer();
        job_queue
    }

    pub fn terminate_all(&self) {
        for worker in self.workers.iter() {
            worker.send_terminate_signal();
        }

        for worker in self.workers.iter() {
            worker.wait_for_termination();
        }

        self.is_terminated.store(true, Ordering::Release);
        self.clean_receiver();
    }

    pub fn is_terminated(&self) -> bool {
        let is_terminated: bool = self.is_terminated.load(Ordering::Acquire);
        is_terminated
    }

    pub fn clean_receiver(&self) {
        self.channel.clean_receiver();
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
        self.is_terminated.store(false, Ordering::Release);
        for worker in self.workers.iter() {
            worker.start();
        }
    }
}

struct ThreadWorker {
    id: usize,
    thread: Mutex<Option<thread::JoinHandle<()>>>,
    channel: Arc<AtomicChannel<Box<dyn FnOnce() + Send>>>,
    is_active: Arc<AtomicBool>,
    is_busy: Arc<AtomicBool>,
    terminate_signal: Arc<AtomicBool>,
}

impl ThreadWorker {
    pub fn new(id: usize, channel: Arc<AtomicChannel<Box<dyn FnOnce() + Send>>>) -> Self {
        let thread: Mutex<Option<thread::JoinHandle<()>>> = Mutex::new(None);
        let is_active: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let is_busy: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let terminate_signal: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        ThreadWorker {
            id,
            thread,
            channel,
            is_active,
            is_busy,
            terminate_signal,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn start(&self) {
        if !self.is_active() {
            let worker_loop = self.get_worker_loop();
            let thread: thread::JoinHandle<()> = thread::spawn(worker_loop);
            if let Ok(mut thread_guard) = self.thread.lock() {
                *thread_guard = Some(thread);
            }
        }
    }

    pub fn is_active(&self) -> bool {
        let is_active: bool = self.is_active.load(Ordering::Acquire);
        is_active
    }

    pub fn is_busy(&self) -> bool {
        let is_busy: bool = self.is_busy.load(Ordering::Acquire);
        is_busy
    }

    pub fn send_terminate_signal(&self) {
        self.terminate_signal.store(true, Ordering::Release);
    }

    pub fn wait_for_termination(&self) {
        if let Ok(mut thread_option) = self.thread.lock() {
            if let Some(thread) = thread_option.take() {
                let _ = thread.join();
            }
        }
        self.terminate_signal.store(false, Ordering::Release);
    }
}

impl ThreadWorker {
    fn get_worker_loop(&self) -> impl Fn() {
        let channel: Arc<AtomicChannel<Box<dyn FnOnce() + Send>>> = self.channel.clone();
        let is_active: Arc<AtomicBool> = self.is_active.clone();
        let is_busy: Arc<AtomicBool> = self.is_busy.clone();
        let terminate_signal: Arc<AtomicBool> = self.terminate_signal.clone();

        let worker_loop = move || {
            let recv_timeout: Duration = Duration::from_micros(1);
            is_active.store(true, Ordering::Release);
            loop {
                if terminate_signal.load(Ordering::Acquire) {
                    break;
                }

                if let Ok(job) = channel.recv_timeout(recv_timeout) {
                    is_busy.store(true, Ordering::Release);
                    job();
                    is_busy.store(false, Ordering::Release);
                }
            }
            is_active.store(false, Ordering::Release);
        };
        worker_loop
    }
}
