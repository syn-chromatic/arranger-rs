use std::io::Error;
use std::path::PathBuf;

use dirs;

use crate::parsers::cfg_parser::CFGLine;
use crate::parsers::cfg_parser::CFGParser;
use crate::python::python::PythonEnvironment;
use crate::python::version::PythonVersion;
use crate::python::virtualenv::VirtualEnv;
use crate::python::virtualenv::VirtualEnvCFG;
use crate::search::file::FileSearch;

pub fn create_virtual_env(major: usize, minor: usize) {
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

pub fn create_virtual_env_in_path(path: PathBuf, major: usize, minor: usize) {
    let data_dir: Option<PathBuf> = dirs::data_local_dir();

    if let Some(data_dir) = data_dir {
        let base_path: PathBuf = data_dir.join("Programs/Python");
        let python_version: PythonVersion = PythonVersion::new(major, minor);

        let environment: Option<PythonEnvironment> =
            PythonEnvironment::new(base_path, python_version);

        if let Some(environment) = environment {
            let virtual_env: VirtualEnv = VirtualEnv::new(&environment);
            virtual_env.create_virtual_env_in_path(path);
        }
    }
}

pub fn fix_virtual_environments() {
    let venv_cfgs: Vec<VirtualEnvCFG> = get_virtual_env_cfgs();
    for venv_cfg in venv_cfgs {
        let version = venv_cfg.version_info;
        let cfg_file = venv_cfg.cfg_file;
        let (major, minor) = version.get_2p_version();
        create_virtual_env_in_path(cfg_file, major, minor);
    }
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

fn get_virtual_env_cfgs() -> Vec<VirtualEnvCFG> {
    let current_dir: Result<PathBuf, Error> = std::env::current_dir();
    let mut venv_cfgs: Vec<VirtualEnvCFG> = Vec::new();
    if let Ok(current_dir) = current_dir {
        let cfg_files: Vec<PathBuf> = find_virtual_env_config(&current_dir);

        for cfg_file in cfg_files {
            let cfg_parser: CFGParser = CFGParser::new();
            let result: Result<Vec<CFGLine>, Error> = cfg_parser.from_file(&cfg_file);

            if let Ok(result) = result {
                let venv_cfg: Option<VirtualEnvCFG> = VirtualEnvCFG::new(cfg_file, &result);
                if let Some(venv_cfg) = venv_cfg {
                    venv_cfgs.push(venv_cfg);
                }
            }
        }
    }
    venv_cfgs
}
