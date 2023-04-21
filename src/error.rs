#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Serialization(serde_yaml::Error),
    TemplateNotFound(String), // pass the template name
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
