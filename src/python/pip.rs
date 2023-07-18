use std::path::PathBuf;

use crate::python::python::PythonEnvironment;
use crate::shell::utils::{CommandE, CommandR};

pub struct Pip {
    environment: PythonEnvironment,
}

impl Pip {
    pub fn new(environment: &PythonEnvironment) -> Self {
        let environment: PythonEnvironment = environment.clone();
        Pip { environment }
    }

    pub fn find_package(&self, package: &str) -> Option<PipShow> {
        let python_executable: PathBuf = self.environment.get_python_executable();
        let args: [&str; 5] = ["-m", "pip", "show", package, "--disable-pip-version-check"];
        let command: CommandE = CommandE::new();
        let response: Option<CommandR> = command.execute_command(&python_executable, &args);

        if let Some(response) = response {
            let mut pip_show = PipShow::new();
            pip_show.parse(response.get_stdout());
            return Some(pip_show);
        }

        None
    }

    pub fn install_package(&self, package: &str) {
        let python_executable: PathBuf = self.environment.get_python_executable();
        let args: [&str; 5] = [
            "-m",
            "pip",
            "install",
            package,
            "--disable-pip-version-check",
        ];

        let command: CommandE = CommandE::new();
        let response: Option<CommandR> = command.execute_command(&python_executable, &args);
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

        if prefix == "name" {
            self.name = suffix;
        } else if prefix == "version" {
            self.version = suffix;
        } else if prefix == "summary" {
            self.summary = suffix;
        } else if prefix == "home-page" {
            self.homepage = suffix;
        } else if prefix == "author" {
            self.author = suffix;
        } else if prefix == "author-email" {
            self.author_email = suffix;
        } else if prefix == "license" {
            self.license = suffix;
        } else if prefix == "location" {
            self.location = suffix;
        } else if prefix == "requires" {
            self.requires = suffix;
        } else if prefix == "required-by" {
            self.required_by = suffix;
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
