mod python;
mod shell;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use dirs;

use crate::python::packages::VirtualEnv;
use crate::python::python::PythonEnvironment;
use crate::python::version::PythonVersion;

fn create_virtual_env(major: usize, minor: usize) {
    let data_dir: Option<PathBuf> = dirs::data_local_dir();

    if let Some(data_dir) = data_dir {
        let base_path: PathBuf = data_dir.join("Programs/Python");
        let python_version: PythonVersion = PythonVersion::new(major, minor);

        let environment: Option<PythonEnvironment> =
            PythonEnvironment::new(base_path, python_version);

        if let Some(environment) = environment {
            let virtual_env: VirtualEnv = VirtualEnv::new(&environment);
            virtual_env.create_virtual_env();
        }
    }
}

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
    /// Select Python version
    #[arg(short = 'V', long = "version")]
    version: PythonVersion,

    #[command(subcommand)]
    subcommands: PythonSubCommands,
}

#[derive(Debug, Subcommand)]
enum PythonSubCommands {
    #[command(about = "Create Virtual Environment")]
    VirtualEnv,
}

fn main() {
    let opt = Cli::try_parse();

    match opt {
        Ok(opt) => match opt.command {
            Commands::Python(python_opt) => {
                println!("python: {:?}", python_opt);
            }
        },
        Err(opt) => {
            println!("{}", opt.to_string());
        }
    }
}
