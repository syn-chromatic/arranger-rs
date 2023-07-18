use std::path::{Path, PathBuf};

use crate::python::pip::{Pip, PipShow};
use crate::shell::utils::{CommandE, CommandR};

#[derive(Clone)]
pub struct PythonVersion {
    major: usize,
    minor: usize,
}

impl PythonVersion {
    pub fn new(major: usize, minor: usize) -> Self {
        PythonVersion { major, minor }
    }

    pub fn get_folder_name(&self) -> String {
        let name: String = format!("Python{}{}\\", self.major, self.minor);
        name
    }
}

#[derive(Clone)]
pub struct PythonEnvironment {
    python_path: PathBuf,
    version: PythonVersion,
}

impl PythonEnvironment {
    pub fn new(base_path: PathBuf, version: PythonVersion) -> Option<Self> {
        let python_path: Option<PathBuf> = Self::get_python_path(&base_path, &version);

        if let Some(python_path) = python_path {
            let environment: PythonEnvironment = PythonEnvironment {
                python_path,
                version,
            };
            return Some(environment);
        }
        None
    }

    pub fn get_python_executable(&self) -> PathBuf {
        let python_executable: PathBuf = self.python_path.join("python.exe");
        python_executable
    }

    fn get_python_path(base_path: &PathBuf, version: &PythonVersion) -> Option<PathBuf> {
        let folder_name: String = version.get_folder_name();
        let folder_path: &Path = Path::new(&folder_name);
        let python_path: PathBuf = base_path.join(folder_path);
        if python_path.exists() {
            return Some(python_path);
        }
        None
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
        let major: usize = self.environment.version.major;
        let minor: usize = self.environment.version.minor;

        let name: String = format!("pyenv{}{}", major, minor);
        name
    }
}
