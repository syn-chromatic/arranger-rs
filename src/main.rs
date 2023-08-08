mod commands;
mod general;
mod parsers;
mod python;
mod rust;
mod search;
mod utils;

use clap::error::Error as ClapError;
use clap::{Parser, Subcommand};

use crate::commands::PythonCreateEnvCommand;
use crate::commands::PythonDLCommand;
use crate::commands::PythonFixEnvCommand;
use crate::commands::PythonPackagesCommand;
use crate::general::version::SemanticVersion;
use crate::rust::tasks::generate_rust_run_task;

#[derive(Debug, Parser)]
#[command(name = "Arranger")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Python Tools")]
    Python(PythonCommand),
    #[command(about = "Rust Tools")]
    Rust(RustCommand),
}

#[derive(Debug, Parser)]
struct PythonCommand {
    #[command(subcommand)]
    subcommands: PythonSubCommands,
}

#[derive(Debug, Subcommand)]
enum PythonSubCommands {
    #[command(about = "Create Virtual Environment", name = "venv")]
    VirtualEnv(VirtualEnvOption),
    #[command(about = "Fix Virtual Environments", name = "fix-venv")]
    FixVirtualEnvironments(FixVirtualEnvOption),
    #[command(about = "Environment Packages", name = "packages")]
    EnvPackages(PackagesOption),
    #[command(about = "Download Python Version", name = "download")]
    PythonDownload(PythonDownloadOption),
}

#[derive(Debug, Parser)]
pub struct PythonDownloadOption {
    /// Select Python version
    #[arg(short = 'V', long = "version")]
    version: SemanticVersion,

    /// Retrieve most recent patch
    #[arg(short = 'R', long = "recent-patch", default_value = "false")]
    recent_patch: bool,

    /// List Python version files [No Download]
    #[arg(short = 'L', long = "list", default_value = "false")]
    list_structure: bool,

    /// Specify Architecture: [amd64, arm64, n/a]
    #[arg(short = 'A', long = "arch", default_value = "amd64")]
    architecture: String,

    /// Specify Platform: [windows, macos, any]
    #[arg(short = 'P', long = "platform", default_value = "windows")]
    platform: String,

    /// Specify Package Type: [standard, webinstall, embed, source]
    #[arg(short = 'T', long = "package-type", default_value = "standard")]
    package_type: String,
}

#[derive(Debug, Parser)]
pub struct VirtualEnvOption {
    /// Select Python version
    #[arg(short = 'V', long = "version")]
    version: SemanticVersion,
}

#[derive(Debug, Parser)]
pub struct FixVirtualEnvOption {
    /// Perform a deep search
    #[arg(short = 'D', long = "deep-search")]
    deep_search: bool,
}

#[derive(Debug, Parser)]
pub struct PackagesOption {
    /// Perform a deep search
    #[arg(short = 'D', long = "deep-search")]
    deep_search: bool,

    /// Create package list for each environment
    #[arg(short = 'S', long = "save-packages", default_value = "false")]
    save_packages: bool,
}

#[derive(Debug, Parser)]
struct RustCommand {
    #[command(subcommand)]
    subcommands: RustSubCommands,
}

#[derive(Debug, Subcommand)]
enum RustSubCommands {
    #[command(about = "Generate VSCode Tasks", name = "vscode-tasks")]
    GenerateTasks(GenerateTasksOption),
}

#[derive(Debug, Parser)]
pub struct GenerateTasksOption {
    /// Generate Run Task
    #[arg(short = 'R', long = "run-task", default_value = "false")]
    run_task: bool,
}

#[tokio::main]
async fn main() {
    let opt: Result<Cli, ClapError> = Cli::try_parse();
    match opt {
        Ok(opt) => match opt.command {
            Commands::Python(python_opt) => match python_opt.subcommands {
                PythonSubCommands::PythonDownload(option) => {
                    let mut command: PythonDLCommand = PythonDLCommand::new(option);
                    command.execute_command().await;
                }
                PythonSubCommands::VirtualEnv(option) => {
                    let command: PythonCreateEnvCommand = PythonCreateEnvCommand::new(option);
                    command.execute_command();
                }
                PythonSubCommands::FixVirtualEnvironments(option) => {
                    let command: PythonFixEnvCommand = PythonFixEnvCommand::new(option);
                    command.execute_command();
                }
                PythonSubCommands::EnvPackages(option) => {
                    let command: PythonPackagesCommand = PythonPackagesCommand::new(option);
                    command.execute_command();
                }
            },
            Commands::Rust(rust_opt) => match rust_opt.subcommands {
                RustSubCommands::GenerateTasks(option) => {
                    if option.run_task {
                        generate_rust_run_task();
                    }
                }
            },
        },
        Err(opt) => {
            println!("Error: {}", opt.to_string());
        }
    }
}
