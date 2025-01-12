use url::Url;

#[derive(Debug)]
pub enum TryFromMapError {
    MissingField(String),
    NoValuesInField(String),
    ParseError { field: String, value: String },
}

impl std::fmt::Display for TryFromMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TryFromMapError::MissingField(field) => write!(f, "Missing field: {}", field),
            TryFromMapError::NoValuesInField(field) => write!(f, "Field '{}' exists but contains no values", field),
            TryFromMapError::ParseError { field, value } => {
                write!(f, "Failed to parse field '{}' with value '{}'", field, value)
            }
        }
    }
}

impl std::error::Error for TryFromMapError {}

pub trait AsUrl {
    fn as_url(&self, base_url: &str) -> Result<Url, url::ParseError>;
}
