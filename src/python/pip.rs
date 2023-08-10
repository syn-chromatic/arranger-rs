use crate::general::path::WPath;
use crate::general::shell::{CommandExecute, CommandResponse};
use crate::general::version::SemanticVersion;
use crate::python::python::PythonEnvironment;

use crate::general::terminal::RedANSI;
use crate::general::terminal::Terminal;

#[derive(Clone)]
pub struct Pip {
    pip_version: SemanticVersion,
}

impl Pip {
    pub fn new(python_executable: &WPath) -> Option<Self> {
        let pip_version: Option<SemanticVersion> = Self::parse_pip_version(&python_executable);
        if let Some(pip_version) = pip_version {
            let pip: Pip = Pip { pip_version };
            return Some(pip);
        }
        let terminal: Terminal = Terminal::new();
        let string: &str = "Unable to retrieve Pip version.";
        terminal.writeln_color(&string, RedANSI);
        None
    }

    pub fn get_pip_version(&self) -> &SemanticVersion {
        &self.pip_version
    }

    pub fn find_package(&self, environment: &PythonEnvironment, package: &str) -> Option<PipShow> {
        let python_executable: &WPath = environment.get_python_executable();
        let args: Vec<&str> = self.get_find_package_args(package);
        let command: CommandExecute = CommandExecute::new();
        let response: Option<CommandResponse> = command.execute_command(&python_executable, &args);

        if let Some(response) = response {
            let mut pip_show = PipShow::new();
            pip_show.parse(response.get_stdout());
            return Some(pip_show);
        }

        None
    }

    pub fn install_package(&self, environment: &PythonEnvironment, package: &str) -> bool {
        let python_executable: &WPath = environment.get_python_executable();
        let args: Vec<&str> = self.get_install_package_args(package);

        let command: CommandExecute = CommandExecute::new();
        let response: Option<CommandResponse> = command.execute_command(&python_executable, &args);
        if let Some(response) = response {
            response.print();
            if response.get_status().success() {
                return true;
            }
        }
        false
    }
}

impl Pip {
    fn get_install_package_args<'a>(&self, package: &'a str) -> Vec<&'a str> {
        let mut args: Vec<&str> = vec![
            "-m",
            "pip",
            "install",
            package,
            "--disable-pip-version-check",
        ];

        if self.pip_version.get_major() > 9 {
            args.push("--no-warn-script-location");
        }
        args
    }

    fn get_find_package_args<'a>(&self, package: &'a str) -> Vec<&'a str> {
        let args: Vec<&str> = vec!["-m", "pip", "show", package, "--disable-pip-version-check"];
        args
    }

    fn parse_pip_version(python_executable: &WPath) -> Option<SemanticVersion> {
        let args: [&str; 3] = ["-m", "pip", "--version"];
        let command: CommandExecute = CommandExecute::new();
        let response: Option<CommandResponse> = command.execute_command(&python_executable, &args);

        if let Some(response) = response {
            let stdout: &str = response.get_stdout();
            let mut chars: Vec<char> = Vec::new();
            if stdout.starts_with("pip") {
                for char in stdout.chars() {
                    if char.is_numeric() & chars.is_empty() {
                        chars.push(char);
                    } else if char.is_whitespace() & !chars.is_empty() {
                        break;
                    } else if !chars.is_empty() {
                        chars.push(char)
                    }
                }
            }

            if !chars.is_empty() {
                let version_string: String = chars.into_iter().collect();
                let version: Option<SemanticVersion> =
                    SemanticVersion::from_string(&version_string);
                return version;
            }
        }
        None
    }
}

pub struct PipShow {
    name: String,
    version: String,
    summary: String,
    homepage: String,
    author: String,
    author_email: String,
    license: String,
    location: String,
    requires: String,
    required_by: String,
}

impl PipShow {
    pub fn new() -> Self {
        let name: String = String::new();
        let version: String = String::new();
        let summary: String = String::new();
        let homepage: String = String::new();
        let author: String = String::new();
        let author_email: String = String::new();
        let license: String = String::new();
        let location: String = String::new();
        let requires: String = String::new();
        let required_by: String = String::new();

        PipShow {
            name,
            version,
            summary,
            homepage,
            author,
            author_email,
            license,
            location,
            requires,
            required_by,
        }
    }

    pub fn parse(&mut self, stdout: &str) {
        for line in stdout.split("\n") {
            let components: Option<(&str, &str)> = line.split_once(":");
            if let Some(components) = components {
                let prefix: &str = components.0.trim();
                let suffix: &str = components.1.trim();

                self.auto_parse_set(prefix, suffix);
            }
        }
    }

    pub fn get_name(&self) -> Option<&str> {
        if self.name.is_empty() {
            return None;
        }
        Some(&self.name)
    }
}

impl PipShow {
    fn auto_parse_set(&mut self, prefix: &str, suffix: &str) {
        let prefix: String = prefix.to_lowercase();
        let suffix: String = suffix.to_string();

        match prefix.as_ref() {
            "name" => self.name = suffix,
            "version" => self.version = suffix,
            "summary" => self.summary = suffix,
            "home-page" => self.homepage = suffix,
            "author" => self.author = suffix,
            "author-email" => self.author_email = suffix,
            "license" => self.license = suffix,
            "location" => self.location = suffix,
            "requires" => self.requires = suffix,
            "required-by" => self.required_by = suffix,
            _ => {}
        }
    }
}

pub struct PipPackage {
    name: String,
    version: SemanticVersion,
}

impl PipPackage {
    pub fn new(name: String, version: SemanticVersion) -> Self {
        PipPackage { name, version }
    }

    pub fn get_string(&self) -> String {
        let version_string: String = self.version.get_string();
        let string: String = format!("Name: {} | Version: {}", self.name, version_string);
        string
    }

    pub fn get_requirement_string(&self) -> String {
        let version_string: String = self.version.get_string();
        let string: String = format!("{}=={}", self.name, version_string);
        string
    }
}

pub struct PipPackageParser {
    packages: Vec<PipPackage>,
}

impl PipPackageParser {
    pub fn new() -> Self {
        let packages: Vec<PipPackage> = Vec::new();
        PipPackageParser { packages }
    }

    pub fn parse(&mut self, string: &str) {
        if string.ends_with("dist-info") {
            let string: &str = string.trim_end_matches("dist-info");
            let parts: Vec<&str> = string.split("-").collect();
            let name: String = parts[0].to_string();
            let version: Option<SemanticVersion> = SemanticVersion::from_string(parts[1]);

            if let Some(version) = version {
                let package: PipPackage = PipPackage::new(name, version);
                self.packages.push(package);
            }
        }
    }

    pub fn get_packages(&self) -> &Vec<PipPackage> {
        &self.packages
    }
}
