use std::io;
use std::io::Write;
use std::sync::Arc;

use arranger::search::formatters::format_time;
use arranger::search::mt_search::mt_progress::SearchMetrics;
use arranger::search::mt_search::mt_search::{FileSearch, SearchThreadScheduler};

#[test]
fn search_benchmark() {
    println!("1. Benchmark [Capture None]");
    println!("2. Benchmark [Capture All]");
    println!("3. Benchmark Simple Regex Pattern [Capture All]");
    println!("4. Benchmark Complex Regex Pattern [Capture All]");

    print!("\nEnter Value: ");
    io::stdout().flush().unwrap();

    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim().parse() {
        Ok(1) => search_no_capture(5),
        Ok(2) => search_capture_all(5),
        Ok(3) => search_regex_capture_all(5),
        Ok(4) => search_regex_complex_capture_all(5),
        _ => println!("Invalid choice"),
    }
}

fn search_no_capture(iterations: usize) {
    println!("[Benchmarking Capture None]");
    let mut file_search: FileSearch = FileSearch::new();
    file_search.set_root("./benches/search_benchmark/benchmark_files");
    file_search.set_exclusive_filename("file_500001.txt");

    let search_scheduler: SearchThreadScheduler = SearchThreadScheduler::new(12, 100, file_search);

    let mut total_time: u128 = 0;
    for _ in 0..iterations {
        let progress: Arc<SearchMetrics> = search_scheduler.search_files_benchmark();
        let elapsed_time: u128 = progress.get_elapsed_time().as_nanos();
        total_time += elapsed_time;
    }

    let average_time: u128 = total_time / iterations as u128;
    let average_time_string: String = format_time(average_time);
    println!("Average Time: {}", average_time_string);
    println!();
}

fn search_capture_all(iterations: usize) {
    println!("[Benchmarking Capture All]");
    let mut file_search: FileSearch = FileSearch::new();
    file_search.set_root("./benches/search_benchmark/benchmark_files");
    file_search.set_exclusive_filename("file_");
    file_search.set_exclusive_extensions(["txt"]);

    let search_scheduler: SearchThreadScheduler = SearchThreadScheduler::new(12, 100, file_search);

    let mut total_time: u128 = 0;
    for _ in 0..iterations {
        let progress: Arc<SearchMetrics> = search_scheduler.search_files_benchmark();
        let elapsed_time: u128 = progress.get_elapsed_time().as_nanos();
        total_time += elapsed_time;
    }

    let average_time: u128 = total_time / iterations as u128;
    let average_time_string: String = format_time(average_time);
    println!("Average Time: {}", average_time_string);
    println!();
}

fn search_regex_capture_all(iterations: usize) {
    println!("[Benchmarking Simple Regex Pattern Capture All]");
    let mut file_search: FileSearch = FileSearch::new();
    file_search.set_root("./benches/search_benchmark/benchmark_files");
    let _ = file_search.set_exclusive_filename_regex(".*");

    let search_scheduler: SearchThreadScheduler = SearchThreadScheduler::new(12, 100, file_search);

    let mut total_time: u128 = 0;
    for _ in 0..iterations {
        let progress: Arc<SearchMetrics> = search_scheduler.search_files_benchmark();
        let elapsed_time: u128 = progress.get_elapsed_time().as_nanos();
        total_time += elapsed_time;
    }

    let average_time: u128 = total_time / iterations as u128;
    let average_time_string: String = format_time(average_time);
    println!("Average Time: {}", average_time_string);
    println!();
}

fn search_regex_complex_capture_all(iterations: usize) {
    println!("[Benchmarking Complex Regex Pattern Capture All]");
    let mut file_search: FileSearch = FileSearch::new();
    file_search.set_root("./benches/search_benchmark/benchmark_files");

    let complex_pattern: String = "^file_(".to_string()
        + "([1-9])|"
        + "([1-9][0-9])|"
        + "([1-9][0-9]{2})|"
        + "([1-9][0-9]{3})|"
        + "([1-9][0-9]{4})|"
        + "(1[0-9]{5}|2[0-9]{5}|3[0-9]{5})|"
        + "4([0-8][0-9]{4}|9[0-9]{3})|"
        + "(500000)"
        + ").*";

    let _ = file_search.set_exclusive_filename_regex(&complex_pattern);

    let search_scheduler: SearchThreadScheduler = SearchThreadScheduler::new(12, 100, file_search);

    let mut total_time: u128 = 0;
    for _ in 0..iterations {
        let progress: Arc<SearchMetrics> = search_scheduler.search_files_benchmark();
        let elapsed_time: u128 = progress.get_elapsed_time().as_nanos();
        total_time += elapsed_time;
    }

    let average_time: u128 = total_time / iterations as u128;
    let average_time_string: String = format_time(average_time);
    println!("Average Time: {}", average_time_string);
    println!();
}
