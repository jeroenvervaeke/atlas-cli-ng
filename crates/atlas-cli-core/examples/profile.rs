use std::{error::Error, str::FromStr};

use atlas_cli_core::{paths::Paths, profile::ProfileFile};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let paths = Paths::new()?;
    let profile_path = paths.profile_path().to_string_lossy().to_string();
    println!("profile path: {profile_path}");
    let mut profile_file = ProfileFile::load(&profile_path).await?;
    println!("profile_file: {profile_file:#?}");

    if let Some(default_profile) = &mut profile_file.default_profile {
        default_profile.base_url = Some(Url::from_str("http://example.com/").expect("valid url"));
    }

    println!();
    println!("updated profile_file: {profile_file:#?}");

    profile_file.save(profile_path).await?;

    Ok(())
}
