use std::fs::Metadata;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::RwLock;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::search::formatters::format_size;
use crate::search::formatters::format_time;

use crate::general::table_display::DynamicTable;
use crate::terminal::ConsoleWriter;

pub struct ProgressMetrics {
    search_counter: AtomicUsize,
    match_counter: AtomicUsize,
    search_bytes: AtomicUsize,
    threads: AtomicUsize,
}

impl ProgressMetrics {
    pub fn new() -> ProgressMetrics {
        let search_counter: AtomicUsize = AtomicUsize::new(0);
        let match_counter: AtomicUsize = AtomicUsize::new(0);
        let search_bytes: AtomicUsize = AtomicUsize::new(0);
        let threads: AtomicUsize = AtomicUsize::new(0);

        ProgressMetrics {
            search_counter,
            match_counter,
            search_bytes,
            threads,
        }
    }

    pub fn increment_search_count(&self) {
        self.search_counter.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_match_count(&self) {
        self.match_counter.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add_search_bytes(&self, metadata: &Metadata) {
        let bytes: usize = metadata.len() as usize;
        self.search_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn set_threads(&self, threads: usize) {
        self.threads.store(threads, Ordering::Relaxed);
    }
}

pub struct SearchMetrics {
    table: Arc<Mutex<DynamicTable>>,
    writer: Arc<Mutex<ConsoleWriter>>,
    metrics: Arc<ProgressMetrics>,
    time: Instant,
    duration: Arc<Mutex<Duration>>,
    display_time: Arc<RwLock<Instant>>,
    display_interval: Duration,
    terminated: AtomicBool,
}

impl SearchMetrics {
    pub fn new(display_interval: Duration) -> Self {
        let table: Arc<Mutex<DynamicTable>> = Arc::new(Mutex::new(DynamicTable::new(0.8, 1)));
        let writer: ConsoleWriter = ConsoleWriter::new();
        writer.setup_console_configuration();
        let writer: Arc<Mutex<ConsoleWriter>> = Arc::new(Mutex::new(writer));
        let metrics: Arc<ProgressMetrics> = Arc::new(ProgressMetrics::new());
        let time: Instant = Instant::now();
        let duration: Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::from_secs(0)));
        let display_time: Arc<RwLock<Instant>> = Arc::new(RwLock::new(Instant::now()));
        let terminated: AtomicBool = AtomicBool::new(false);

        SearchMetrics {
            table,
            writer,
            metrics,
            time,
            duration,
            display_time,
            display_interval,
            terminated,
        }
    }

    pub fn get_metrics(&self) -> Arc<ProgressMetrics> {
        self.metrics.clone()
    }

    pub fn get_duration(&self) -> Duration {
        let duration: Duration = self.time.elapsed();
        if let Ok(mut duration_guard) = self.duration.lock() {
            if self.terminated.load(Ordering::SeqCst) {
                return *duration_guard;
            }
            *duration_guard = duration;
        }
        duration
    }

    pub fn set_search_path(&mut self, path: &PathBuf) {
        let path_string: String = self.get_path_string(&path);
        if let Ok(mut table) = self.table.lock() {
            table.add_parameter_string("Path", &path_string);
        }
    }

    pub fn display_progress(&self) {
        let elapsed_time: Duration = self.get_elapsed_display_time();
        if elapsed_time >= self.display_interval {
            if let Ok(mut table) = self.table.lock() {
                table.update_terminal_width();
            }
            self.write_progress(Ordering::Relaxed);
            self.update_display_time();
        }
    }

    pub fn blocking_display_progress(&self) {
        if let Ok(mut table) = self.table.lock() {
            table.update_terminal_width();
        }
        self.write_progress(Ordering::Relaxed);
        thread::sleep(self.display_interval);
    }

    pub fn finalize(&self) {
        self.write_progress(Ordering::SeqCst);
        if let Ok(mut writer) = self.writer.lock() {
            writer.go_to_end();
            writer.reset_console_configuration();
        }
        println!();
    }

    pub fn terminate(&self) {
        self.terminated.store(true, Ordering::SeqCst);
    }
}

impl SearchMetrics {
    fn update_display_time(&self) {
        if let Ok(mut display_time) = self.display_time.write() {
            *display_time = Instant::now();
        }
    }

    fn get_elapsed_display_time(&self) -> Duration {
        if let Ok(display_time) = self.display_time.read() {
            let elapsed_time: Duration = display_time.elapsed();
            return elapsed_time;
        }
        Duration::from_secs(0)
    }

    fn write_progress(&self, ordering: Ordering) {
        let search_bytes: usize = self.metrics.search_bytes.load(ordering);
        let match_counter: usize = self.metrics.match_counter.load(ordering);
        let search_counter: usize = self.metrics.search_counter.load(ordering);
        let threads: usize = self.metrics.threads.load(ordering);

        let size_string: String = format_size(search_bytes);
        let time_string: String = format_time(self.get_duration().as_nanos());

        if let Ok(mut table) = self.table.lock() {
            table.add_parameter("Match", match_counter);
            table.add_parameter("Search", search_counter);
            table.add_parameter_string("Size", &size_string);
            table.add_parameter("Threads", threads);
            table.add_parameter_string("Time", &time_string);

            let table_string: String = table.get_table_string();
            if let Ok(mut writer) = self.writer.lock() {
                writer.write(&table_string);
            }
        }
    }

    fn get_path_string(&self, path: &PathBuf) -> String {
        let mut path_string: String = path.to_string_lossy().to_string();

        let stripped_path: Option<&str> = path_string.strip_prefix(r"\\?\");
        if let Some(stripped_path) = stripped_path {
            path_string = stripped_path.to_string();
        }
        path_string
    }
}
