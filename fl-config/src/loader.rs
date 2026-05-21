use crate::raw::{RawAudioConfig, RawConfig};
use crate::{AudioConfig, ConfigError, Fixture, FixtureProfile};
use std::path::Path;
use std::sync::Arc;

pub fn load_fixture_config(path: &Path) -> Result<(Vec<Arc<FixtureProfile>>, Vec<Fixture>), ConfigError> {
    let data = std::fs::read_to_string(path)?;
    let raw_config: RawConfig = toml::from_str(&data)?;

    let mut fixture_profiles: Vec<Arc<FixtureProfile>> = Vec::new();
    let mut fixtures: Vec<Fixture> = Vec::new();

    // Create FixtureProfile from RawFixtureProfile
    for profile in raw_config.fixture_profiles {
        fixture_profiles.push(Arc::new(profile.try_into()?))
    }

    // Check if specified profiles exist and create Fixture from RawFixture
    for fixture in raw_config.fixtures {
        let profile = fixture_profiles.iter().find(|&p|
            p.name == fixture.profile).ok_or_else(|| { ConfigError::UnknownProfile {
                fixture: fixture.label.clone(), profile: fixture.profile.into()
            }
        })?;


        fixtures.push(Fixture::new(
            &*fixture.label,
            fixture.start_address,
            profile.clone()
        )?)
    }

    // Check if fixtures are overlapping
    for i in 0..fixtures.len() {
        for j in (i + 1)..fixtures.len() {
            let start_a = fixtures[i].get_start_address();
            let start_b = fixtures[j].get_start_address();
            let end_a = start_a + fixtures[i].get_profile().channels().len() as u16 - 1;
            let end_b = start_b + fixtures[j].get_profile().channels().len() as u16 - 1;
            if start_a <= end_b && start_b <= end_a {
                return Err(ConfigError::AddressConflict {
                    fixture_a: fixtures[i].label.clone(), fixture_b: fixtures[j].label.clone()})
            }
        }
    }

    Ok((fixture_profiles, fixtures))
}


pub fn load_audio_config(path: &Path) ->  Result<AudioConfig, ConfigError> {
    let data = std::fs::read_to_string(path)?;
    let raw_audio_config: RawAudioConfig = toml::from_str(&data)?;

    raw_audio_config.try_into()
}
