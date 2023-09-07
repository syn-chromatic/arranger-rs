use std::str::FromStr;

use clap::{Parser, Subcommand};

use crate::general::version::SemanticVersion;

#[derive(Debug, Parser)]
#[command(name = "Arranger")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Python Tools")]
    Python(PythonCommand),
    #[command(about = "Rust Tools")]
    Rust(RustCommand),
    #[command(about = "Search Tool")]
    Search(SearchOption),
}

#[derive(Debug, Parser)]
pub struct PythonCommand {
    #[command(subcommand)]
    pub subcommands: PythonSubCommands,
}

#[derive(Debug, Subcommand)]
pub enum PythonSubCommands {
    #[command(about = "Create Virtual Environment", name = "venv")]
    VirtualEnv(VirtualEnvOption),
    #[command(about = "Fix Virtual Environments", name = "fix-venv")]
    FixVirtualEnvironments(FixVirtualEnvOption),
    #[command(about = "Execute Command To Virtual Environments", name = "execute")]
    VirtualEnvExecute(VirtualEnvExecuteOption),
    #[command(about = "Virtual Environment Packages", name = "packages")]
    EnvPackages(PackagesOption),
    #[command(about = "Python Download", name = "download")]
    PythonDownload(PythonDownloadOption),
}

#[derive(Debug, Parser)]
pub struct PythonDownloadOption {
    /// Select Python version
    #[arg(short = 'V', long = "version")]
    pub version: SemanticVersion,

    /// Retrieve most recent patch
    #[arg(short = 'R', long = "recent-patch", default_value = "false")]
    pub recent_patch: bool,

    /// List Python version files [No Download]
    #[arg(short = 'L', long = "list", default_value = "false")]
    pub list_structure: bool,

    /// Specify Architecture: [amd64, arm64, n/a]
    #[arg(short = 'A', long = "arch", default_value = "amd64")]
    pub architecture: String,

    /// Specify Platform: [windows, macos, any]
    #[arg(short = 'P', long = "platform", default_value = "windows")]
    pub platform: String,

    /// Specify Package Type: [standard, webinstall, embed, source]
    #[arg(short = 'T', long = "package-type", default_value = "standard")]
    pub package_type: String,
}

#[derive(Debug, Parser)]
pub struct VirtualEnvOption {
    /// Select Python version
    #[arg(short = 'V', long = "version")]
    pub version: SemanticVersion,
}

#[derive(Debug, Parser)]
pub struct FixVirtualEnvOption {
    /// Perform a deep search
    #[arg(short = 'D', long = "deep-search")]
    pub deep_search: bool,
}

#[derive(Debug, Parser)]
pub struct VirtualEnvExecuteOption {
    /// Perform a deep search
    #[arg(short = 'D', long = "deep-search")]
    pub deep_search: bool,

    /// Pass command to each virtual environment
    #[arg(short = 'C', long = "command", raw(true))]
    pub command: String,
}

/// Virtual Environment Packages
///
/// [$ENV placeholder refers to the root path of a Python Virtual Environment]
#[derive(Debug, Parser)]
pub struct PackagesOption {
    /// Perform a deep search
    #[arg(short = 'D', long = "deep-search")]
    pub deep_search: bool,

    /// Save package list for each environment [$ENV/packages.txt]
    #[arg(short = 'S', long = "save", default_value = "false")]
    pub save: bool,

    /// Distill packages by mutual dependencies [With -S: $ENV/distilled_packages.txt]
    #[arg(short = 'X', long = "distill", default_value = "false")]
    pub distill: bool,
}

#[derive(Debug, Parser)]
pub struct RustCommand {
    #[command(subcommand)]
    pub subcommands: RustSubCommands,
}

#[derive(Debug, Subcommand)]
pub enum RustSubCommands {
    #[command(about = "Generate VSCode Tasks Command", name = "vscode-tasks")]
    RustVSCodeTasks(RustVSCodeTasksOption),
}

#[derive(Debug, Parser)]
pub struct RustVSCodeTasksOption {
    /// Generate Run Task
    #[arg(short = 'R', long = "run-task", default_value = "false")]
    pub run_task: bool,
}

#[derive(Debug, Parser)]
pub struct SearchOption {
    /// Specify Filename [Matches by start of name when used without regex]
    #[arg(short = 'F', long = "filename")]
    pub filename: Option<String>,

    /// Specify Extensions [Can be used multiple times to add items]
    #[arg(short = 'E', long = "extensions", default_value = None)]
    pub extensions: Vec<String>,

    /// Specify Directory To Exclude [Can be used multiple times to add items]
    #[arg(short = 'X', long = "exclude-dir", default_value = None)]
    pub excluded_dirs: Vec<String>,

    /// Specify Sorting Of Results [size_asc, size_desc, created_asc, created_desc, modified_asc, modified_desc]
    #[arg(short = 'S', long = "sort")]
    pub sort: Option<SearchSort>,

    /// Specify Limit For Results
    #[arg(short = 'L', long = "limit", value_parser = parse_search_option_limit)]
    pub limit: Option<usize>,

    /// Enable the regex engine for pattern matching
    #[arg(short = 'R', long = "regex", default_value = "false")]
    pub regex: bool,

    /// Specify the amount of threads to use
    #[arg(short = 'T', long = "threads", default_value = "4")]
    pub threads: usize,
}

fn parse_search_option_limit(value: &str) -> Result<usize, &'static str> {
    match value.parse::<usize>() {
        Ok(0) => Err("value must be greater than 0"),
        Ok(val) => Ok(val),
        Err(_) => Err("invalid digit found in string"),
    }
}

#[derive(Debug, Clone)]
pub enum SearchSort {
    SizeAsc,
    SizeDesc,
    CreatedAsc,
    CreatedDesc,
    ModifiedAsc,
    ModifiedDesc,
}

impl FromStr for SearchSort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "size_asc" => Ok(Self::SizeAsc),
            "size_desc" => Ok(Self::SizeDesc),
            "created_asc" => Ok(Self::CreatedAsc),
            "created_desc" => Ok(Self::CreatedDesc),
            "modified_asc" => Ok(Self::ModifiedAsc),
            "modified_desc" => Ok(Self::ModifiedDesc),
            _ => Err(format!("Invalid Sorting Option")),
        }
    }
}
