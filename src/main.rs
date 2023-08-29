pub mod ansi_support;
pub mod commands;
pub mod general;
pub mod parsers;
pub mod python;
pub mod rust;
pub mod search;
pub mod structures;
pub mod utils;

use std::io;

use clap::error::Error as ClapError;
use clap::Parser;

use crate::ansi_support::AnsiSupport;

use crate::commands::configuration::Cli;
use crate::commands::configuration::Commands;
use crate::commands::configuration::PythonSubCommands;
use crate::commands::configuration::RustSubCommands;

use crate::commands::python::PythonCreateEnvCommand;
use crate::commands::python::PythonDLCommand;
use crate::commands::python::PythonExecuteCommand;
use crate::commands::python::PythonFixEnvCommand;
use crate::commands::python::PythonPackagesCommand;
use crate::commands::rust::RustVSCodeTaskCommand;
use crate::commands::search::SearchCommand;

use crate::utils::OptionsPrinter;

#[tokio::main]
async fn main() {
    let ansi_support: Result<(), io::Error> = AnsiSupport::enable();

    if let Err(_) = ansi_support {
        println!("WARNING: ANSI Color Code support is not supported on this platform.");
    }

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
                PythonSubCommands::VirtualEnvExecute(option) => {
                    let command: PythonExecuteCommand = PythonExecuteCommand::new(option);
                    command.execute_command();
                }
            },
            Commands::Rust(rust_opt) => match rust_opt.subcommands {
                RustSubCommands::RustVSCodeTasks(option) => {
                    let command: RustVSCodeTaskCommand = RustVSCodeTaskCommand::new(option);
                    command.execute_command();
                }
            },
            Commands::Search(option) => {
                let command: SearchCommand = SearchCommand::new(option);
                command.execute_command();
            }
        },
        Err(opt) => {
            let opt_string: String = opt.to_string();
            let options_printer: OptionsPrinter = OptionsPrinter::new();
            options_printer.print(&opt_string);
        }
    }
}
