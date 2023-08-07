mod general;
mod parsers;
mod python;
mod search;
mod utils;

use clap::error::Error as ClapError;
use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::general::version::SemanticVersion;
use crate::utils::{
    create_virtual_env, download_python, fix_virtual_environments, list_environment_packages,
};

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
    #[command(about = "List Environment Packages", name = "list-packages")]
    ListEnvPackages(ListEnvPackagesCommand),
}

#[derive(Debug, Parser)]
pub struct DownloadPythonCommand {
    /// Select Python version
    #[arg(short = 'V', long = "version")]
    version: SemanticVersion,

    /// List Version Files
    #[arg(short = 'L', long = "list", default_value = "false")]
    list_structure: bool,

    /// Specify Architecture: [amd64, arm64, n/a]
    #[arg(short = 'A', long = "arch", default_value = "amd64")]
    architecture: String,

    /// Specify Platform: [windows, macos, linux]
    #[arg(short = 'P', long = "platform", default_value = "windows")]
    platform: String,

    /// Specify Package Type: [standard, webinstall, embed]
    #[arg(short = 'T', long = "package-type", default_value = "standard")]
    package_type: String,
}

#[derive(Debug, Parser)]
pub struct VirtualEnvCommand {
    /// Select Python version
    #[arg(short = 'V', long = "version")]
    version: SemanticVersion,
}

#[derive(Debug, Parser)]
pub struct FixVirtualEnvCommand {
    /// Perform a deep search
    #[arg(short = 'D', long = "deep-search")]
    deep_search: bool,
}

#[derive(Debug, Parser)]
pub struct ListEnvPackagesCommand {
    /// Perform a deep search
    #[arg(short = 'D', long = "deep-search")]
    deep_search: bool,

    /// Create Package Requirements File For Environments
    #[arg(short = 'S', long = "save-packages", default_value = "false")]
    save_packages: bool,
}

fn main() {
    let opt: Result<Cli, ClapError> = Cli::try_parse();
    match opt {
        Ok(opt) => match opt.command {
            Commands::Python(python_opt) => match python_opt.subcommands {
                PythonSubCommands::DownloadPython(download_command) => {
                    let version: SemanticVersion = download_command.version;
                    let list_structure: bool = download_command.list_structure;
                    let arch: String = download_command.architecture;
                    let platform: String = download_command.platform;
                    let package_type: String = download_command.package_type;
                    download_python(version, list_structure, &arch, &platform, &package_type);
                }
                PythonSubCommands::VirtualEnv(venv_command) => {
                    let (major, minor): (usize, usize) = venv_command.version.get_2p_version();
                    create_virtual_env(major, minor);
                }
                PythonSubCommands::FixVirtualEnvironments(fix_venv_command) => {
                    fix_virtual_environments(fix_venv_command);
                }
                PythonSubCommands::ListEnvPackages(list_packages_command) => {
                    list_environment_packages(list_packages_command);
                }
            },
        },
        Err(opt) => {
            println!("Error: {}", opt.to_string());
        }
    }
}
