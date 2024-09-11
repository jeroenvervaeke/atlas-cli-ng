use std::{collections::BTreeMap, fs::write, io, ops::Deref};

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

pub const DEFAULT_PROFILE: &str = "default";
const ENV_VARIABLE_PREFIX: &str = "ATLAS";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProfileFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongosh_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_update_check: Option<bool>,
    #[serde(flatten)]
    pub profiles: BTreeMap<String, Profile>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Profile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongosh_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_manager_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Output>,

    #[serde(flatten)]
    pub auth: Option<Auth>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Auth {
    ApiKeys(ApiKeys),
    OAuth(OAuth),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Output {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "plaintext")]
    Plaintext,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Service {
    #[serde(rename = "cloud")]
    Cloud,
    #[serde(rename = "cloudgov")]
    GovCloud,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiKeys {
    #[serde(rename = "public_api_key")]
    public: String,
    #[serde(rename = "private_api_key")]
    private: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuth {
    access_token: String,
    refresh_token: String,
}

impl ProfileFile {
    pub fn load(path: &str) -> Result<Self, ProfileFileLoadError> {
        let config = Config::builder()
            .add_source(File::with_name(path).required(false))
            .add_source(Environment::with_prefix(ENV_VARIABLE_PREFIX))
            .build()?;

        Ok(Self::from_config(config)?)
    }

    pub fn from_config(c: Config) -> Result<Self, ProfileFileFromConfigError> {
        let profile_file: ProfileFile = c
            .try_deserialize()
            .map_err(ProfileFileFromConfigError::FailedToParseConfig)?;

        Ok(profile_file)
    }

    pub fn save(&self, path: &str) -> Result<(), ProfileFileSaveError> {
        let toml = toml::to_string_pretty(self)?;
        write(path, toml)?;

        Ok(())
    }

    pub fn init_profile(&mut self, name: &str) -> bool {
        if !self.profile_exists(name) {
            self.profiles.insert(name.to_string(), Default::default());
            return true;
        }

        false
    }

    pub fn profile_exists(&self, name: &str) -> bool {
        self.profiles.contains_key(name)
    }

    pub fn profile_names(&self) -> impl Iterator<Item = &str> {
        self.profiles.keys().map(Deref::deref)
    }

    pub fn profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    pub fn profile_mut(&mut self, name: &str) -> Option<&mut Profile> {
        self.profiles.get_mut(name)
    }

    pub fn init_default_profile(&mut self) -> bool {
        self.init_profile(DEFAULT_PROFILE)
    }

    pub fn default_profile(&self) -> Option<&Profile> {
        self.profile(DEFAULT_PROFILE)
    }

    pub fn default_profile_mut(&mut self) -> Option<&mut Profile> {
        self.profile_mut(DEFAULT_PROFILE)
    }
}

#[derive(Error, Debug)]
pub enum ProfileFileSaveError {
    #[error("Serialize error")]
    SerializeError(#[from] toml::ser::Error),
    #[error("Write error")]
    WriteError(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum ProfileFileLoadError {
    #[error("Config error")]
    Config(#[from] ConfigError),
    #[error("Failed to convert config to profile")]
    ProfileFileFromConfig(#[from] ProfileFileFromConfigError),
}

#[derive(Error, Debug)]
pub enum ProfileFileFromConfigError {
    #[error("Failed to parse config")]
    FailedToParseConfig(#[source] ConfigError),
}
