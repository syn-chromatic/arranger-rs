use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use libc;

static RUNNING: AtomicBool = AtomicBool::new(true);

extern "C" fn handle_signal(_: libc::c_int) {
    RUNNING.store(false, Ordering::SeqCst);
}

pub struct InterruptHandler<F, G>
where
    F: FnOnce() + Send + 'static,
    G: Fn() + Send + 'static,
{
    main: F,
    interrupt: G,
}

impl<F, G> InterruptHandler<F, G>
where
    F: FnOnce() + Send + 'static,
    G: Fn() + Send + 'static,
{
    pub fn new(main: F, interrupt: G) -> Self {
        Self { main, interrupt }
    }

    pub fn run(self) {
        unsafe {
            libc::signal(libc::SIGINT, handle_signal as libc::sighandler_t);
        }

        let main_handle: JoinHandle<()> = thread::spawn(move || {
            (self.main)();
        });

        loop {
            thread::sleep(Duration::from_millis(100));

            if !RUNNING.load(Ordering::SeqCst) {
                (self.interrupt)();
                break;
            } else if main_handle.is_finished() {
                break;
            }
        }
    }
}
