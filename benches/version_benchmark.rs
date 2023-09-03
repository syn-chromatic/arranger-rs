use arranger::general::version::SemanticVersion;
use regex::Regex;
use std::time::Instant;

#[test]
fn version_benchmark() {
    let version_string: &str = "3.12.0b4";
    let iterations: usize = 1_000_000;
    algorithm_benchmark(version_string, iterations);
    regex_benchmark(version_string, iterations);
}

fn algorithm_benchmark(version_string: &str, iterations: usize) {
    println!("[Benchmarking Algorithm]");
    let now: Instant = Instant::now();
    for _ in 0..iterations {
        let _: Option<SemanticVersion> = SemanticVersion::from_string(version_string);
    }
    println!("Elapsed Time: {} ms\n", now.elapsed().as_millis());
}

fn regex_benchmark(version_string: &str, iterations: usize) {
    println!("[Benchmarking Regex]");
    let re: Regex = Regex::new(r"^(?P<Major>\d+)\.(?P<Minor>\d+)\.(?P<Patch>\d+)(?P<PreleaseType>[a-zA-Z])(?P<PreleaseVersion>\d+)$").unwrap();

    let now: Instant = Instant::now();
    for _ in 0..iterations {
        let _: Option<(u32, u32, u32, String, u32)> = regex_version(version_string, &re);
    }
    println!("Elapsed Time: {} ms\n", now.elapsed().as_millis());
}

fn regex_version(version_string: &str, re: &Regex) -> Option<(u32, u32, u32, String, u32)> {
    let mut version: Option<(u32, u32, u32, String, u32)> = None;

    if let Some(captures) = re.captures(version_string) {
        let major: u32 = captures["Major"].parse::<u32>().unwrap();
        let minor: u32 = captures["Minor"].parse::<u32>().unwrap();
        let patch: u32 = captures["Patch"].parse::<u32>().unwrap();
        let prelease_type: String = captures["PreleaseType"].to_string();
        let prelease_version: u32 = captures["PreleaseVersion"].parse::<u32>().unwrap();

        version = Some((major, minor, patch, prelease_type, prelease_version));
    }
    version
}
