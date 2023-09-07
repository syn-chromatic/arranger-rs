use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub struct ThreadChannel {
    sender: Arc<Mutex<mpsc::Sender<Box<dyn FnOnce() + Send>>>>,
    receiver: Arc<Mutex<mpsc::Receiver<Box<dyn FnOnce() + Send>>>>,
    send_buffer: AtomicUsize,
    receive_buffer: AtomicUsize,
}

impl ThreadChannel {
    pub fn new() -> Self {
        let (sender, receiver): (
            mpsc::Sender<Box<dyn FnOnce() + Send>>,
            mpsc::Receiver<Box<dyn FnOnce() + Send>>,
        ) = mpsc::channel();

        let sender: Arc<Mutex<mpsc::Sender<Box<dyn FnOnce() + Send>>>> =
            Arc::new(Mutex::new(sender));
        let receiver: Arc<Mutex<mpsc::Receiver<Box<dyn FnOnce() + Send>>>> =
            Arc::new(Mutex::new(receiver));

        ThreadChannel {
            sender,
            receiver,
            send_buffer: AtomicUsize::new(0),
            receive_buffer: AtomicUsize::new(0),
        }
    }

    pub fn send(
        &self,
        value: Box<dyn FnOnce() + Send>,
    ) -> Result<(), mpsc::SendError<Box<dyn FnOnce() + Send>>> {
        self.send_buffer.fetch_add(1, Ordering::SeqCst);
        let result: Result<(), mpsc::SendError<Box<dyn FnOnce() + Send>>> =
            self.sender.lock().unwrap().send(value);
        if result.is_ok() {
            self.receive_buffer.fetch_add(1, Ordering::SeqCst);
        } else {
            self.send_buffer.fetch_sub(1, Ordering::SeqCst);
        }
        result
    }

    pub fn recv(&self) -> Result<Box<dyn FnOnce() + Send>, mpsc::RecvError> {
        let received: Result<Box<dyn FnOnce() + Send>, mpsc::RecvError> =
            self.receiver.lock().unwrap().recv();
        if received.is_ok() {
            self.receive_buffer.fetch_sub(1, Ordering::SeqCst);
        }
        received
    }

    pub fn try_recv(&self) -> Result<Box<dyn FnOnce() + Send>, mpsc::TryRecvError> {
        let received: Result<Box<dyn FnOnce() + Send>, mpsc::TryRecvError> =
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

pub struct ThreadManager {
    channel: Arc<ThreadChannel>,
    workers: Vec<ThreadWorker>,
}

impl ThreadManager {
    pub fn new(size: usize) -> ThreadManager {
        let channel: Arc<ThreadChannel> = Arc::new(ThreadChannel::new());
        let workers: Vec<ThreadWorker> = Self::get_workers(size, channel.clone());

        ThreadManager { channel, workers }
    }

    // Should check if threads are terminated
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Box<dyn FnOnce() + Send + 'static> = Box::new(f);
        let _ = self.channel.send(job);
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
        let receive_buffer: usize = self.channel.get_receive_buffer();
        receive_buffer
    }

    pub fn terminate_all(&self) {
        for worker in self.workers.iter() {
            worker.terminate();
        }
        self.clear_receiver_channel();
    }

    pub fn clear_receiver_channel(&self) {
        while let Ok(received) = self.channel.try_recv() {
            drop(received);
        }
    }
}

impl ThreadManager {
    fn get_workers(size: usize, channel: Arc<ThreadChannel>) -> Vec<ThreadWorker> {
        let mut workers: Vec<ThreadWorker> = Vec::with_capacity(size);

        for id in 0..size {
            let mut worker: ThreadWorker = ThreadWorker::new(id, channel.clone());
            worker.start();
            workers.push(worker);
        }
        workers
    }
}

pub struct ThreadWorker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    channel: Arc<ThreadChannel>,
    is_active: Arc<AtomicBool>,
    terminate_signal: Arc<AtomicBool>,
}

impl ThreadWorker {
    pub fn new(id: usize, channel: Arc<ThreadChannel>) -> ThreadWorker {
        let thread: Option<thread::JoinHandle<()>> = None;
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

    pub fn start(&mut self) {
        let worker_loop = self.get_worker_loop();
        let thread: thread::JoinHandle<()> = thread::spawn(move || worker_loop());
        self.thread = Some(thread);
    }

    pub fn terminate(&self) {
        self.terminate_signal.store(true, Ordering::SeqCst);
    }

    pub fn is_active(&self) -> bool {
        let is_active: bool = self.is_active.load(Ordering::SeqCst);
        is_active
    }
}

impl ThreadWorker {
    fn get_worker_loop(&self) -> impl Fn() {
        let channel: Arc<ThreadChannel> = self.channel.clone();
        let is_active: Arc<AtomicBool> = self.is_active.clone();
        let terminate_signal: Arc<AtomicBool> = self.terminate_signal.clone();

        let worker_loop = move || {
            loop {
                if terminate_signal.load(Ordering::SeqCst) {
                    is_active.store(false, Ordering::SeqCst);
                    break;
                }

                let job: Result<Box<dyn FnOnce() + Send>, mpsc::TryRecvError> = channel.try_recv();
                if let Ok(job) = job {
                    is_active.store(true, Ordering::SeqCst);
                    job();
                    is_active.store(false, Ordering::SeqCst);
                } else {
                    thread::sleep(Duration::from_nanos(1));
                }
            }
            is_active.store(false, Ordering::SeqCst);
        };
        worker_loop
    }
}
