use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::str::SplitWhitespace;

use dirs;

use crate::general::terminal::Terminal;
use crate::general::terminal::{GreenANSI, RedANSI, YellowANSI};

use crate::commands::configuration::FixVirtualEnvOption;
use crate::commands::configuration::PackagesOption;
use crate::commands::configuration::PythonDownloadOption;
use crate::commands::configuration::VirtualEnvExecuteOption;
use crate::commands::configuration::VirtualEnvOption;

use crate::general::grid_printer::DynamicTable;
use crate::general::https::HTTPS;
use crate::general::path::WPath;
use crate::general::version::SemanticVersion;

use crate::python::ftp::PythonFTPRetriever;
use crate::python::pip::{PipMetadata, PipPackage, PipPackageParser};
use crate::python::python::PythonEnvironment;
use crate::python::virtualenv::VirtualEnv;
use crate::python::virtualenv::VirtualEnvCFG;
use crate::python::virtualenv::VirtualEnvSearch;

pub struct PythonCreateEnvCommand {
    option: VirtualEnvOption,
}

impl PythonCreateEnvCommand {
    pub fn new(option: VirtualEnvOption) -> Self {
        PythonCreateEnvCommand { option }
    }

    pub fn execute_command(&self) {
        let mut version: SemanticVersion = self.option.version.clone();
        let data_dir: Option<PathBuf> = dirs::data_local_dir();

        if let Some(data_dir) = data_dir {
            let base_path_buf: PathBuf = data_dir.join("Programs/Python");
            let base_path: WPath = WPath::from_path_buf(&base_path_buf);
            version.set_patch(0);

            let environment: Option<PythonEnvironment> =
                PythonEnvironment::new(&base_path, &version);

            if let Some(environment) = environment {
                let virtual_env: VirtualEnv = VirtualEnv::new(&environment);
                virtual_env.create_environment();
            }
        }
    }
}

pub struct PythonFixEnvCommand {
    option: FixVirtualEnvOption,
    terminal: Terminal,
}

impl PythonFixEnvCommand {
    pub fn new(option: FixVirtualEnvOption) -> Self {
        let terminal = Terminal::new();
        PythonFixEnvCommand { option, terminal }
    }

    pub fn execute_command(&self) {
        let deep_search: bool = self.option.deep_search;
        self.print_search_parameters();

        let venv_search: VirtualEnvSearch = VirtualEnvSearch::new(deep_search);
        let venv_cfgs: Vec<VirtualEnvCFG> = venv_search.find_configs();

        for venv_cfg in venv_cfgs {
            let mut environment_directory: WPath = venv_cfg.cfg_file.clone().into();
            environment_directory.to_directory();

            let directory_string: String = format!("{:?}", environment_directory);
            let parts: [&str; 2] = ["Attempting Environment Fix: ", &directory_string];
            self.terminal.writeln_parameter(&parts, &YellowANSI);

            let version: SemanticVersion = venv_cfg.version_info;
            let cfg_file: WPath = venv_cfg.cfg_file;
            self.create_env(&cfg_file, &version);
            println!();
        }
    }
}

impl PythonFixEnvCommand {
    fn create_env(&self, path: &WPath, version: &SemanticVersion) {
        let data_dir: Option<PathBuf> = dirs::data_local_dir();
        let mut version: SemanticVersion = version.clone();
        version.set_patch(0);

        if let Some(data_dir) = data_dir {
            let base_path_buf: PathBuf = data_dir.join("Programs/Python");
            let base_path: WPath = WPath::from_path_buf(&base_path_buf);

            let environment: Option<PythonEnvironment> =
                PythonEnvironment::new(&base_path, &version);

            if let Some(environment) = environment {
                let virtual_env: VirtualEnv = VirtualEnv::new(&environment);
                virtual_env.create_environment_in_path(path);
            }
        }
    }

    fn print_search_parameters(&self) {
        let deep_search: bool = self.option.deep_search;
        let mut table = DynamicTable::new(0.6, 1);
        table.set_header("Fix Venv Parameters");
        table.add_parameter("Deep Search", deep_search);
        table.print();
        println!();
    }
}

pub struct PythonExecuteCommand {
    option: VirtualEnvExecuteOption,
    terminal: Terminal,
}

impl PythonExecuteCommand {
    pub fn new(option: VirtualEnvExecuteOption) -> Self {
        let terminal: Terminal = Terminal::new();
        PythonExecuteCommand { option, terminal }
    }

    pub fn execute_command(&self) {
        let deep_search: bool = self.option.deep_search;
        self.print_search_parameters();

        let args: Vec<&str> = self.get_command_args();
        let venv_search: VirtualEnvSearch = VirtualEnvSearch::new(deep_search);
        let venv_cfgs: Vec<VirtualEnvCFG> = venv_search.find_configs();

        for (idx, venv_cfg) in venv_cfgs.iter().enumerate() {
            let env_directory: WPath = venv_cfg.get_environment_directory();
            let python_executable: WPath = venv_cfg.get_python_executable();
            let version: &SemanticVersion = &venv_cfg.version_info;
            let environment: Option<PythonEnvironment> =
                PythonEnvironment::from_custom_path(&env_directory, &python_executable, version);

            if let Some(environment) = environment {
                let string: String = format!("[Environment -> {:?}]", env_directory);
                self.terminal.writeln_ansi(&string, &YellowANSI);

                let virtual_env: VirtualEnv = VirtualEnv::new(&environment);
                virtual_env.execute_custom_command(&args);
            }

            if idx != (venv_cfgs.len() - 1) {
                println!()
            }
        }
    }
}

impl PythonExecuteCommand {
    fn get_command_args(&self) -> Vec<&str> {
        let command: &str = &self.option.command;
        let split: SplitWhitespace = command.split_whitespace();
        let args: Vec<&str> = split.into_iter().collect();
        args
    }
    fn print_search_parameters(&self) {
        let deep_search: bool = self.option.deep_search;
        let command: &String = &self.option.command;

        let mut table: DynamicTable = DynamicTable::new(0.6, 1);
        table.set_header("Execute Parameters");
        table.add_parameter("Deep Search", deep_search);
        table.add_parameter("Command", command);
        table.print();
        println!();
    }
}

pub struct PythonPackagesCommand {
    option: PackagesOption,
    terminal: Terminal,
}

impl PythonPackagesCommand {
    pub fn new(option: PackagesOption) -> Self {
        let terminal: Terminal = Terminal::new();
        PythonPackagesCommand { option, terminal }
    }

    pub fn execute_command(&self) {
        let deep_search: bool = self.option.deep_search;
        let save: bool = self.option.save;

        self.print_search_parameters();

        let venv_search: VirtualEnvSearch = VirtualEnvSearch::new(deep_search);
        let venv_cfgs: Vec<VirtualEnvCFG> = venv_search.find_configs();

        for (idx, venv_cfg) in venv_cfgs.iter().enumerate() {
            let env_dir: WPath = venv_cfg.get_environment_directory();

            let packages: Result<Vec<PipPackage>, io::Error> =
                self.get_packages_from_option(&env_dir);

            if let Ok(packages) = packages {
                let string: String = format!("[Environment -> {:?}]", env_dir);
                self.terminal.writeln_ansi(&string, &YellowANSI);

                self.list_packages(&packages);

                if save {
                    self.save_packages(&env_dir, &packages);
                }

                if idx != (venv_cfgs.len() - 1) {
                    println!()
                }
            }
        }
    }
}

impl PythonPackagesCommand {
    fn print_search_parameters(&self) {
        let deep_search: bool = self.option.deep_search;
        let save: bool = self.option.save;
        let distill: bool = self.option.distill;

        let mut table: DynamicTable = DynamicTable::new(0.6, 1);
        table.set_header("Packages Parameters");
        table.add_parameter("Deep Search", deep_search);
        table.add_parameter("Distill", distill);
        table.add_parameter("Save", save);
        table.print();
        println!();
    }

    fn get_filename_from_option(&self) -> &str {
        let distill: bool = self.option.distill;
        let filename: &str = if distill {
            "distilled_packages.txt"
        } else {
            "packages.txt"
        };
        filename
    }

    fn get_packages_from_option(&self, env_dir: &WPath) -> Result<Vec<PipPackage>, io::Error> {
        let distill: bool = self.option.distill;
        let packages: Result<Vec<PipPackage>, io::Error> = if distill {
            self.get_distilled_packages(&env_dir)
        } else {
            self.get_packages(&env_dir)
        };
        packages
    }

    fn get_packages(&self, env_dir: &WPath) -> Result<Vec<PipPackage>, io::Error> {
        let packages_dir: WPath = env_dir.join("Lib/site-packages/");
        let package_parser: PipPackageParser = PipPackageParser::new(&packages_dir);

        let packages: Vec<PipPackage> = package_parser.get_packages()?;

        Ok(packages)
    }

    fn get_distilled_packages(&self, env_dir: &WPath) -> Result<Vec<PipPackage>, io::Error> {
        let packages_dir: WPath = env_dir.join("Lib/site-packages/");
        let package_parser: PipPackageParser = PipPackageParser::new(&packages_dir);

        let mut packages: Vec<PipPackage> = package_parser.get_packages()?;
        let metadata: Vec<PipMetadata> = package_parser.get_metadata(&packages);
        package_parser.distill_packages(&mut packages, &metadata);

        Ok(packages)
    }

    fn list_packages(&self, packages: &Vec<PipPackage>) {
        for package in packages {
            let package_string: String = package.get_string();
            println!("{}", package_string);
        }

        let packages_length: String = packages.len().to_string();
        let parts: [&str; 2] = ["Total Packages: ", &packages_length];
        self.terminal.writeln_parameter(&parts, &YellowANSI);
    }

    fn save_packages(&self, env_dir: &WPath, packages: &Vec<PipPackage>) {
        let filename: &str = self.get_filename_from_option();
        let file_path: WPath = env_dir.join(filename);
        let mut file: File = File::create(&file_path).expect("Could not create file");

        for package in packages {
            let requirement_string: String = package.get_requirement_string();
            writeln!(file, "{}", requirement_string).expect("Could not write to file");
        }

        let terminal = Terminal::new();
        let path_str: String = format!("[{:?}]", file_path);
        let parts: [&str; 2] = ["Packages List Saved: ", &path_str];
        terminal.writeln_parameter(&parts, &GreenANSI);
    }
}

pub struct PythonDLCommand {
    option: PythonDownloadOption,
}

impl PythonDLCommand {
    pub fn new(option: PythonDownloadOption) -> Self {
        PythonDLCommand { option }
    }

    pub async fn execute_command(&mut self) {
        let ftp_retriever: PythonFTPRetriever = PythonFTPRetriever::new();
        let mut version: SemanticVersion = self.option.version.clone();

        self.print_search_parameters();
        let url: Option<String> = self.get_url(&ftp_retriever, &mut version).await;

        if self.option.list_structure {
            ftp_retriever.list_file_structure(&version).await;
            return;
        }

        self.download_from_url(&url).await;
    }
}

impl PythonDLCommand {
    async fn download_from_url(&self, url: &Option<String>) {
        let terminal: Terminal = Terminal::new();

        if let Some(url) = url {
            let parts: [&str; 2] = ["Found version: ", url];
            terminal.writeln_parameter(&parts, &GreenANSI);

            let https: HTTPS = HTTPS::new();
            let result: Result<String, Box<dyn Error>> = https.download_file(&url).await;
            if let Ok(file_name) = result {
                let parts: [&str; 2] = ["File Downloaded: ", &file_name];
                terminal.writeln_parameter(&parts, &GreenANSI);
                return;
            } else {
                let error: String = result.unwrap_err().to_string();
                let parts: [&str; 2] = ["Error: ", &error];
                terminal.writeln_parameter(&parts, &RedANSI);
            }
        }
        let string: &str = "Python version not found.";
        terminal.writeln_ansi(&string, &RedANSI);
    }

    async fn get_url(
        &self,
        ftp_retriever: &PythonFTPRetriever,
        version: &mut SemanticVersion,
    ) -> Option<String> {
        let recent_patch: bool = self.option.recent_patch;
        let arch: &str = &self.option.architecture;
        let platform: &str = &self.option.platform;
        let package_type: &str = &self.option.package_type;

        let url: Option<String> = if recent_patch {
            ftp_retriever
                .get_setup_file_latest_patch(version, arch, platform, package_type)
                .await
        } else {
            ftp_retriever
                .get_setup_file(version, arch, platform, package_type)
                .await
        };
        url
    }

    fn print_search_parameters(&self) {
        let arch: &str = &self.option.architecture;
        let platform: &str = &self.option.platform;
        let package_type: &str = &self.option.package_type;
        let recent_patch: bool = self.option.recent_patch;
        let list_structure: bool = self.option.list_structure;

        let mut table: DynamicTable = DynamicTable::new(0.6, 1);
        table.set_header("Download Parameters");
        table.add_parameter("Arch", arch);
        table.add_parameter("Platform", platform);
        table.add_parameter("Type", package_type);
        table.add_parameter("Most recent Patch", recent_patch);
        table.add_parameter("List Results", list_structure);
        table.print();
        println!();
    }
}
