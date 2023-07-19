mod python;
mod shell;

use std::path::PathBuf;

use dirs;
use structopt;
use structopt::StructOpt;

use crate::python::packages::VirtualEnv;
use crate::python::python::{PythonEnvironment, PythonVersion};

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

#[derive(Debug)]
enum LanguageEnum {
    Python,
    Invalid,
}

impl std::str::FromStr for LanguageEnum {
    type Err = LanguageEnum;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == "python" {
            return Ok(LanguageEnum::Python);
        }
        Err(LanguageEnum::Invalid)
    }
}

impl std::fmt::Display for LanguageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LanguageEnum::Python => write!(f, "Python"),
            LanguageEnum::Invalid => write!(f, ""),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Arranger", version = "0.1", about = "Python")]
struct PythonOpt {
    /// Set a major number
    #[structopt(short = "M", long = "major")]
    major: usize,

    /// Set a minor number
    #[structopt(short = "m", long = "minor")]
    minor: usize,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Arranger", version = "0.1", about = "Languages")]
struct LanguageOpt {
    /// Select a language.
    #[structopt(short = "L", long = "language")]
    language: LanguageEnum,
}

fn main() {
    let language_opt = LanguageOpt::from_args();
    println!("language: {:?}", language_opt);

    match language_opt.language {
        LanguageEnum::Python => {
            let python_opt = PythonOpt::from_args();
            println!("python: {:?}", python_opt);
        }
        LanguageEnum::Invalid => {}
    }
    // if language_opt.language == LanguageEnum::Python {

    // }

    // let opt: PythonOpt = PythonOpt::from_args();
    // println!("Major: {}, Minor: {}", opt.major, opt.minor);
    // create_virtual_env(opt.major, opt.minor);
}
