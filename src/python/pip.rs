use crate::general::path::WPath;
use crate::general::shell::{CommandExecute, CommandResponse};
use crate::general::version::SemanticVersion;
use crate::python::python::PythonEnvironment;

pub struct Pip {
    environment: PythonEnvironment,
}

impl Pip {
    pub fn new(environment: &PythonEnvironment) -> Self {
        let environment: PythonEnvironment = environment.clone();
        Pip { environment }
    }

    pub fn find_package(&self, package: &str) -> Option<PipShow> {
        let python_executable: WPath = self.environment.get_python_executable();
        let args: [&str; 5] = ["-m", "pip", "show", package, "--disable-pip-version-check"];
        let command: CommandExecute = CommandExecute::new();
        let response: Option<CommandResponse> = command.execute_command(&python_executable, &args);

        if let Some(response) = response {
            let mut pip_show = PipShow::new();
            pip_show.parse(response.get_stdout());
            return Some(pip_show);
        }

        None
    }

    pub fn install_package(&self, package: &str) {
        let python_executable: WPath = self.environment.get_python_executable();
        let args: [&str; 5] = [
            "-m",
            "pip",
            "install",
            package,
            "--disable-pip-version-check",
        ];

        let command: CommandExecute = CommandExecute::new();
        let response: Option<CommandResponse> = command.execute_command(&python_executable, &args);
        if let Some(response) = response {
            println!("STDOUT: {}", response.get_stdout());
            println!("STDERR: {}", response.get_stderr());
        }
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

    pub fn get_name(&self) -> Option<&str> {
        if self.name.is_empty() {
            return None;
        }
        Some(&self.name)
    }

    pub fn get_string(&self) -> String {
        let string = format!("Name: {}\nVersion: {}\nSummary: {}\nHomepage: {}\nAuthor: {}\nAuthor Email: {}\nLicense: {}\nLocation: {}\nRequires: {}\nRequired By: {}",
    self.name, self.version, self.summary, self.homepage, self.author, self.author_email, self.license, self.location, self.requires, self.required_by);
        string
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
