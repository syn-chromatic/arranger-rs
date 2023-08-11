use crate::general::path::WPath;
use crate::general::terminal::RedANSI;
use crate::general::terminal::Terminal;
use crate::general::version::SemanticVersion;
use crate::python::pip::Pip;

#[derive(Clone)]
pub struct PythonEnvironment {
    python_version: SemanticVersion,
    python_path: WPath,
    python_executable: WPath,
    pip: Pip,
}

impl PythonEnvironment {
    pub fn new(base_dir: &WPath, version: &SemanticVersion) -> Option<Self> {
        let python_path: Option<WPath> = Self::get_python_path(&base_dir, &version);

        if let Some(python_path) = python_path {
            let python_executable = python_path.join("python.exe");
            let pip: Option<Pip> = Pip::new(&python_executable);
            if let Some(pip) = pip {
                let python_version: SemanticVersion = version.clone();
                let environment: PythonEnvironment = PythonEnvironment {
                    python_version,
                    python_path,
                    python_executable,
                    pip,
                };
                return Some(environment);
            }
        } else {
            let terminal: Terminal = Terminal::new();
            let version_string: String = version.get_2p_string();
            let string: String = format!("Unable to retrieve Python {}.", version_string);
            terminal.writeln_color(&string, RedANSI);
        }
        None
    }

    pub fn from_custom_path(
        python_path: &WPath,
        python_executable: &WPath,
        version: &SemanticVersion,
    ) -> Option<PythonEnvironment> {
        let python_path: WPath = python_path.clone();
        let python_executable: WPath = python_executable.clone();
        let python_version: SemanticVersion = version.clone();

        let pip: Option<Pip> = Pip::new(&python_executable);
        if let Some(pip) = pip {
            let environment: PythonEnvironment = PythonEnvironment {
                python_version,
                python_path,
                python_executable,
                pip,
            };
            return Some(environment);
        }
        None
    }

    pub fn get_python_executable(&self) -> &WPath {
        &self.python_executable
    }

    pub fn get_python_version(&self) -> &SemanticVersion {
        &self.python_version
    }

    pub fn get_pip(&self) -> &Pip {
        &self.pip
    }
}

impl PythonEnvironment {
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
