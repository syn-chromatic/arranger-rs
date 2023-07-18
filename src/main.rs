mod python;
mod shell;

use crate::python::packages::VirtualEnv;
use crate::python::python::{PythonEnvironment, PythonVersion};
use std::path::PathBuf;

use dirs;

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

fn main() {
    create_virtual_env(3, 7);
}
