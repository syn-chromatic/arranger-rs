use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::fs::ReadDir;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

use crate::general::path::WPath;
use crate::general::shell::{CommandExecute, CommandResponse};
use crate::general::version::SemanticVersion;
use crate::python::python::PythonEnvironment;

use crate::terminal::RedANSI;
use crate::terminal::Terminal;

#[derive(Clone)]
pub struct Pip {
    pip_version: SemanticVersion,
}

impl Pip {
    pub fn new(python_executable: &WPath) -> Option<Self> {
        let pip_version: Result<SemanticVersion, io::Error> =
            Self::parse_pip_version(&python_executable);

        if let Ok(pip_version) = pip_version {
            let pip: Pip = Pip { pip_version };
            return Some(pip);
        }
        let error: io::Error = pip_version.unwrap_err();
        let terminal: Terminal = Terminal::new();
        let string: &str = "Found Python, but was unable to retrieve Pip version.\n";
        terminal.writeln_ansi(&string, &RedANSI);
        terminal.writeln_ansi(&error.to_string(), &RedANSI);
        None
    }

    #[allow(dead_code)]
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

    fn parse_pip_version(python_executable: &WPath) -> Result<SemanticVersion, io::Error> {
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
                if let Some(version) = version {
                    return Ok(version);
                }
            }

            let stderr: &str = response.get_stderr();
            if !stderr.is_empty() {
                let error_string: String = format!("Pip Error: {}", stderr);
                let error: io::Error = io::Error::new(io::ErrorKind::Other, error_string);
                return Err(error);
            }
        }
        let error_string: &str = "Error: Failed to get response from Pip.";
        let error: io::Error = io::Error::new(io::ErrorKind::Other, error_string);
        Err(error)
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

pub struct PipMetadata {
    package: PipPackage,
    requires: HashMap<String, HashSet<PipPackageName>>,
}

impl PipMetadata {
    pub fn new(package: &PipPackage, requires: &HashMap<String, HashSet<PipPackageName>>) -> Self {
        let package: PipPackage = package.clone();
        let requires: HashMap<String, HashSet<PipPackageName>> = requires.clone();

        PipMetadata { package, requires }
    }

    #[allow(dead_code)]
    pub fn get_package(&self) -> &PipPackage {
        &self.package
    }

    pub fn get_requires(&self) -> &HashMap<String, HashSet<PipPackageName>> {
        &self.requires
    }
}

struct PipMetadataParser;

impl PipMetadataParser {
    pub fn new() -> Self {
        PipMetadataParser
    }

    fn parse_requires(
        &self,
        reader: io::BufReader<File>,
    ) -> HashMap<String, HashSet<PipPackageName>> {
        let mut flag_name: String = "base".to_string();
        let mut requires: HashMap<String, HashSet<PipPackageName>> = HashMap::new();
        let mut package_names: HashSet<PipPackageName> = HashSet::new();
        let mut requires_begin: bool = false;
        let requires_dist: &str = "Requires-Dist:";
        let provides_extra: &str = "Provides-Extra:";

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with(provides_extra) && !requires_begin {
                    flag_name = self.parse_flag_name(&line, provides_extra);
                } else if line.starts_with(&provides_extra) && requires_begin {
                    requires.insert(flag_name.to_string(), package_names.clone());
                    package_names.clear();
                    flag_name = self.parse_flag_name(&line, provides_extra);
                } else if line.starts_with(requires_dist) {
                    requires_begin = true;
                    let name: PipPackageName = self.parse_requires_name(&line, requires_dist);
                    package_names.insert(name);
                } else if requires_begin && line.is_empty() {
                    requires.insert(flag_name.to_string(), package_names.clone());
                    package_names.clear();
                    break;
                }
            }
        }

        if !package_names.is_empty() {
            requires.insert(flag_name.to_string(), package_names.clone());
        }

        requires
    }
}

impl PipMetadataParser {
    fn parse_flag_name(&self, line: &str, provides_extra: &str) -> String {
        let line: &str = line.trim_start_matches(provides_extra);
        let line: &str = line.trim();
        let provides_name: String = line.to_string();
        provides_name
    }

    fn parse_requires_name(&self, line: &str, requires_dist: &str) -> PipPackageName {
        let line: &str = line.trim_start_matches(requires_dist);
        let line: &str = line.trim();
        let mut chars: Vec<char> = Vec::new();
        for character in line.chars() {
            if character.is_whitespace() {
                break;
            }
            chars.push(character);
        }
        let string: String = chars.into_iter().collect();
        let name: PipPackageName = PipPackageName::new(&string);
        name
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct PipPackageName {
    name: String,
}
impl PipPackageName {
    pub fn new(name: &str) -> Self {
        let name: String = name.replace("_", "-").to_lowercase();
        PipPackageName { name }
    }

    pub fn get_string(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
pub struct PipPackage {
    name: PipPackageName,
    version: SemanticVersion,
    path: WPath,
}

impl PipPackage {
    pub fn new(name: PipPackageName, version: SemanticVersion, path: WPath) -> Self {
        PipPackage {
            name,
            version,
            path,
        }
    }

    pub fn get_name(&self) -> &PipPackageName {
        &self.name
    }

    #[allow(dead_code)]
    pub fn get_name_string(&self) -> &str {
        self.name.get_string()
    }

    #[allow(dead_code)]
    pub fn get_version(&self) -> &SemanticVersion {
        &self.version
    }

    #[allow(dead_code)]
    pub fn get_path(&self) -> &WPath {
        &self.path
    }

    pub fn get_string(&self) -> String {
        let version_string: String = self.version.get_string();
        let name: &str = self.name.get_string();
        let string: String = format!("Name: {} | Version: {}", name, version_string);
        string
    }

    pub fn get_requirement_string(&self) -> String {
        let version_string: String = self.version.get_string();
        let name: &str = self.name.get_string();
        let string: String = format!("{}=={}", name, version_string);
        string
    }
}

impl Debug for PipPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipPackage")
            .field("name", &self.name.get_string())
            .field("version", &self.version)
            .field("path", &self.path)
            .finish()
    }
}

pub struct PipPackageParser {
    packages_dir: WPath,
}

impl PipPackageParser {
    pub fn new(packages_dir: &WPath) -> Self {
        let packages_dir = packages_dir.clone();
        PipPackageParser { packages_dir }
    }

    pub fn get_packages(&self) -> Result<Vec<PipPackage>, io::Error> {
        let read_dir: ReadDir = self.packages_dir.read_dir()?;
        let mut packages: Vec<PipPackage> = Vec::new();
        for entry in read_dir {
            if let Ok(entry) = entry {
                let entry_path: PathBuf = entry.path();
                if entry_path.is_dir() {
                    let entry_name: Option<&str> =
                        entry_path.file_name().unwrap_or_default().to_str();

                    if let Some(entry_name) = entry_name {
                        self.parse_dir_name(&entry_path, entry_name, &mut packages);
                    }
                }
            }
        }
        Ok(packages)
    }

    pub fn get_metadata(&self, packages: &Vec<PipPackage>) -> Vec<PipMetadata> {
        let mut metadata: Vec<PipMetadata> = Vec::new();

        for package in packages {
            let metadata_path: &WPath = &package.path.join("METADATA");
            if metadata_path.exists() {
                let file: Result<File, io::Error> = File::open(&metadata_path);
                if let Ok(file) = file {
                    let reader: io::BufReader<File> = io::BufReader::new(file);
                    let metadata_parser: PipMetadataParser = PipMetadataParser::new();
                    let requires: HashMap<String, HashSet<PipPackageName>> =
                        metadata_parser.parse_requires(reader);

                    let pip_metadata: PipMetadata = PipMetadata::new(package, &requires);
                    metadata.push(pip_metadata);
                }
            }
        }

        metadata
    }

    pub fn distill_packages(&self, packages: &mut Vec<PipPackage>, metadata: &Vec<PipMetadata>) {
        for meta in metadata {
            let requires: &HashMap<String, HashSet<PipPackageName>> = meta.get_requires();
            let base: Option<&HashSet<PipPackageName>> = requires.get("base");

            if let Some(base) = base {
                self.distill_from_base_dependencies(base, packages);
            }
        }
    }
}

impl PipPackageParser {
    fn distill_from_base_dependencies(
        &self,
        base: &HashSet<PipPackageName>,
        packages: &mut Vec<PipPackage>,
    ) {
        let mut to_remove: Vec<usize> = Vec::new();
        for (idx, package) in packages.iter().enumerate() {
            if !package.version.qualifier.is_empty() {
                continue;
            } else if base.contains(package.get_name()) {
                to_remove.push(idx);
            }
        }

        let mut shift = 0;
        for idx in to_remove.iter() {
            packages.remove(*idx - shift);
            shift += 1
        }
    }

    fn parse_dir_name(
        &self,
        entry_path: &PathBuf,
        entry_name: &str,
        packages: &mut Vec<PipPackage>,
    ) {
        if entry_name.ends_with(".dist-info") {
            let string: &str = entry_name.trim_end_matches(".dist-info");
            let parts: Vec<&str> = string.split("-").collect();
            let name: PipPackageName = PipPackageName::new(parts[0]);
            let version: Option<SemanticVersion> = SemanticVersion::from_string(parts[1]);

            if let Some(version) = version {
                let path: WPath = entry_path.into();
                let package: PipPackage = PipPackage::new(name, version, path);
                packages.push(package);
            }
        }
    }
}
