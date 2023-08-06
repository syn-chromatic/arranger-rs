mod general;
mod parsers;
mod python;
mod search;
mod utils;

use clap::error::Error as ClapError;
use clap::{Args, Parser, Subcommand, ValueEnum};

use reqwest;
use scraper::{Html, Selector};
use std::collections::HashSet;
use tokio::runtime::Runtime;

use crate::general::path::WPath;
use crate::general::version::SemanticVersion;
use crate::python::python_ftp::{FileStructure, LinkType};
use crate::python::version::PythonVersion;
use crate::utils::{create_virtual_env, download_python, fix_virtual_environments};

#[derive(Debug, Parser)]
#[command(name = "Arranger")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Python Arranger")]
    Python(PythonCommand),
}

#[derive(Debug, Parser)]
struct PythonCommand {
    #[command(subcommand)]
    subcommands: PythonSubCommands,
}

#[derive(Debug, Subcommand)]
enum PythonSubCommands {
    #[command(about = "Download Python Version", name = "download")]
    DownloadPython(DownloadPythonCommand),
    #[command(about = "Create Virtual Environment", name = "venv")]
    VirtualEnv(VirtualEnvCommand),
    #[command(about = "Fix Virtual Environments", name = "fix-venv")]
    FixVirtualEnvironments(FixVirtualEnvCommand),
}

#[derive(Debug, Parser)]
pub struct DownloadPythonCommand {
    /// Select Python version
    #[arg(short = 'V', long = "version")]
    version: PythonVersion,
}

#[derive(Debug, Parser)]
pub struct VirtualEnvCommand {
    /// Select Python version
    #[arg(short = 'V', long = "version")]
    version: PythonVersion,
}

#[derive(Debug, Parser)]
pub struct FixVirtualEnvCommand {
    /// Perform a deep search
    #[arg(short = 'D', long = "deep-search")]
    deep_search: bool,
}

fn main() {
    let opt: Result<Cli, ClapError> = Cli::try_parse();
    match opt {
        Ok(opt) => match opt.command {
            Commands::Python(python_opt) => match python_opt.subcommands {
                PythonSubCommands::DownloadPython(download_command) => {
                    let version = download_command.version;
                    download_python(version);
                }
                PythonSubCommands::VirtualEnv(venv_command) => {
                    let (major, minor) = venv_command.version.get_version();
                    create_virtual_env(major, minor);
                }
                PythonSubCommands::FixVirtualEnvironments(fix_venv_command) => {
                    fix_virtual_environments(fix_venv_command);
                }
            },
        },
        Err(opt) => {
            println!("Error: {}", opt.to_string());
        }
    }
}
