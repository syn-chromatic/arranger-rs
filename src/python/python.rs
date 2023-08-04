use crate::python::version::PythonVersion;
use crate::general::path::AbPath;

#[derive(Clone)]
pub struct PythonEnvironment {
    pub version: PythonVersion,
    python_path: AbPath,
}

impl PythonEnvironment {
    pub fn new(base_path: AbPath, version: PythonVersion) -> Option<Self> {
        let python_path: Option<AbPath> = Self::get_python_path(&base_path, &version);

        if let Some(python_path) = python_path {
            let environment: PythonEnvironment = PythonEnvironment {
                version,
                python_path,
            };
            return Some(environment);
        }
        None
    }

    pub fn get_python_executable(&self) -> AbPath {
        let python_executable: AbPath = self.python_path.join("python.exe");
        python_executable
    }

    fn get_python_path(base_path: &AbPath, version: &PythonVersion) -> Option<AbPath> {
        let folder_name: String = version.get_folder_name();
        let folder_path: &AbPath = &AbPath::from_string(&folder_name);
        let python_path: AbPath = base_path.join(folder_path);
        if python_path.exists() {
            return Some(python_path);
        }
        None
    }
}
