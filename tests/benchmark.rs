use arranger::search::file::FileSearch;

#[test]
pub fn search_benchmark() {
    let mut file_search: FileSearch = FileSearch::new();
    file_search.set_root("./tests/benchmark_files");

    for _ in 0..10 {
        let _ = file_search.search_files_benchmark();
    }
}
