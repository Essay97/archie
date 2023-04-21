use std::{
    path::PathBuf,
    process::{ExitCode, Termination},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Serialization(serde_yaml::Error),
    /// Argument is the name of the template that cannot be found
    TemplateNotFound(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let detail = match self {
            Self::IO(err) => "could not open configuration file".to_string(),
            Self::Serialization(_) => {
                "the configuration file contains an error. Please look into it.\nIf you are unsure about which config file you are using, run `archie list`".to_string()
            }
            Self::TemplateNotFound(name) => "could not find template ".to_owned() + name,
        };
        write!(f, "\n\nError: {}\n\n", detail)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        Self::Serialization(value)
    }
}

pub enum Exit {
    Ok,
    Err(Error, bool),
}

impl Termination for Exit {
    fn report(self) -> ExitCode {
        match self {
            Exit::Ok => ExitCode::SUCCESS,
            Exit::Err(e, debug) => {
                if debug {
                    eprintln!("{:?}", e);
                } else {
                    eprintln!("{}", e);
                }

                ExitCode::FAILURE
            }
        }
    }
}

impl Exit {
    pub fn from<T>(value: Result<T>, debug: bool) -> Self {
        match value {
            Ok(_) => Exit::Ok,
            Err(e) => Exit::Err(e, debug),
        }
    }
}
