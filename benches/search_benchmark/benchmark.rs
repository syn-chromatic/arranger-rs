use arranger::search::file::FileSearch;
use arranger::search::formatters::format_time;
use arranger::search::progress::SearchProgress;

#[test]
pub fn search_benchmark() {
    search_nonexistent_benchmark(5);
    search_standard_benchmark(5);
    search_regex_benchmark(5);
}

pub fn search_nonexistent_benchmark(iterations: usize) {
    println!("[Benchmarking Search 1]");
    let mut file_search: FileSearch = FileSearch::new();
    file_search.set_root("./benches/search_benchmark/benchmark_files");
    file_search.set_exclusive_filename("file_500001.txt");

    let mut total_time: u128 = 0;
    for _ in 0..iterations {
        let progress: SearchProgress = file_search.search_files_benchmark();
        let elapsed_time: u128 = progress.get_elapsed_time_ns();
        total_time += elapsed_time;
    }

    let average_time: u128 = total_time / iterations as u128;
    let average_time_string: String = format_time(average_time);
    println!("Average Time: {}", average_time_string);
    println!();
}

pub fn search_standard_benchmark(iterations: usize) {
    println!("[Benchmarking Search 2]");
    let mut file_search: FileSearch = FileSearch::new();
    file_search.set_root("./benches/search_benchmark/benchmark_files");
    file_search.set_exclusive_filename("file_");
    file_search.set_exclusive_extensions(["txt"]);

    let mut total_time: u128 = 0;
    for _ in 0..iterations {
        let progress: SearchProgress = file_search.search_files_benchmark();
        let elapsed_time: u128 = progress.get_elapsed_time_ns();
        total_time += elapsed_time;
    }

    let average_time: u128 = total_time / iterations as u128;
    let average_time_string: String = format_time(average_time);
    println!("Average Time: {}", average_time_string);
    println!();
}

pub fn search_regex_benchmark(iterations: usize) {
    println!("[Benchmarking Search Regex]");
    let mut file_search: FileSearch = FileSearch::new();
    file_search.set_root("./benches/search_benchmark/benchmark_files");
    let _ = file_search.set_exclusive_filename_regex(".*");

    let mut total_time: u128 = 0;
    for _ in 0..iterations {
        let progress: SearchProgress = file_search.search_files_benchmark();
        let elapsed_time: u128 = progress.get_elapsed_time_ns();
        total_time += elapsed_time;
    }

    let average_time: u128 = total_time / iterations as u128;
    let average_time_string: String = format_time(average_time);
    println!("Average Time: {}", average_time_string);
    println!();
}
