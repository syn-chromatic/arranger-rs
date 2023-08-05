use std::collections::HashSet;
use std::io::{Error, Write};
use std::path::PathBuf;

use dirs;
use tokio::runtime::Runtime;

use crate::general::http::HTTP;
use crate::general::path::WPath;
use crate::general::version::SemanticVersion;
use crate::parsers::cfg_parser::CFGLine;
use crate::parsers::cfg_parser::CFGParser;
use crate::python::python::PythonEnvironment;
use crate::python::python_ftp::PythonFTP;
use crate::python::version::PythonVersion;
use crate::python::virtualenv::VirtualEnv;
use crate::python::virtualenv::VirtualEnvCFG;
use crate::search::file::FileSearch;
use crate::FixVirtualEnvCommand;

pub fn create_virtual_env(major: usize, minor: usize) {
    let data_dir: Option<PathBuf> = dirs::data_local_dir();

    if let Some(data_dir) = data_dir {
        let base_path_buf: PathBuf = data_dir.join("Programs/Python");
        let base_path: WPath = WPath::from_path_buf(&base_path_buf);

        let python_version: PythonVersion = PythonVersion::new(major, minor, 0);

        let environment: Option<PythonEnvironment> =
            PythonEnvironment::new(base_path, python_version);

        if let Some(environment) = environment {
            let virtual_env: VirtualEnv = VirtualEnv::new(&environment);
            virtual_env.create_virtual_env();
        }
    }
}

pub fn create_virtual_env_in_path(path: WPath, major: usize, minor: usize) {
    let data_dir: Option<PathBuf> = dirs::data_local_dir();

    if let Some(data_dir) = data_dir {
        let base_path_buf: PathBuf = data_dir.join("Programs/Python");
        let base_path: WPath = WPath::from_path_buf(&base_path_buf);

        let python_version: PythonVersion = PythonVersion::new(major, minor, 0);

        let environment: Option<PythonEnvironment> =
            PythonEnvironment::new(base_path, python_version);

        if let Some(environment) = environment {
            let virtual_env: VirtualEnv = VirtualEnv::new(&environment);
            virtual_env.create_virtual_env_in_path(path);
        }
    }
}

pub fn fix_virtual_environments(fix_venv_command: FixVirtualEnvCommand) {
    let deep_search: bool = fix_venv_command.deep_search;
    println!("Deep search: {}", deep_search);
    let venv_cfgs: Vec<VirtualEnvCFG> = get_virtual_env_cfgs(deep_search);
    for venv_cfg in venv_cfgs {
        let version: crate::general::version::SemanticVersion = venv_cfg.version_info;
        let cfg_file: WPath = venv_cfg.cfg_file;
        let (major, minor): (usize, usize) = version.get_2p_version();
        create_virtual_env_in_path(cfg_file, major, minor);
    }
}

pub fn download_python(version: PythonVersion) {
    let version_tuple: (usize, usize, usize) = version.get_3p_version();
    let python_ftp: Option<PythonFTP> = PythonFTP::from_tuple(version_tuple);

    if let Some(python_ftp) = python_ftp {
        let url: &str = python_ftp.get_url();
        let rt: Runtime = Runtime::new().unwrap();
        let http: HTTP = HTTP::new();
        let result = rt.block_on(http.download_file(url));
        println!("Result: {:?}", result.unwrap());
    }
}

fn find_virtual_env_config(current_dir: &PathBuf, deep_search: bool) -> HashSet<PathBuf> {
    let mut file_search: FileSearch = FileSearch::new();

    let root: &PathBuf = current_dir;
    let exclusive_filenames: Vec<&str> = vec!["pyvenv.cfg"];
    let exclusive_exts: Vec<&str> = vec![];
    let exclude_dirs: Vec<&str> = vec![];
    let quit_directory_on_match: bool = !deep_search;

    file_search.set_root(root);
    file_search.set_exclusive_filenames(exclusive_filenames);
    file_search.set_exclusive_extensions(exclusive_exts);
    file_search.set_exclude_directories(exclude_dirs);
    file_search.set_quit_directory_on_match(quit_directory_on_match);

    let files: HashSet<PathBuf> = file_search.search_files();
    files
}

fn find_virtual_env_activations(scripts_dir: &PathBuf) -> HashSet<PathBuf> {
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

    let files: HashSet<PathBuf> = file_search.search_files();

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

fn confirm_and_continue() -> bool {
    let mut input = String::new();
    print!("\nDo you want to continue? [y/N]: ");
    std::io::stdout().flush().unwrap();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim() == "y" || input.trim() == "Y" {
                println!("Continuing...");
                return true;
            } else {
                println!("Not continuing...");
                return false;
            }
        }
        Err(e) => {
            println!("Failed to read line: {}", e);
            return false;
        }
    }
}

fn confirm_discovered_environments(cfg_files: &HashSet<PathBuf>) -> bool {
    println!("\nFound Environments:");
    for cfg_file in cfg_files {
        let mut environment_directory: WPath = cfg_file.into();
        environment_directory.to_directory();

        let directory_string = environment_directory.get_canonical_string();
        if let Some(directory_string) = directory_string {
            println!("Path: [{}]", directory_string);
        }
    }
    confirm_and_continue()
}

fn get_virtual_env_cfgs(deep_search: bool) -> Vec<VirtualEnvCFG> {
    let current_dir: Result<PathBuf, Error> = std::env::current_dir();
    let mut venv_cfgs: Vec<VirtualEnvCFG> = Vec::new();
    if let Ok(current_dir) = current_dir {
        let cfg_files: HashSet<PathBuf> = find_virtual_env_config(&current_dir, deep_search);
        let response: bool = confirm_discovered_environments(&cfg_files);
        if !response {
            return venv_cfgs;
        }

        for cfg_file in cfg_files {
            let cfg_parser: CFGParser = CFGParser::new();
            let result: Result<Vec<CFGLine>, Error> = cfg_parser.from_file(&cfg_file);

            if let Ok(result) = result {
                let cfg_path: WPath = WPath::from_path_buf(&cfg_file);
                let venv_cfg: Option<VirtualEnvCFG> = VirtualEnvCFG::new(cfg_path, &result);
                if let Some(venv_cfg) = venv_cfg {
                    venv_cfgs.push(venv_cfg);
                }
            }
        }
    }
    venv_cfgs
}
