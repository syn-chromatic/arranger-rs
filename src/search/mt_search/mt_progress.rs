use std::fs::Metadata;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

use crate::search::formatters::format_size;
use crate::search::formatters::format_time;

use crate::search::mt_search::mt_structs::AtomicCounter;

use crate::general::table_display::DynamicTable;
use crate::terminal::ConsoleWriter;

pub struct SearchMetrics {
    search_counter: AtomicCounter,
    match_counter: AtomicCounter,
    search_bytes: AtomicCounter,
    threads: AtomicCounter,
    queue: AtomicCounter,
    receive_buffer: AtomicCounter,
}

impl SearchMetrics {
    pub fn new() -> SearchMetrics {
        let search_counter: AtomicCounter = AtomicCounter::new(0);
        let match_counter: AtomicCounter = AtomicCounter::new(0);
        let search_bytes: AtomicCounter = AtomicCounter::new(0);
        let threads: AtomicCounter = AtomicCounter::new(0);
        let queue: AtomicCounter = AtomicCounter::new(0);
        let receive_buffer: AtomicCounter = AtomicCounter::new(0);

        SearchMetrics {
            search_counter,
            match_counter,
            search_bytes,
            threads,
            queue,
            receive_buffer,
        }
    }

    pub fn increment_search_count(&self) {
        self.search_counter.add_relaxed(1);
    }

    pub fn increment_match_count(&self) {
        self.match_counter.add_relaxed(1);
    }

    pub fn add_search_bytes(&self, metadata: &Metadata) {
        let bytes: usize = metadata.len() as usize;
        self.search_bytes.add_relaxed(bytes);
    }

    pub fn set_threads(&self, threads: usize) {
        self.threads.store_value_relaxed(threads);
    }

    pub fn set_queue(&self, queue: usize) {
        self.queue.store_value_relaxed(queue);
    }

    pub fn set_receive_buffer(&self, receive_buffer: usize) {
        self.receive_buffer.store_value_relaxed(receive_buffer);
    }
}

pub struct SearchMetricsThreaded {
    table: Arc<Mutex<DynamicTable>>,
    writer: Arc<Mutex<ConsoleWriter>>,
    metrics: Arc<SearchMetrics>,
    time: Instant,
    display_time: Arc<RwLock<Instant>>,
    display_interval: Duration,
    elapsed_time_ns: AtomicCounter,
}

impl SearchMetricsThreaded {
    pub fn new(display_interval: Duration) -> Self {
        let table: Arc<Mutex<DynamicTable>> = Arc::new(Mutex::new(DynamicTable::new(0.8, 1)));
        let writer: Arc<Mutex<ConsoleWriter>> = Arc::new(Mutex::new(ConsoleWriter::new()));
        let metrics: Arc<SearchMetrics> = Arc::new(SearchMetrics::new());
        let time: Instant = Instant::now();
        let display_time: Arc<RwLock<Instant>> = Arc::new(RwLock::new(Instant::now()));
        let elapsed_time_ns: AtomicCounter = AtomicCounter::new(0);

        writer.lock().unwrap().setup_console_configuration();

        SearchMetricsThreaded {
            table,
            writer,
            metrics,
            time,
            display_time,
            display_interval,
            elapsed_time_ns,
        }
    }

    pub fn get_metrics(&self) -> Arc<SearchMetrics> {
        self.metrics.clone()
    }

    pub fn display_progress(&self) {
        let elapsed_time: Duration = self.get_elapsed_display_time();
        if elapsed_time >= self.display_interval {
            self.table.lock().unwrap().update_terminal_width();
            self.write_progress();
            self.update_display_time();
        }
    }

    pub fn display_progress_finalize(&self) {
        self.write_progress();
        self.writer.lock().unwrap().end();
        self.writer.lock().unwrap().reset_console_configuration();
        println!();
    }

    pub fn set_search_path(&mut self, path: &PathBuf) {
        let path_string: String = self.get_path_string(&path);
        let mut table_guard: MutexGuard<'_, DynamicTable> = self.table.lock().unwrap();
        table_guard.add_parameter_string("Path", &path_string);
    }

    pub fn get_elapsed_time_ns(&self) -> usize {
        self.elapsed_time_ns.load_relaxed()
    }
}

impl SearchMetricsThreaded {
    fn update_elapsed_time_ns(&self) {
        let elapsed_time_ns: usize = self.time.elapsed().as_nanos() as usize;
        self.elapsed_time_ns.store_value_relaxed(elapsed_time_ns);
    }

    fn update_display_time(&self) {
        let mut instant_guard: RwLockWriteGuard<'_, Instant> = self.display_time.write().unwrap();
        *instant_guard = Instant::now();
    }

    fn get_elapsed_display_time(&self) -> Duration {
        let instant_guard: RwLockReadGuard<'_, Instant> = self.display_time.read().unwrap();
        let elapsed_time: Duration = instant_guard.elapsed();
        elapsed_time
    }

    fn write_progress(&self) {
        let search_bytes: usize = self.metrics.search_bytes.load_relaxed();
        let match_counter: usize = self.metrics.match_counter.load_relaxed();
        let search_counter: usize = self.metrics.search_counter.load_relaxed();
        let threads: usize = self.metrics.threads.load_relaxed();
        let queue: usize = self.metrics.queue.load_relaxed();
        let receive_buffer: usize = self.metrics.receive_buffer.load_relaxed();

        let size_string: String = format_size(search_bytes);

        self.update_elapsed_time_ns();
        let time_string: String = format_time(self.get_elapsed_time_ns() as u128);

        let mut table_guard: MutexGuard<'_, DynamicTable> = self.table.lock().unwrap();

        table_guard.add_parameter("Match", match_counter);
        table_guard.add_parameter("Search", search_counter);
        table_guard.add_parameter_string("Size", &size_string);
        table_guard.add_parameter("Threads", threads);
        table_guard.add_parameter("Queue", queue);
        table_guard.add_parameter("Buffer", receive_buffer);
        table_guard.add_parameter_string("Time", &time_string);

        let table_string: String = table_guard.get_table_string();
        self.writer.lock().unwrap().write(&table_string);
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
