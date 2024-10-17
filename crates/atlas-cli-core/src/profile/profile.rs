use serde::Deserialize;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tokio::fs::{create_dir_all, read_to_string, write};
use toml::Table;
use url::Url;

use super::{ApiKeys, Auth, OAuth, Output, Service};

pub const DEFAULT_PROFILE: &str = "default";

#[derive(Clone, Debug, Default)]
pub struct ProfileFile {
    pub mongosh_path: Option<String>,
    pub telemetry_enabled: Option<bool>,
    pub skip_update_check: Option<bool>,
    pub default_profile: Option<Profile>,
    pub profiles: BTreeMap<String, Profile>,
    pub additional_properties: BTreeMap<String, AdditionalProperty>,
}

#[derive(Clone, Debug, Default)]
pub struct Profile {
    pub project_id: Option<String>,
    pub org_id: Option<String>,
    pub mongosh_path: Option<String>,
    pub service: Option<Service>,
    pub client_id: Option<String>,
    pub ops_manager_url: Option<Url>,
    pub base_url: Option<Url>,
    pub output: Option<Output>,
    pub auth: Option<Auth>,
    pub additional_properties: BTreeMap<String, AdditionalProperty>,
}

#[derive(Clone, Debug)]
pub struct AdditionalProperty(toml::Value);

impl ProfileFile {
    pub async fn load(path: impl AsRef<Path>) -> Result<Self, ProfileFileLoadError> {
        let yaml_string = read_to_string(path).await?;
        let yaml_value = yaml_string.parse::<toml::Table>()?;
        Ok(ProfileFile::try_from(yaml_value)?)
    }

    pub async fn save(&self, path: impl AsRef<Path>) -> Result<(), ProfileFileSaveError> {
        let toml_table: toml::Table = self.clone().into();
        let toml = toml::to_string_pretty(&toml_table)?;

        let path = path.as_ref();
        let mut directory_path = PathBuf::try_from(path).expect("error is of type Infallible");
        directory_path.pop();

        create_dir_all(directory_path).await?;
        write(path, toml).await?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ProfileFileLoadError {
    #[error("Read error")]
    Read(#[from] std::io::Error),
    #[error("Deserialize error")]
    Deserialize(#[from] toml::de::Error),
    #[error("Failed to convert toml")]
    YamlConversion(#[from] ProfileFileTryFromTomlError),
}

#[derive(Error, Debug)]
pub enum ProfileFileSaveError {
    #[error("Serialize error")]
    SerializeError(#[from] toml::ser::Error),
    #[error("Write error")]
    WriteError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ProfileFileTryFromTomlError {
    #[error("the value 'default' is supposed to be an object")]
    ValueDefaultOfWrongType,
    #[error("failed to convert entry")]
    ConvertEntry(#[from] InvalidEntryTypeError),
    #[error("failed to convert profile")]
    ConvertProfile(#[from] ProfileTryFromTomlError),
}

#[derive(Error, Debug)]
#[error("the value '{key}' has an unexpected type")]
pub struct InvalidEntryTypeError {
    key: &'static str,
}

#[derive(Error, Debug)]
pub enum ProfileTryFromTomlError {
    #[error("failed to convert entry")]
    ConvertEntry(#[from] InvalidEntryTypeError),
}

fn remove_entry<'de, T: Deserialize<'de>>(
    table: &mut Table,
    key: &'static str,
) -> Result<Option<T>, InvalidEntryTypeError> {
    table
        .remove(key)
        .map(|value| toml::Value::try_into(value).map_err(|_| InvalidEntryTypeError { key }))
        .transpose()
}

fn insert_entry<T: Into<toml::Value>>(table: &mut Table, key: &'static str, value: &mut Option<T>) {
    if let Some(value) = value.take() {
        table.insert(key.to_string(), value.into());
    }
}

impl TryFrom<toml::Table> for ProfileFile {
    type Error = ProfileFileTryFromTomlError;

    fn try_from(mut value: toml::Table) -> Result<Self, Self::Error> {
        let mut file = Self::default();
        file.mongosh_path = remove_entry(&mut value, "mongosh_path")?; //: Option<String>,
        file.telemetry_enabled = remove_entry(&mut value, "telemetry_enabled")?; //: Option<bool>,
        file.skip_update_check = remove_entry(&mut value, "skip_update_check")?; //: Option<bool>,

        for (key, value) in value {
            match value {
                toml::Value::Table(map) => {
                    let profile = Profile::try_from(map)?;

                    if key == DEFAULT_PROFILE {
                        file.default_profile = Some(profile);
                    } else {
                        file.profiles.insert(key, profile);
                    }
                }
                _ => {
                    if key == DEFAULT_PROFILE {
                        return Err(ProfileFileTryFromTomlError::ValueDefaultOfWrongType);
                    }

                    file.additional_properties
                        .insert(key, AdditionalProperty(value));
                }
            }
        }

        Ok(file)
    }
}

impl From<ProfileFile> for toml::Table {
    fn from(mut value: ProfileFile) -> Self {
        let mut table = Default::default();

        insert_entry(&mut table, "mongosh_path", &mut value.mongosh_path);
        insert_entry(
            &mut table,
            "telemetry_enabled",
            &mut value.telemetry_enabled,
        );
        insert_entry(
            &mut table,
            "skip_update_check",
            &mut value.skip_update_check,
        );

        for (key, value) in value.additional_properties {
            table.insert(key, value.0);
        }

        if let Some(default_profile) = value.default_profile.take() {
            table.insert(
                DEFAULT_PROFILE.to_string(),
                toml::Value::Table(default_profile.into()),
            );
        }

        for (name, profile) in value.profiles {
            table.insert(name, toml::Value::Table(profile.into()));
        }

        table
    }
}

impl TryFrom<toml::Table> for Profile {
    type Error = ProfileTryFromTomlError;
    fn try_from(mut value: toml::Table) -> Result<Self, Self::Error> {
        let mut profile = Self::default();

        if let (Some(public), Some(private)) = (
            remove_entry::<String>(&mut value, "public_api_key")?,
            remove_entry::<String>(&mut value, "private_api_key")?,
        ) {
            profile.auth = Some(Auth::ApiKeys(ApiKeys { private, public }));
        } else {
            if let (Some(access_token), Some(refresh_token)) = (
                remove_entry::<String>(&mut value, "access_token")?,
                remove_entry::<String>(&mut value, "refresh_token")?,
            ) {
                profile.auth = Some(Auth::OAuth(OAuth {
                    access_token,
                    refresh_token,
                }));
            }
        }

        profile.project_id = remove_entry(&mut value, "project_id")?;
        profile.org_id = remove_entry(&mut value, "org_id")?;
        profile.mongosh_path = remove_entry(&mut value, "mongosh_path")?;
        profile.service = remove_entry(&mut value, "service")?;
        profile.client_id = remove_entry(&mut value, "client_id")?;
        profile.ops_manager_url = remove_entry(&mut value, "ops_manager_url")?;
        profile.base_url = remove_entry(&mut value, "base_url")?;
        profile.output = remove_entry(&mut value, "output")?;

        for (key, value) in value {
            profile
                .additional_properties
                .insert(key, AdditionalProperty(value));
        }

        Ok(profile)
    }
}

impl From<Profile> for toml::Table {
    fn from(mut value: Profile) -> Self {
        let mut table = Default::default();
        insert_entry(&mut table, "project_id", &mut value.project_id);
        insert_entry(&mut table, "org_id", &mut value.org_id);
        insert_entry(&mut table, "mongosh_path", &mut value.mongosh_path);
        insert_entry(
            &mut table,
            "service",
            &mut value.service.map(|s| s.to_string()),
        );
        insert_entry(&mut table, "client_id", &mut value.client_id);
        insert_entry(
            &mut table,
            "ops_manager_url",
            &mut value.ops_manager_url.map(|s| s.to_string()),
        );
        insert_entry(
            &mut table,
            "base_url",
            &mut value.base_url.map(|s| s.to_string()),
        );
        insert_entry(
            &mut table,
            "output",
            &mut value.output.map(|s| s.to_string()),
        );

        if let Some(auth) = value.auth {
            match auth {
                Auth::ApiKeys(ApiKeys { public, private }) => {
                    table.insert("public_api_key".to_string(), public.into());
                    table.insert("private_api_key".to_string(), private.into());
                }
                Auth::OAuth(OAuth {
                    access_token,
                    refresh_token,
                }) => {
                    table.insert("access_token".to_string(), access_token.into());
                    table.insert("refresh_token".to_string(), refresh_token.into());
                }
            }
        }

        for (key, value) in value.additional_properties {
            table.insert(key, value.0);
        }

        table
    }
}
