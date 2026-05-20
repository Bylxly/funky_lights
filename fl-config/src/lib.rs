mod raw;
mod convert;
pub mod loader;

use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("'{type_name}' name cannot be empty")]
    EmptyName {type_name: &'static str},

    #[error("Fixture '{fixture}' has invalid address: {address}")]
    InvalidAddress {fixture: String, address: u16},

    #[error("Fixture '{fixture}' references unknown profile '{profile}'")]
    UnknownProfile {fixture: String, profile: String},

    #[error("Address conflict between '{fixture_a}' and '{fixture_b}'")]
    AddressConflict {fixture_a: String, fixture_b: String},

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlError(#[from] toml::de::Error)
}

pub struct ChannelDef {
    pub name: String,
    pub offset: u8,
    pub default: u8,
}

impl ChannelDef {
    pub fn new(name: &str, offset: u8, default: u8) -> Result<Self, ConfigError> {
        if name.is_empty() {return Err(ConfigError::EmptyName{type_name: "ChannelDef"})};

        Ok (Self {
            name: name.into(),
            offset,
            default
        })
    }
}

pub struct FixtureProfile {
    pub name: String,
    channel_list: Vec<ChannelDef>,
}

impl FixtureProfile {
    pub fn new(name: &str, channel_list: Vec<ChannelDef>) -> Result<Self, ConfigError> {
        if name.is_empty() {return Err(ConfigError::EmptyName {type_name: "FixtureProfile"})}

        Ok(Self {
            name: name.into(),
            channel_list,
        })
    }

    pub fn channels(&self) -> &[ChannelDef] {
        &self.channel_list
    }
}

pub struct Fixture {
    pub label: String,
    start_address: u16,
    profile: Arc<FixtureProfile>
}

impl Fixture {
    pub fn new(label: &str, start_address: u16, profile: Arc<FixtureProfile>) -> Result<Self, ConfigError> {
        if label.is_empty() {return Err(ConfigError::EmptyName {type_name: "Fixture"})}
        if start_address > 512 || start_address == 0 {
            //TODO: Error richtig definieren
            return Err(ConfigError::InvalidAddress { fixture: "".to_string(), address: 0 })
        }

        Ok(Self{
            label: label.into(),
            start_address,
            profile
        })
    }

    pub fn get_start_address(&self) -> u16 {
        self.start_address
    }

    pub fn get_profile(&self) -> &FixtureProfile {
        &self.profile
    }
}