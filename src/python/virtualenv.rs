use crate::general::path::WPath;
use crate::general::shell::{CommandExecute, CommandResponse};
use crate::general::version::SemanticVersion;
use crate::parsers::cfg_parser::CFGLine;
use crate::python::pip::{Pip, PipShow};
use crate::python::python::PythonEnvironment;

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
}

impl VirtualEnv {
    pub fn new(environment: &PythonEnvironment) -> Self {
        let environment: PythonEnvironment = environment.clone();
        VirtualEnv { environment }
    }

    pub fn create_virtual_env_in_path(&self, mut path: WPath) {
        path.to_directory();
        let canonical_string: Option<String> = path.get_canonical_string();

        if let Some(canonical_string) = canonical_string {
            let python_version: String = self.environment.version.get_version_string();
            println!(
                "\nCreating Virtual Environment for Python {} in: {}",
                python_version, canonical_string
            );

            let venv_args: [&str; 3] = ["-m", "virtualenv", &canonical_string];
            self.execute_virtual_env_command(&venv_args);
        }
    }

    pub fn create_virtual_env(&self) {
        let venv_name: String = self.get_virtual_env_name();
        let venv_args: [&str; 3] = ["-m", "virtualenv", &venv_name];
        self.execute_virtual_env_command(&venv_args);
    }

    fn execute_virtual_env_command(&self, venv_args: &[&str]) {
        let pip: Pip = Pip::new(&self.environment);
        let python_executable: WPath = self.environment.get_python_executable();

        let package_name: &str = "virtualenv";
        let pip_show: Option<PipShow> = pip.find_package(package_name);

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
            println!("INSTALLING PACKAGE");
            pip.install_package(package_name);
        }

        let command: CommandExecute = CommandExecute::new();
        let response: Option<CommandResponse> =
            command.execute_command(&python_executable, &venv_args);
        if let Some(response) = response {
            println!("{:?}", response);
        }
    }

    fn get_virtual_env_name(&self) -> String {
        let (major, minor) = self.environment.version.get_version();

        let name: String = format!("pyenv{}{}", major, minor);
        name
    }
}
