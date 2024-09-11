use std::path::PathBuf;

use dirs::config_dir;
use thiserror::Error;

pub struct Paths {
    base_path: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self, NewPathsError> {
        let config_dir = config_dir().ok_or(NewPathsError::NotFound)?;
        let base_path = config_dir.join("atlascli");

        Ok(Self { base_path })
    }

    pub fn profile_path(&self) -> PathBuf {
        self.base_path.join("config.toml")
    }
}

#[derive(Error, Debug)]
pub enum NewPathsError {
    #[error("Failed to find user config directory")]
    NotFound,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_profile_path() {
        let paths = Paths {
            base_path: PathBuf::from_str("/home/user/.config/atlascli").unwrap(),
        };

        let expected = PathBuf::from_str("/home/user/.config/atlascli/config.toml").unwrap();
        let actual = paths.profile_path();

        assert_eq!(expected, actual);
    }
}
