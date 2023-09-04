use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
    active_count: Arc<AtomicUsize>,
}

type Job = Option<Box<dyn FnOnce() + Send + 'static>>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver): (
            mpsc::Sender<Option<Box<dyn FnOnce() + Send>>>,
            mpsc::Receiver<Option<Box<dyn FnOnce() + Send>>>,
        ) = mpsc::channel();
        let receiver: Arc<Mutex<mpsc::Receiver<Option<Box<dyn FnOnce() + Send>>>>> =
            Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        let active_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

        for id in 0..size {
            let active_count: Arc<AtomicUsize> = active_count.clone();
            workers.push(Worker::new(id, Arc::clone(&receiver), active_count));
        }

        ThreadPool {
            workers,
            sender,
            active_count,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Box<dyn FnOnce() + Send + 'static> = Box::new(f);
        self.sender
            .send(Some(job))
            .expect("Failed to send job to workers.");
    }

    pub fn has_threads_working(&self) -> bool {
        self.active_count.load(Ordering::SeqCst) > 0
    }

    pub fn get_active_threads(&self) -> usize {
        let active_threads: usize = self.active_count.load(Ordering::SeqCst);
        active_threads
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
        active_count: Arc<AtomicUsize>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let job_option = receiver
                .lock()
                .expect("Receiver lock failed")
                .recv()
                .expect("Failed to receive job.");

            match job_option {
                Some(job) => {
                    active_count.fetch_add(1, Ordering::SeqCst);
                    job();
                    active_count.fetch_sub(1, Ordering::SeqCst);
                }
                None => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(None).expect("Failed to terminate worker.");
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().expect("Failed to join on the worker thread.");
            }
        }
    }
}
