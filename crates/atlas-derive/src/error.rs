#[derive(Debug)]
pub enum TryFromMapError {
    MissingField(String),
    EmptyField(String),
    ParseError { field: String, value: String },
}

impl std::fmt::Display for TryFromMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TryFromMapError::MissingField(field) => write!(f, "Missing field: {}", field),
            TryFromMapError::EmptyField(field) => write!(f, "Field is empty: {}", field),
            TryFromMapError::ParseError { field, value } => {
                write!(f, "Failed to parse field '{}' with value '{}'", field, value)
            }
        }
    }
}

impl std::error::Error for TryFromMapError {}
