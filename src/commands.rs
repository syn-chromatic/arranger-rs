use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::fs::ReadDir;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use dirs;

use crate::general::terminal::Terminal;
use crate::general::terminal::{ANSICode, CyanANSI, WhiteANSI};
use crate::general::terminal::{GreenANSI, RedANSI, YellowANSI};

use crate::FixVirtualEnvOption;
use crate::PackagesOption;
use crate::PythonDownloadOption;
use crate::VirtualEnvOption;

use crate::general::http::HTTP;
use crate::general::path::WPath;
use crate::general::version::SemanticVersion;
use crate::parsers::cfg_parser::CFGLine;
use crate::parsers::cfg_parser::CFGParser;
use crate::python::pip::{PipPackage, PipPackageParser};
use crate::python::python::PythonEnvironment;
use crate::python::python_ftp::PythonFTPRetriever;
use crate::python::virtualenv::VirtualEnv;
use crate::python::virtualenv::VirtualEnvCFG;
use crate::search::file::FileSearch;
use crate::utils::confirm_and_continue;

pub struct PythonCreateEnvCommand {
    option: VirtualEnvOption,
}

impl PythonCreateEnvCommand {
    pub fn new(option: VirtualEnvOption) -> Self {
        PythonCreateEnvCommand { option }
    }

    pub fn execute_command(&self) {
        let terminal: Terminal = Terminal::new();
        let mut version: SemanticVersion = self.option.version.clone();
        let data_dir: Option<PathBuf> = dirs::data_local_dir();

        let version_string: String = version.get_2p_string();

        if let Some(data_dir) = data_dir {
            let base_path_buf: PathBuf = data_dir.join("Programs/Python");
            let base_path: WPath = WPath::from_path_buf(&base_path_buf);
            version.set_patch(0);

            let environment: Option<PythonEnvironment> =
                PythonEnvironment::new(&base_path, &version);

            if let Some(environment) = environment {
                let virtual_env: VirtualEnv = VirtualEnv::new(&environment);
                virtual_env.create_environment();
            } else {
                let string: String =
                    format!("Couldn't find Python {} installation.", version_string);
                terminal.writeln_color(&string, RedANSI);
            }
        }
    }
}

pub struct PythonFixEnvCommand {
    option: FixVirtualEnvOption,
}

impl PythonFixEnvCommand {
    pub fn new(option: FixVirtualEnvOption) -> Self {
        PythonFixEnvCommand { option }
    }

    pub fn execute_command(&self) {
        let deep_search: bool = self.option.deep_search;

        let terminal: Terminal = Terminal::new();
        let parameters: String = format!("Deep Search: [{}]", deep_search);
        let parts: [&str; 2] = ["Search Parameters: ", &parameters];
        let colors: [Box<dyn ANSICode>; 2] = [YellowANSI.boxed(), WhiteANSI.boxed()];
        terminal.writeln_color_p(&parts, &colors);

        let venv_search: VirtualEnvSearch = VirtualEnvSearch::new(deep_search);
        let venv_cfgs: Vec<VirtualEnvCFG> = venv_search.find_configs();

        for venv_cfg in venv_cfgs {
            let mut environment_directory: WPath = venv_cfg.file.clone().into();
            environment_directory.to_directory();

            let directory_string: String = format!("{:?}", environment_directory);
            let parts: [&str; 2] = ["Attempting Environment Fix: ", &directory_string];
            terminal.writeln_2p_primary(&parts, YellowANSI);

            let version: SemanticVersion = venv_cfg.version_info;
            let cfg_file: WPath = venv_cfg.file;
            self.create_env(&cfg_file, &version, &terminal);
        }
    }
}

impl PythonFixEnvCommand {
    fn create_env(&self, path: &WPath, version: &SemanticVersion, terminal: &Terminal) {
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
            } else {
                let version_string: String = version.get_2p_string();
                let string: String =
                    format!("Couldn't find Python {} installation.\n", version_string);
                terminal.writeln_color(&string, RedANSI);
            }
        }
    }
}

pub struct PythonPackagesCommand {
    option: PackagesOption,
}

impl PythonPackagesCommand {
    pub fn new(option: PackagesOption) -> Self {
        PythonPackagesCommand { option }
    }

    pub fn execute_command(&self) {
        let deep_search: bool = self.option.deep_search;
        let save_packages: bool = self.option.save_packages;

        let terminal: Terminal = Terminal::new();
        let parameters: String = format!("Deep Search: [{}]", deep_search);
        let parts: [&str; 2] = ["Search Parameters: ", &parameters];
        let colors: [Box<dyn ANSICode>; 2] = [YellowANSI.boxed(), WhiteANSI.boxed()];
        terminal.writeln_color_p(&parts, &colors);

        let venv_search: VirtualEnvSearch = VirtualEnvSearch::new(deep_search);
        let venv_cfgs: Vec<VirtualEnvCFG> = venv_search.find_configs();

        for venv_cfg in venv_cfgs {
            let env_dir: WPath = venv_cfg.get_environment_directory();
            let package_parser: PipPackageParser = self.parse_packages(&env_dir);
            let packages: &Vec<PipPackage> = package_parser.get_packages();

            let string: String = format!("Environment: {:?}", env_dir);
            terminal.writeln_color(&string, YellowANSI);

            self.list_packages(&packages);

            if save_packages {
                self.save_packages(&env_dir, packages);
            }

            println!()
        }
    }
}

impl PythonPackagesCommand {
    fn parse_packages(&self, env_dir: &WPath) -> PipPackageParser {
        let mut package_parser: PipPackageParser = PipPackageParser::new();
        let packages_dir: WPath = env_dir.join("Lib/site-packages/");

        let read_dir: Result<ReadDir, io::Error> = packages_dir.read_dir();
        if let Ok(read_dir) = read_dir {
            for entry in read_dir {
                if let Ok(entry) = entry {
                    let entry_path: PathBuf = entry.path();
                    if entry_path.is_dir() {
                        let entry_name: Option<&str> =
                            entry_path.file_name().unwrap_or_default().to_str();
                        if let Some(entry_name) = entry_name {
                            package_parser.parse(entry_name);
                        }
                    }
                }
            }
        }
        package_parser
    }

    fn list_packages(&self, packages: &Vec<PipPackage>) {
        for package in packages {
            let package_string: String = package.get_string();
            println!("{}", package_string);
        }
    }

    fn save_packages(&self, env_dir: &WPath, packages: &Vec<PipPackage>) {
        let file_path: WPath = env_dir.join("packages.txt");
        let mut file: File = File::create(&file_path).expect("Could not create file");

        for package in packages {
            let requirement_string: String = package.get_requirement_string();
            writeln!(file, "{}", requirement_string).expect("Could not write to file");
        }

        let terminal = Terminal::new();
        let path_str: String = format!("[{:?}]", file_path);
        let parts: [&str; 2] = ["Packages List Saved: ", &path_str];
        let colors: [Box<dyn ANSICode>; 2] = [GreenANSI.boxed(), WhiteANSI.boxed()];
        terminal.writeln_color_p(&parts, &colors);
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
            let colors: [Box<dyn ANSICode>; 2] = [GreenANSI.boxed(), WhiteANSI.boxed()];
            terminal.writeln_color_p(&parts, &colors);

            let http: HTTP = HTTP::new();
            let result: Result<String, Box<dyn Error>> = http.download_file(&url).await;
            if let Ok(file_name) = result {
                let parts: [&str; 2] = ["File Downloaded: ", &file_name];
                let colors: [Box<dyn ANSICode>; 2] = [GreenANSI.boxed(), WhiteANSI.boxed()];
                terminal.writeln_color_p(&parts, &colors);
                return;
            } else {
                let error: String = result.unwrap_err().to_string();
                let parts: [&str; 2] = ["Error: ", &error];
                let colors: [Box<dyn ANSICode>; 2] = [RedANSI.boxed(), WhiteANSI.boxed()];
                terminal.writeln_color_p(&parts, &colors);
            }
        }
        let string: &str = "Python version not found.";
        terminal.writeln_color(&string, RedANSI);
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
        let terminal: Terminal = Terminal::new();
        let arch: &str = &self.option.architecture;
        let platform: &str = &self.option.platform;
        let package_type: &str = &self.option.package_type;

        let parameters: String = format!(
            "Arch: [{}] | Platform: [{}] | Type: [{}]\n",
            arch, platform, package_type
        );
        let parts: [&str; 2] = ["Search Parameters: ", &parameters];
        let colors: [Box<dyn ANSICode>; 2] = [YellowANSI.boxed(), WhiteANSI.boxed()];
        terminal.writeln_color_p(&parts, &colors);
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
            let cfg_files: HashSet<PathBuf> = self.find_config(&current_dir);

            if !self.confirm_search(&cfg_files) {
                return venv_cfgs;
            }

            for cfg_file in cfg_files {
                let cfg_parser: CFGParser = CFGParser::new();
                let result: Result<Vec<CFGLine>, io::Error> = cfg_parser.from_file(&cfg_file);

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
}

impl VirtualEnvSearch {
    fn find_config(&self, root: &PathBuf) -> HashSet<PathBuf> {
        let mut file_search: FileSearch = FileSearch::new();

        let exclusive_filenames: Vec<&str> = vec!["pyvenv.cfg"];
        let exclusive_exts: Vec<&str> = vec![];
        let exclude_dirs: Vec<&str> = vec![];
        let quit_directory_on_match: bool = !self.deep_search;

        file_search.set_root(root);
        file_search.set_exclusive_filenames(exclusive_filenames);
        file_search.set_exclusive_extensions(exclusive_exts);
        file_search.set_exclude_directories(exclude_dirs);
        file_search.set_quit_directory_on_match(quit_directory_on_match);

        let files: HashSet<PathBuf> = file_search.search_files();
        files
    }

    fn confirm_search(&self, files: &HashSet<PathBuf>) -> bool {
        let terminal: Terminal = Terminal::new();
        terminal.writeln_color("\nFound Environments:", GreenANSI);

        for file in files {
            let mut environment_directory: WPath = file.into();
            environment_directory.to_directory();

            let directory_string = environment_directory.get_canonical_string();
            if let Some(directory_string) = directory_string {
                let path_str: String = format!("[{}]", directory_string);
                let parts: [&str; 2] = ["Path: ", &path_str];
                let colors: [Box<dyn ANSICode>; 2] = [CyanANSI.boxed(), WhiteANSI.boxed()];
                terminal.writeln_color_p(&parts, &colors);
            }
        }
        confirm_and_continue()
    }
}
