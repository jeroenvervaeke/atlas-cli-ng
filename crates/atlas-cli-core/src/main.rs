use std::{error::Error, str::FromStr};

use atlas_cli_core::{paths::Paths, profile::ProfileFile};
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    let paths = Paths::new()?;
    let profile_path = paths.profile_path().to_string_lossy().to_string();
    let mut profile_file = ProfileFile::load(&profile_path)?;
    profile_file.init_default_profile();
    let profile = profile_file.default_profile().unwrap();
    println!("default profile: {profile:?}");
    println!(
        "profile names: {:?}",
        profile_file.profile_names().collect::<Vec<_>>()
    );

    profile_file.init_profile("test");
    if let Some(test_profile) = profile_file.profile_mut("test") {
        test_profile.ops_manager_url = Some(Url::from_str("https://jev.sh").unwrap());
        println!("test profile: {test_profile:?}");
    }

    profile_file.save(&profile_path)?;
    println!(
        "new profile names: {:?}",
        profile_file.profile_names().collect::<Vec<_>>()
    );

    Ok(())
}
