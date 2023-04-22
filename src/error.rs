use std::{
    path::PathBuf,
    process::{ExitCode, Termination},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Argument is the name of the template that cannot be found
    TemplateNotFound(String),
    /// Argument is the absolute path of the folder that already exists
    RootFolderExistent(PathBuf),
    /// Argument is the absolute path that does not exist
    PathNotExistent(PathBuf),
    CurrentDirectoryUnavailable,
    /// Argument is the absolute path of the non accessible file
    FileNotAccessible(PathBuf),
    NoHomeFolder,
    /// A file contains non UTF-8 characters
    WrongFileEncoding,
    OnDeserialize,
    /// Error while reading user input from stdin
    OnInput,
    /// Argument is some kind of path (at least folder name) of the folder that could not be created
    OnCreateFolder(PathBuf),
    /// Returns a fake error in order to exit with code 1 without printing a message
    Dummy,
    /// Argument is the absolute path of the folder you are trying to change to
    OnChangeFolder(PathBuf),
    /// Argument is some kind of path (at least file name) of the file that could not be created
    OnCreateFile(PathBuf),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let detail = match self {
            Self::TemplateNotFound(name) => "could not find template ".to_owned() + name,
            Self::RootFolderExistent(p) => format!("root folder {} already exists", p.display()),
            Self::PathNotExistent(p) => format!("path {} does not exist", p.display()),
            Self::CurrentDirectoryUnavailable => "could not access current directory".to_owned(),
            Self::FileNotAccessible(f) => format!("could not open file {}", f.display()),
            Self::NoHomeFolder => "could not access home directory".to_owned(),
            Self::WrongFileEncoding => {
                "reading from a file that contains non UTF-8 characters".to_owned()
            }
            Self::OnDeserialize => "config file has the wrong format".to_owned(),
            Self::OnInput => "a problem occurred while reading user input".to_owned(),
            Self::OnCreateFolder(p) => format!("could not create folder {}", p.display()),
            Self::Dummy => String::new(),
            Self::OnChangeFolder(p) => format!("could not change to {} folder", p.display()),
            Self::OnCreateFile(f) => format!("could not create file {}", f.display()),
        };
        write!(f, "\nError: {}\n", detail)
    }
}

impl std::error::Error for Error {}

pub enum Exit {
    Ok,
    Err(Error),
}

impl Termination for Exit {
    fn report(self) -> ExitCode {
        match self {
            Exit::Ok => ExitCode::SUCCESS,
            Exit::Err(e) => {
                eprintln!("{}", e);
                ExitCode::FAILURE
            }
        }
    }
}

impl From<Result<()>> for Exit {
    fn from(value: Result<()>) -> Self {
        match value {
            Ok(_) => Exit::Ok,
            Err(e) => Exit::Err(e),
        }
    }
}
