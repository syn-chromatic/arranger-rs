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

    pub fn get_version(&self) -> (usize, usize) {
        (self.major, self.minor)
    }
}

#[derive(Clone)]
pub struct PythonEnvironment {
    pub version: PythonVersion,
    python_path: PathBuf,
}

impl PythonEnvironment {
    pub fn new(base_path: PathBuf, version: PythonVersion) -> Option<Self> {
        let python_path: Option<PathBuf> = Self::get_python_path(&base_path, &version);

        if let Some(python_path) = python_path {
            let environment: PythonEnvironment = PythonEnvironment {
                version,
                python_path,
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
