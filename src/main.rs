pub mod commands;
pub mod languages;
pub mod misc;
pub mod parsers;
pub mod search;
pub mod terminal;
pub mod utils;

use std::io;

use clap::error::Error as ClapError;
use clap::Parser;
use tokio::runtime::Runtime;

use crate::misc::ansi_support::AnsiSupport;

use crate::commands::config::Cli;
use crate::commands::config::Commands;
use crate::commands::config::PythonSubCommands;
use crate::commands::config::RustSubCommands;

use crate::commands::python::PythonCreateEnvCommand;
use crate::commands::python::PythonDLCommand;
use crate::commands::python::PythonExecuteCommand;
use crate::commands::python::PythonFixEnvCommand;
use crate::commands::python::PythonPackagesCommand;
use crate::commands::rust::RustVSCodeTaskCommand;
use crate::commands::search::SearchCommand;

use crate::utils::OptionsPrinter;

use crate::misc::interrupt_handler::InterruptHandler;
use crate::terminal::{ANSICode, CursorOnANSI, LineWrapOnANSI, ResetANSI};

fn main() {
    let handler: InterruptHandler<_, _> = InterruptHandler::new(main_runtime, interrupt);
    handler.run();
}

fn main_runtime() {
    let runtime: Runtime = Runtime::new().unwrap();
    runtime.block_on(main_program());
}

async fn main_program() {
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

fn interrupt() {
    let reset_ansi: String = ResetANSI.value();
    let cursor_ansi: String = CursorOnANSI.value();
    let line_wrap_ansi: String = LineWrapOnANSI.value();
    print!("{}{}{}", reset_ansi, cursor_ansi, line_wrap_ansi);
}
