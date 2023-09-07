use std::collections::HashSet;
use std::io;
use std::path::PathBuf;

use crate::general::path::WPath;
use crate::general::shell::{CommandExecute, CommandResponse};
use crate::general::version::SemanticVersion;
use crate::parsers::cfg_parser::CFGLine;
use crate::parsers::cfg_parser::CFGParser;
use crate::python::pip::{Pip, PipShow};
use crate::python::python::PythonEnvironment;
use crate::search::file_search::{FileSearch, SearchThreadScheduler};
use crate::search::info::FileInfo;

use crate::terminal::Terminal;
use crate::terminal::{CyanANSI, GreenANSI, RedANSI};

use crate::utils::ConfirmationPrompt;

#[derive(Debug)]
pub struct VirtualEnvCFG {
    pub home: WPath,
    pub implementation: String,
    pub version_info: SemanticVersion,
    pub virtualenv: SemanticVersion,
    pub include_system_site_packages: bool,
    pub base_prefix: WPath,
    pub base_exec_prefix: WPath,
    pub base_executable: WPath,
    pub cfg_file: WPath,
}

impl VirtualEnvCFG {
    pub fn new(cfg_file: WPath, parsed_cfg: &Vec<CFGLine>) -> Option<Self> {
        let mut home: Option<WPath> = None;
        let mut implementation: Option<String> = None;
        let mut version_info: Option<SemanticVersion> = None;
        let mut virtualenv: Option<SemanticVersion> = None;
        let mut include_system_site_packages: Option<bool> = None;
        let mut base_prefix: Option<WPath> = None;
        let mut base_exec_prefix: Option<WPath> = None;
        let mut base_executable: Option<WPath> = None;

        for cfg_line in parsed_cfg {
            let cfg_name: String = cfg_line.get_name().to_string();
            let cfg_setting: String = cfg_line.get_setting().to_string();
            let cfg_path: WPath = WPath::from_string(&cfg_setting);
            let cfg_version: Option<SemanticVersion> = SemanticVersion::from_string(&cfg_setting);
            let cfg_boolean: Option<bool> = Self::parse_boolean_string(&cfg_setting);

            match cfg_name.as_ref() {
                "home" => home = Some(cfg_path),
                "implementation" => implementation = Some(cfg_setting),
                "version_info" => version_info = Some(cfg_version?),
                "virtualenv" => virtualenv = Some(cfg_version?),
                "include-system-site-packages" => include_system_site_packages = Some(cfg_boolean?),
                "base-prefix" => base_prefix = Some(cfg_path),
                "base-exec-prefix" => base_exec_prefix = Some(cfg_path),
                "base-executable" => base_executable = Some(cfg_path),
                _ => return None,
            }
        }

        let venv_cfg: VirtualEnvCFG = VirtualEnvCFG {
            home: home.unwrap(),
            implementation: implementation.unwrap(),
            version_info: version_info.unwrap(),
            virtualenv: virtualenv.unwrap(),
            include_system_site_packages: include_system_site_packages.unwrap(),
            base_prefix: base_prefix.unwrap(),
            base_exec_prefix: base_exec_prefix.unwrap(),
            base_executable: base_executable.unwrap(),
            cfg_file,
        };
        Some(venv_cfg)
    }

    pub fn get_environment_directory(&self) -> WPath {
        let directory: WPath = self.cfg_file.as_directory();
        directory
    }

    pub fn get_python_executable(&self) -> WPath {
        let directory: WPath = self.get_environment_directory();
        let python_executable: WPath = directory.join("Scripts/python.exe");
        python_executable
    }
}

impl VirtualEnvCFG {
    fn parse_boolean_string(boolean: &str) -> Option<bool> {
        match boolean {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }
}

pub struct VirtualEnv {
    environment: PythonEnvironment,
    terminal: Terminal,
}

impl VirtualEnv {
    pub fn new(environment: &PythonEnvironment) -> Self {
        let environment: PythonEnvironment = environment.clone();
        let terminal = Terminal::new();
        VirtualEnv {
            environment,
            terminal,
        }
    }

    pub fn create_environment(&self) {
        let venv_name: String = self.get_environment_name();
        let venv_args: [&str; 3] = ["-m", "virtualenv", &venv_name];
        self.execute_venv_command(&venv_args);
    }

    pub fn create_environment_in_path(&self, path: &WPath) {
        let path: WPath = path.as_directory();
        let canonical_string: Option<String> = path.get_canonical_string();

        if let Some(canonical_string) = canonical_string {
            let venv_args: [&str; 3] = ["-m", "virtualenv", &canonical_string];
            self.execute_venv_command(&venv_args);
        }
    }

    pub fn execute_custom_command(&self, args: &[&str]) {
        let python_executable: &WPath = self.environment.get_python_executable();
        self.print_executing_custom_command();
        let command: CommandExecute = CommandExecute::new();
        command.execute_spawn_command(&python_executable, &args);
    }
}

impl VirtualEnv {
    fn execute_venv_command(&self, venv_args: &[&str]) {
        let pip: &Pip = self.environment.get_pip();
        let python_executable: &WPath = self.environment.get_python_executable();

        let package_name: &str = "virtualenv";
        let pip_show: Option<PipShow> = pip.find_package(&self.environment, package_name);
        let mut venv_installed: bool = false;

        if let Some(pip_show) = pip_show {
            let pip_name: Option<&str> = pip_show.get_name();
            if let Some(pip_name) = pip_name {
                if pip_name == package_name {
                    venv_installed = true;
                }
            }
        }

        if !venv_installed {
            self.print_installing_package();
            let state: bool = pip.install_package(&self.environment, package_name);
            println!();
            if !state {
                return;
            }
        }

        self.print_creating_environment();
        let command: CommandExecute = CommandExecute::new();
        let response: Option<CommandResponse> =
            command.execute_command(&python_executable, &venv_args);
        if let Some(response) = response {
            response.print();
        }
    }

    fn get_environment_name(&self) -> String {
        let version: &SemanticVersion = self.environment.get_python_version();
        let (major, minor): (usize, usize) = version.get_2p_version();
        let name: String = format!("pyenv{}{}", major, minor);
        name
    }

    fn print_installing_package(&self) {
        let string: &str = "[Installing Virtual Environment Package]\n";
        self.terminal.writeln_ansi(string, &CyanANSI);
    }

    fn print_creating_environment(&self) {
        let version: &SemanticVersion = self.environment.get_python_version();
        let version_string: String = version.get_2p_string();
        let string: String = format!("[Creating Python {} Environment]\n", version_string);
        self.terminal.writeln_ansi(&string, &CyanANSI);
    }

    fn print_executing_custom_command(&self) {
        let string: &str = "[Executing command]\n";
        self.terminal.writeln_ansi(string, &CyanANSI);
    }
}

pub struct VirtualEnvSearch {
    deep_search: bool,
}

impl VirtualEnvSearch {
    pub fn new(deep_search: bool) -> Self {
        VirtualEnvSearch { deep_search }
    }

    pub fn find_configs(&self) -> Vec<VirtualEnvCFG> {
        let current_dir: Result<PathBuf, io::Error> = std::env::current_dir();

        let mut venv_cfgs: Vec<VirtualEnvCFG> = Vec::new();
        if let Ok(current_dir) = current_dir {
            let cfg_files: HashSet<FileInfo> = self.find_config(&current_dir);

            if !self.confirm_search(&cfg_files) {
                return venv_cfgs;
            }

            for file_info in cfg_files {
                let cfg_file: &PathBuf = file_info.get_path();
                let cfg_parser: CFGParser = CFGParser::new();
                let result: Result<Vec<CFGLine>, io::Error> = cfg_parser.from_file(cfg_file);

                if let Ok(result) = result {
                    let cfg_file: WPath = WPath::from_path_buf(cfg_file);
                    let venv_cfg: Option<VirtualEnvCFG> = VirtualEnvCFG::new(cfg_file, &result);
                    if let Some(venv_cfg) = venv_cfg {
                        venv_cfgs.push(venv_cfg);
                    }
                }
            }
        }
        venv_cfgs
    }
}

impl VirtualEnvSearch {
    fn find_config(&self, root: &PathBuf) -> HashSet<FileInfo> {
        let mut file_search: FileSearch = FileSearch::new();

        let filename: &str = "pyvenv.cfg";
        let quit_directory_on_match: bool = !self.deep_search;

        file_search.set_root(root);
        file_search.set_exclusive_filename(filename);
        file_search.set_quit_directory_on_match(quit_directory_on_match);

        let threads: usize = 4;
        let batch_size: usize = 100;
        let search_scheduler: SearchThreadScheduler =
            SearchThreadScheduler::new(threads, batch_size, file_search);

        let files: HashSet<FileInfo> = search_scheduler.search_files();
        files
    }

    fn confirm_search(&self, files: &HashSet<FileInfo>) -> bool {
        let terminal: Terminal = Terminal::new();

        if files.len() == 0 {
            let string: &str =
                "\nNo environments were found.\nTry creating one with: arranger python venv\n";
            terminal.writeln_ansi(string, &RedANSI);
            return false;
        }

        terminal.writeln_ansi("\nFound Environments:", &GreenANSI);

        for file in files {
            let mut environment_directory: WPath = file.get_path().into();
            environment_directory.to_directory();

            let directory_string = environment_directory.get_canonical_string();
            if let Some(directory_string) = directory_string {
                let path_str: String = format!("[{}]", directory_string);
                let parts: [&str; 2] = ["Path: ", &path_str];
                terminal.writeln_parameter(&parts, &CyanANSI);
            }
        }

        ConfirmationPrompt::prompt(&terminal)
    }
}
