use serde::{Deserialize, Serialize};

mod profile;

pub use profile::*;

#[derive(Clone, Debug)]
pub enum Auth {
    ApiKeys(ApiKeys),
    OAuth(OAuth),
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum_macros::Display,
    strum_macros::EnumString,
    Serialize,
    Deserialize,
)]
pub enum Output {
    #[serde(rename = "json")]
    #[strum(serialize = "json")]
    Json,
    #[serde(rename = "plaintext")]
    #[strum(serialize = "plaintext")]
    Plaintext,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum_macros::Display,
    strum_macros::EnumString,
    Serialize,
    Deserialize,
)]
pub enum Service {
    #[serde(rename = "cloud")]
    #[strum(serialize = "cloud")]
    Cloud,
    #[serde(rename = "cloudgov")]
    #[strum(serialize = "cloudgov")]
    GovCloud,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiKeys {
    public: String,
    private: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OAuth {
    access_token: String,
    refresh_token: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enums() {
        assert_eq!("json", Output::Json.to_string());
        assert_eq!("plaintext", Output::Plaintext.to_string());
        assert_eq!("cloud", Service::Cloud.to_string());
        assert_eq!("cloudgov", Service::GovCloud.to_string());

        assert_eq!(Output::Json, "json".parse::<Output>().unwrap());
        assert_eq!(Output::Plaintext, "plaintext".parse::<Output>().unwrap());
        assert_eq!(Service::Cloud, "cloud".parse::<Service>().unwrap());
        assert_eq!(Service::GovCloud, "cloudgov".parse::<Service>().unwrap());
    }
}
