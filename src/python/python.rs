use crate::general::path::WPath;
use crate::general::version::SemanticVersion;

#[derive(Clone)]
pub struct PythonEnvironment {
    pub version: SemanticVersion,
    python_path: WPath,
}

impl PythonEnvironment {
    pub fn new(base_path: WPath, version: SemanticVersion) -> Option<Self> {
        let python_path: Option<WPath> = Self::get_python_path(&base_path, &version);

        if let Some(python_path) = python_path {
            let environment: PythonEnvironment = PythonEnvironment {
                version,
                python_path,
            };
            return Some(environment);
        }
        None
    }

    pub fn get_python_executable(&self) -> WPath {
        let python_executable: WPath = self.python_path.join("python.exe");
        python_executable
    }

    fn get_python_path(base_path: &WPath, version: &SemanticVersion) -> Option<WPath> {
        let folder_name: String = format!("Python{}{}\\", version.major, version.minor);
        let folder_path: &WPath = &WPath::from_string(&folder_name);
        let python_path: WPath = base_path.join(folder_path);
        if python_path.exists() {
            return Some(python_path);
        }
        None
    }
}
