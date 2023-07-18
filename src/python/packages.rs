use std::path::PathBuf;

use crate::python::pip::Pip;
use crate::python::python::PythonEnvironment;
use crate::shell::utils::{CommandE, CommandR};

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
