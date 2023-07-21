use std::path::{Path, PathBuf};

use crate::parsers::cfg_parser::CFGLine;
use crate::python::pip::Pip;
use crate::python::python::PythonEnvironment;
use crate::shell::utils::{CommandE, CommandR};
use crate::types::version::SemanticVersion;

#[derive(Debug)]
pub struct VirtualEnvCFG {
    home: PathBuf,
    implementation: String,
    version_info: SemanticVersion,
    virtualenv: SemanticVersion,
    include_system_site_packages: bool,
    base_prefix: PathBuf,
    base_exec_prefix: PathBuf,
    base_executable: PathBuf,
}

impl VirtualEnvCFG {
    pub fn new(parsed_cfg: &Vec<CFGLine>) -> Option<Self> {
        let mut home: Option<PathBuf> = None;
        let mut implementation: Option<String> = None;
        let mut version_info: Option<SemanticVersion> = None;
        let mut virtualenv: Option<SemanticVersion> = None;
        let mut include_system_site_packages: Option<bool> = None;
        let mut base_prefix: Option<PathBuf> = None;
        let mut base_exec_prefix: Option<PathBuf> = None;
        let mut base_executable: Option<PathBuf> = None;

        for cfg_line in parsed_cfg {
            let cfg_name: &str = cfg_line.get_name();
            let cfg_setting: &str = cfg_line.get_setting();
            match cfg_name {
                "home" => home = Some(Path::new(cfg_setting).to_path_buf()),
                "implementation" => implementation = Some(cfg_setting.to_string()),
                "version_info" => {
                    version_info = Some(SemanticVersion::new_from_string(cfg_setting)?)
                }
                "virtualenv" => virtualenv = Some(SemanticVersion::new_from_string(cfg_setting)?),
                "include-system-site-packages" => {
                    include_system_site_packages = Some(Self::parse_boolean_string(cfg_setting)?)
                }
                "base-prefix" => base_prefix = Some(Path::new(cfg_setting).to_path_buf()),
                "base-exec-prefix" => base_exec_prefix = Some(Path::new(cfg_setting).to_path_buf()),
                "base-executable" => base_executable = Some(Path::new(cfg_setting).to_path_buf()),
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

    pub fn create_virtual_env(&self) {
        let pip = Pip::new(&self.environment);

        let python_executable: PathBuf = self.environment.get_python_executable();
        let venv_name: String = self.get_virtual_env_name();
        let venv_args: [&str; 3] = ["-m", "virtualenv", &venv_name];
        let package_name = "virtualenv";
        let pip_show = pip.find_package(package_name);

        let mut venv_installed = false;

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

        println!("CREATING VIRTUAL ENV");
        let command: CommandE = CommandE::new();
        let response: Option<CommandR> = command.execute_command(&python_executable, &venv_args);
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
