mod general;
mod parsers;
mod python;
mod search;
mod utils;

use clap::error::Error as ClapError;
use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::python::version::PythonVersion;
use crate::utils::{create_virtual_env, fix_virtual_environments};

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
    #[command(about = "Create Virtual Environment", name = "venv")]
    VirtualEnv(VirtualEnvCommand),
    #[command(about = "Fix Virtual Environments", name = "fix-venv")]
    FixVirtualEnvironments,
}

#[derive(Debug, Parser)]
struct VirtualEnvCommand {
    /// Select Python version
    #[arg(short = 'V', long = "version")]
    version: PythonVersion,
}

fn main() {
    let opt: Result<Cli, ClapError> = Cli::try_parse();
    match opt {
        Ok(opt) => match opt.command {
            Commands::Python(python_opt) => match python_opt.subcommands {
                PythonSubCommands::VirtualEnv(venv_command) => {
                    let (major, minor) = venv_command.version.get_version();
                    create_virtual_env(major, minor);
                }
                PythonSubCommands::FixVirtualEnvironments => {
                    fix_virtual_environments();
                }
            },
        },
        Err(opt) => {
            println!("Error: {}", opt.to_string());
        }
    }
}
