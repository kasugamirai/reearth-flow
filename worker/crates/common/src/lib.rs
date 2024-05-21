#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ColorUtilError: {0}")]
    Color(String),

    #[error("CSVUtilError: {0}")]
    Csv(String),

    #[error("FSError: {0}")]
    Fs(String),

    #[error("SerdeUtilError: {0}")]
    Serde(String),

    #[error("URIUtilError: {0}")]
    Uri(String),

    #[error("XMLUtilError: {0}")]
    Xml(String),

    #[error("DirError: {0}")]
    Dir(String),
}

impl Error {
    pub fn color<T: ToString>(message: T) -> Self {
        Self::Color(message.to_string())
    }

    pub fn csv<T: ToString>(message: T) -> Self {
        Self::Csv(message.to_string())
    }

    pub fn fs<T: ToString>(message: T) -> Self {
        Self::Fs(message.to_string())
    }

    pub fn serde<T: ToString>(message: T) -> Self {
        Self::Serde(message.to_string())
    }

    pub fn uri<T: ToString>(message: T) -> Self {
        Self::Uri(message.to_string())
    }

    pub fn xml<T: ToString>(message: T) -> Self {
        Self::Xml(message.to_string())
    }

    pub fn dir<T: ToString>(message: T) -> Self {
        Self::Dir(message.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod collection;
pub mod color;
pub mod csv;
pub mod dir;
pub mod fs;
pub mod serde;
pub mod str;
pub mod uri;
pub mod xml;
