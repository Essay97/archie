use std::{
    ops::{FromResidual, Try},
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
            Self::IO(err) => format!("{}", *err),
            Self::Serialization(err) => format!("{}", *err),
            Self::TemplateNotFound(name) => "could not find template ".to_owned() + name,
        };
        write!(f, "MY STRING Error: {}", detail)
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

impl FromResidual for Exit {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        todo!()
    }
}

impl Try for Exit {
    type Output = ();

    type Residual = Result<!>;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        todo!()
    }
}
