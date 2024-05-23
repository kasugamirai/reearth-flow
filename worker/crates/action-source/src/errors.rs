use thiserror::Error;

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("File Reader Factory error: {0}")]
    FileReaderFactory(String),
    #[error("File Reader error: {0}")]
    FileReader(String),
    #[error("File Path Extractor Factory error: {0}")]
    FilePathExtractorFactory(String),
    #[error("File Path Extractor error: {0}")]
    FilePathExtractor(String),
}

pub type Result<T, E = SourceError> = std::result::Result<T, E>;
