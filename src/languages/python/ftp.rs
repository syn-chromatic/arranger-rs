use core::fmt::Debug;
use std::collections::HashSet;
use std::error::Error;
use std::io;

use regex::Regex;

use crate::misc::https::HTTPS;
use crate::misc::version::{PreRelease, SemanticVersion};
use crate::terminal::Terminal;
use crate::terminal::YellowANSI;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum LinkType {
    File(String),
    Directory(String),
}

impl LinkType {
    pub fn new(link: &str) -> Option<Self> {
        if link != "../" {
            let link_type = if link.ends_with('/') {
                LinkType::Directory(link.to_string())
            } else {
                LinkType::File(link.to_string())
            };
            return Some(link_type);
        }
        None
    }
}

pub struct FileStructure {
    url: String,
    structure: HashSet<LinkType>,
}

impl FileStructure {
    pub async fn new(url: &str) -> Option<Self> {
        let structure: Result<HashSet<LinkType>, Box<dyn Error>> =
            Self::build_file_structure(url).await;
        if let Ok(structure) = structure {
            let url: String = url.to_string();
            let file_structure: FileStructure = FileStructure { url, structure };
            return Some(file_structure);
        }
        None
    }

    pub async fn access_directory(&mut self, directory: &str) -> bool {
        let link_type: Option<LinkType> = LinkType::new(directory);
        if let Some(link_type) = &link_type {
            match link_type {
                LinkType::Directory(dir) => {
                    let new_url: String = format!("{}{}", self.url, dir);
                    let structure: Result<HashSet<LinkType>, Box<dyn std::error::Error>> =
                        Self::build_file_structure(&new_url).await;

                    if let Ok(structure) = structure {
                        self.url = new_url;
                        self.structure = structure;
                        return true;
                    }
                }
                LinkType::File(_) => {}
            };
        }
        return false;
    }

    #[allow(dead_code)]
    pub async fn access_file(&mut self, file: &str) -> Option<String> {
        let link_type: Option<LinkType> = LinkType::new(file);
        if let Some(link_type) = &link_type {
            match link_type {
                LinkType::File(file) => {
                    if self.structure.contains(link_type) {
                        let file_url = format!("{}{}", self.url, file);
                        return Some(file_url);
                    }
                }
                LinkType::Directory(_) => {}
            }
        }
        None
    }

    pub fn get_structure(&self) -> HashSet<LinkType> {
        self.structure.clone()
    }

    #[allow(dead_code)]
    pub fn list_structure(&self) {
        for file_type in &self.structure {
            match file_type {
                LinkType::Directory(dir) => {
                    println!("Directory: {}", dir);
                }
                LinkType::File(file) => {
                    println!("File: {}", file);
                }
            }
        }
    }

    async fn build_file_structure(url: &str) -> Result<HashSet<LinkType>, Box<dyn Error>> {
        let https: HTTPS = HTTPS::new();
        let body: String = https.get_response_body(url).await?;

        let mut links: HashSet<LinkType> = HashSet::new();
        let regex: Regex = Regex::new(r#"<a href="(.*?)""#).unwrap();

        for capture in regex.captures_iter(&body) {
            let link: &str = &capture[1];
            let link_type: Option<LinkType> = LinkType::new(link);
            if let Some(link_type) = link_type {
                links.insert(link_type);
            }
        }
        if links.is_empty() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unable to access directory.",
            )));
        }
        Ok(links)
    }
}

pub struct PythonFTPRetriever {
    ftp_url: String,
}

impl PythonFTPRetriever {
    pub fn new() -> Self {
        let ftp_url = "https://www.python.org/ftp/python/".to_string();
        PythonFTPRetriever { ftp_url }
    }

    pub async fn get_setup_file_latest_patch(
        &self,
        version: &mut SemanticVersion,
        arch: &str,
        platform: &str,
        package_type: &str,
    ) -> Option<String> {
        let terminal: Terminal = Terminal::new();
        let mut setup_file: Option<String> = None;
        let mut counter: usize = 0;
        let limit: usize = 50;
        version.set_patch(0);

        while counter <= limit {
            counter += 1;
            let file: Option<String> = self
                .get_setup_file(&version, arch, platform, package_type)
                .await;

            if let Some(_) = file {
                setup_file = file;
            } else if version.get_patch() == 0 {
            } else {
                let patch: usize = version.get_patch();
                version.set_patch(patch - 1);
                break;
            }

            let parts: [&str; 2] = ["\rVersion: ", &version.get_string()];
            terminal.write_parameter(&parts, &YellowANSI);

            let patch: usize = version.get_patch();
            version.set_patch(patch + 1);
        }
        println!();
        setup_file
    }

    pub async fn get_setup_file(
        &self,
        version: &SemanticVersion,
        arch: &str,
        platform: &str,
        package_type: &str,
    ) -> Option<String> {
        let version_3p_directory: String = self.get_3p_version_directory(version);
        let file_structure: Option<FileStructure> = FileStructure::new(&self.ftp_url).await;

        if let Some(mut file_structure) = file_structure {
            let mut result: bool = file_structure.access_directory(&version_3p_directory).await;
            if !result && version.get_patch() == 0 {
                let version_2p_directory: String = self.get_2p_version_directory(version);
                result = file_structure.access_directory(&version_2p_directory).await;
            }

            if result {
                let structure: HashSet<LinkType> = file_structure.get_structure();

                let setup_file: Option<String> =
                    self.find_setup_file(&structure, arch, platform, package_type);
                if let Some(setup_file) = setup_file {
                    let url: String = format!("{}{}", file_structure.url, setup_file);
                    return Some(url);
                }
            }
        }
        None
    }

    pub async fn list_file_structure(&self, version: &SemanticVersion) {
        let version_3p_directory: String = self.get_3p_version_directory(version);
        let file_structure: Option<FileStructure> = FileStructure::new(&self.ftp_url).await;

        if let Some(mut file_structure) = file_structure {
            let mut result: bool = file_structure.access_directory(&version_3p_directory).await;
            if !result && version.get_patch() == 0 {
                let version_2p_directory: String = self.get_2p_version_directory(version);
                result = file_structure.access_directory(&version_2p_directory).await;
            }

            if result {
                let structure: HashSet<LinkType> = file_structure.get_structure();
                for link in structure {
                    match link {
                        LinkType::File(file) => {
                            let filename: Option<PythonFilename> = PythonFilename::new(&file);
                            if let Some(filename) = filename {
                                let name: String = filename.name;
                                let version: String = filename.version.get_string();
                                let platform: String = filename.platform;
                                let architecture: String = filename.architecture;
                                let package_type: String = filename.package_type;
                                let fmt_str: String = format!(
                                    "Name: {} | Ver: {} | Platform: {} | Arch: {} | Type: {}",
                                    name, version, platform, architecture, package_type
                                );
                                println!("{}", fmt_str);
                            }
                        }
                        LinkType::Directory(_) => {}
                    }
                }
            }
        }
    }

    fn get_3p_version_directory(&self, version: &SemanticVersion) -> String {
        let (major, minor, patch): (usize, usize, usize) = version.get_3p_version();
        let version_directory: String = format!("{}.{}.{}/", major, minor, patch);
        version_directory
    }

    fn get_2p_version_directory(&self, version: &SemanticVersion) -> String {
        let (major, minor): (usize, usize) = version.get_2p_version();
        let version_directory: String = format!("{}.{}/", major, minor);
        version_directory
    }

    fn find_setup_file(
        &self,
        structure: &HashSet<LinkType>,
        arch: &str,
        platform: &str,
        package_type: &str,
    ) -> Option<String> {
        let mut setup_file: Option<String> = None;
        let mut python_filename: Option<PythonFilename> = None;

        for link in structure {
            match link {
                LinkType::File(file) => {
                    let filename: Option<PythonFilename> = PythonFilename::new(file);
                    if let Some(filename) = filename {
                        let requirement: bool = filename.match_requirements(
                            "python",
                            arch,
                            package_type,
                            platform,
                            &["exe", "msi", "pkg", "dmg", "tgz"],
                        );
                        if requirement {
                            if let Some(_python_filename) = &python_filename {
                                let version: &SemanticVersion = &filename.version;
                                let prev_version: &SemanticVersion = &_python_filename.version;

                                let pre_release: &Option<PreRelease> = version.get_pre_release();
                                let prev_pre_release: &Option<PreRelease> =
                                    prev_version.get_pre_release();

                                if let Some(pre_release) = pre_release {
                                    if let Some(prev_pre_release) = prev_pre_release {
                                        if pre_release < prev_pre_release {
                                            continue;
                                        }
                                    } else {
                                        continue;
                                    }
                                }
                                setup_file = Some(file.to_string());
                                python_filename = Some(filename);
                            } else {
                                setup_file = Some(file.to_string());
                                python_filename = Some(filename);
                            }
                        }
                    }
                }
                LinkType::Directory(_) => {}
            }
        }
        setup_file
    }
}

#[derive(Clone)]
pub struct PythonFilename {
    name: String,
    version: SemanticVersion,
    architecture: String,
    package_type: String,
    platform: String,
    extension: String,
}

impl PythonFilename {
    pub fn new(filename: &str) -> Option<Self> {
        let parts: Vec<&str> = filename.rsplitn(2, '.').collect();

        if parts.len() == 2 {
            let (main_part, extension) = (parts[1], parts[0]);

            if Self::is_valid_extension(extension) {
                let (name, version, architecture, package_type, platform): (
                    Option<String>,
                    Option<SemanticVersion>,
                    String,
                    String,
                    Option<String>,
                ) = Self::get_components(main_part, extension);

                if let (Some(name), Some(version), Some(platform)) = (name, version, platform) {
                    let extension: String = extension.to_string();
                    return Some(PythonFilename {
                        name,
                        version,
                        architecture,
                        package_type,
                        platform,
                        extension,
                    });
                }
            }
        }
        None
    }

    pub fn match_requirements(
        &self,
        name: &str,
        architecture: &str,
        package_type: &str,
        platform: &str,
        extension: &[&str],
    ) -> bool {
        if self.name == name
            && self.architecture == architecture
            && self.platform == platform
            && self.package_type == package_type
            && extension.contains(&self.extension.as_ref())
        {
            return true;
        }
        false
    }

    fn is_name(segment: &str) -> bool {
        if segment == "python" {
            return true;
        }
        false
    }

    fn is_version(segment: &str) -> Option<SemanticVersion> {
        let version: Option<SemanticVersion> = SemanticVersion::from_string(segment);
        version
    }

    fn is_architecture(segment: &str) -> bool {
        match segment {
            "amd64" => return true,
            "arm64" => return true,
            "ia64" => return true,
            _ => return false,
        }
    }

    fn is_package_type(segment: &str, extension: &str) -> Option<String> {
        if extension == "tgz" {
            return Some("source".to_string());
        };

        let segment: String = segment.to_string();
        match segment.as_ref() {
            "embed" => return Some(segment),
            "webinstall" => return Some(segment),
            _ => return None,
        }
    }

    fn is_platform(segment: &str, extension: &str) -> Option<String> {
        if ["exe", "msi"].contains(&extension) || segment == "win32" {
            return Some("windows".to_string());
        }

        if segment.starts_with("macos") || ["pkg", "dmg"].contains(&extension) {
            return Some("macos".to_string());
        }

        if extension == "tgz" {
            return Some("any".to_string());
        }
        None
    }

    fn is_valid_extension(extension: &str) -> bool {
        match extension {
            "msi" => true,
            "exe" => true,
            "zip" => true,
            "pkg" => true,
            "dmg" => true,
            "tgz" => true,
            _ => false,
        }
    }

    fn get_components(
        main_part: &str,
        extension: &str,
    ) -> (
        Option<String>,
        Option<SemanticVersion>,
        String,
        String,
        Option<String>,
    ) {
        let mut name: Option<String> = None;
        let mut version: Option<SemanticVersion> = None;
        let mut architecture: String = "n/a".to_string();
        let mut package_type: String = "standard".to_string();
        let mut platform: Option<String> = None;

        let mut split: Vec<String> = main_part.split('-').map(|s| s.to_string()).collect();
        Self::process_split(&mut split);

        for segment in split {
            let segment: String = segment.to_lowercase();
            let is_name: bool = Self::is_name(&segment);
            let is_version: Option<SemanticVersion> = Self::is_version(&segment);
            let is_architecture: bool = Self::is_architecture(&segment);
            let is_package_type: Option<String> = Self::is_package_type(&segment, extension);
            let is_platform: Option<String> = Self::is_platform(&segment, extension);

            if is_name {
                let segment: String = segment.clone();
                name = Some(segment);
            }
            if is_version.is_some() {
                version = Some(is_version.unwrap());
            }
            if is_architecture {
                let segment: String = segment.clone();
                architecture = segment;
            }
            if is_package_type.is_some() {
                package_type = is_package_type.unwrap();
            }
            if is_platform.is_some() {
                platform = Some(is_platform.unwrap());
            }
        }

        (name, version, architecture, package_type, platform)
    }

    fn process_split(split: &mut Vec<String>) {
        let second_part: &String = &split[1];
        let mut version_part: Vec<char> = Vec::new();
        let mut version_segments: usize = 0;
        let mut filled_segment: bool = false;
        let mut p_numeric: bool = false;
        let mut p_separate: bool = false;
        let mut split_idx: usize = 0;

        for (idx, character) in second_part.chars().enumerate() {
            if character.is_numeric() && !(idx == second_part.len() - 1) {
                version_part.push(character);
                p_numeric = true;
                p_separate = false;
                filled_segment = true;
            } else if ['a', 'b', 'c'].contains(&character)
                && !(idx == second_part.len() - 1)
                && filled_segment
            {
                version_part.push(character);
                p_numeric = false;
                p_separate = false;
                filled_segment = false;
            } else if character == '.' && version_segments == 2 && p_numeric && filled_segment {
                split_idx = idx;
                break;
            } else if (!p_numeric && !p_separate) || (idx == second_part.len() - 1) {
                version_part.clear();
                break;
            } else if character == '.' && p_numeric {
                version_part.push(character);
                p_numeric = false;
                p_separate = true;
                version_segments += 1;
                filled_segment = false;
            } else {
                p_numeric = false;
                p_separate = false;
            }
        }
        if !version_part.is_empty() {
            let remaining_part: String = second_part.split_at(split_idx + 1).1.to_string();
            let version_string: String = version_part.into_iter().collect();
            split.remove(1);
            split.insert(1, version_string);
            split.insert(2, remaining_part);
        }
    }
}

impl Debug for PythonFilename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PythonFilename")
            .field("name", &self.name)
            .field("version", &self.version)
            .field("architecture", &self.architecture)
            .field("package_type", &self.package_type)
            .field("platform", &self.platform)
            .field("extension", &self.extension)
            .finish()
    }
}
