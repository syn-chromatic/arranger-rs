mod python;
mod search;
mod shell;

use std::io::Error;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::error::Error as ClapError;
use clap::{Args, Parser, Subcommand, ValueEnum};
use dirs;
use regex::Regex;

use crate::python::packages::VirtualEnv;
use crate::python::python::PythonEnvironment;
use crate::python::version::PythonVersion;
use crate::search::file::FileSearch;

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
    #[command(about = "Create Virtual Environment", name = "venv")]
    VirtualEnv,
    #[command(about = "Fix Virtual Environments", name = "fix-venv")]
    FixVirtualEnvironments,
}

fn find_virtual_env_config(current_dir: &PathBuf) -> Vec<PathBuf> {
    let mut file_search: FileSearch = FileSearch::new();

    let root: &PathBuf = current_dir;
    let exclusive_filenames: Vec<&str> = vec!["pyvenv.cfg"];
    let exclusive_exts: Vec<&str> = vec![];
    let exclude_dirs: Vec<&str> = vec![];

    file_search.set_root(root);
    file_search.set_exclusive_filenames(exclusive_filenames);
    file_search.set_exclusive_extensions(exclusive_exts);
    file_search.set_exclude_directories(exclude_dirs);

    let files: Vec<PathBuf> = file_search.search_files();

    for file in &files {
        println!("[{:?}]", file);
    }
    files
}

fn find_virtual_env_activations(scripts_dir: &PathBuf) -> Vec<PathBuf> {
    let mut file_search: FileSearch = FileSearch::new();

    let root: &PathBuf = scripts_dir;
    let exclusive_filenames: Vec<&str> =
        vec!["activate", "activate.bat", "activate.fish", "activate.nu"];
    let exclusive_exts: Vec<&str> = vec![];
    let exclude_dirs: Vec<&str> = vec![];

    file_search.set_root(root);
    file_search.set_exclusive_filenames(exclusive_filenames);
    file_search.set_exclusive_extensions(exclusive_exts);
    file_search.set_exclude_directories(exclude_dirs);

    let files: Vec<PathBuf> = file_search.search_files();

    for file in &files {
        println!("[{:?}]", file);
    }
    files
}

fn get_virtual_env_path_string(path: &PathBuf) -> String {
    let dir_str = path.to_string_lossy();

    let dir_str = dir_str.strip_prefix(r"\\?\").unwrap_or(&dir_str);
    dir_str.to_string()
}

fn get_virtual_envs() {
    let current_dir: Result<PathBuf, Error> = std::env::current_dir();
    if let Ok(current_dir) = current_dir {
        let cfg_files = find_virtual_env_config(&current_dir);
        for cfg_file in &cfg_files {
            let env_dir = cfg_file.parent();
            if let Some(env_dir) = env_dir {
                let env_dir = env_dir.to_path_buf();
                let scripts_dir = env_dir.join("Scripts");
                let deactivate_path = scripts_dir.join("deactivate.nu");

                if scripts_dir.exists() {
                    let activations = find_virtual_env_activations(&scripts_dir);

                    let env_str = get_virtual_env_path_string(&env_dir);
                    let deactivate_str = get_virtual_env_path_string(&deactivate_path);

                    for activation in &activations {
                        let contents = std::fs::read_to_string(activation)
                            .expect("Something went wrong reading the file");
                        let mut output = String::new();

                        let re = Regex::new(
                            r#"(.*)(VIRTUAL_ENV='|set "VIRTUAL_ENV=|set -gx VIRTUAL_ENV '|let virtual_env = '|alias deactivate = source ')(.*)('|")"#,
                        ).unwrap();

                        for line in contents.lines() {
                            if let Some(cap) = re.captures(line) {
                                let new_line = if cap[2].trim().starts_with("alias deactivate") {
                                    format!("{}{}{}{}", &cap[1], &cap[2], deactivate_str, &cap[4])
                                } else {
                                    format!("{}{}{}{}", &cap[1], &cap[2], env_str, &cap[4])
                                };
                                output.push_str(&new_line);
                                output.push('\n');
                            } else {
                                output.push_str(line);
                                output.push('\n');
                            }
                        }

                        let mut file = std::fs::OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .open(activation)
                            .unwrap();
                        // std::fs::write(path, contents)
                        // file

                        let result = file.write_all(output.as_bytes());
                        println!("result: {:?}", result);
                    }
                }
            }
        }
    }
}

fn main() {
    let opt: Result<Cli, ClapError> = Cli::try_parse();
    match opt {
        Ok(opt) => match opt.command {
            Commands::Python(python_opt) => {
                println!("python: {:?}", python_opt);
                match python_opt.subcommands {
                    PythonSubCommands::VirtualEnv => {
                        let (major, minor) = python_opt.version.get_version();
                        create_virtual_env(major, minor);
                    }
                    PythonSubCommands::FixVirtualEnvironments => {
                        get_virtual_envs();
                    }
                }
            }
        },
        Err(opt) => {
            println!("{}", opt.to_string());
        }
    }
}
