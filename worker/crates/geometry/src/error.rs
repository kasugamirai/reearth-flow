#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error mismatched geometry: {0}")]
    MismatchedGeometry(String),

    #[error("Error projection: {0}")]
    Projection(String),
}

impl Error {
    pub fn mismatched_geometry<T: ToString>(message: T) -> Self {
        Self::MismatchedGeometry(message.to_string())
    }

    pub fn projection<T: ToString>(message: T) -> Self {
        Self::Projection(message.to_string())
    }
}

// implement Eq and PartialEq for Error so that we can compare errors in tests
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::MismatchedGeometry(a), Self::MismatchedGeometry(b)) => a == b,
            (Self::Projection(a), Self::Projection(b)) => a == b,
            _ => false,
        }
    }
}
