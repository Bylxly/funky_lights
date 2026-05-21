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
    TomlError(#[from] toml::de::Error),


    #[error("Band '{band}' has invalid band range")]
    InvalidBandRange{band: String},

    #[error("Value '{field}' is invalid. Reason: '{reason}'")]
    InvalidValue {field: String, reason: String}
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
            return Err(ConfigError::InvalidAddress { fixture: label.to_string(), address: start_address })
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



pub struct BandEnergies {
    sub_bass: f32,
    bass: f32,
    mid: f32,
    high: f32,
}

impl BandEnergies {
    pub fn new() -> Self {
        Self{
            sub_bass: 0f32,
            bass: 0f32,
            mid: 0f32,
            high: 0f32,
        }
    }

    pub fn update(&mut self, sub_bass: f32, bass: f32, mid: f32, high: f32) {
        self.sub_bass = sub_bass;
        self.bass = bass;
        self.mid = mid;
        self.high = high;
    }

    pub fn get_sub_bass(&self) -> f32 {
        self.sub_bass
    }

    pub fn get_bass(&self) -> f32 {
        self.bass
    }

    pub fn get_mid(&self) -> f32 {
        self.mid
    }

    pub fn get_high(&self) -> f32 {
        self.high
    }
}

pub struct BandConfig {
    min: f32,
    max: f32,
}

impl BandConfig {
    pub fn new(min: f32, max: f32) -> Self {
        Self{ min, max }
    }

    pub fn get_min(&self) -> f32 {
        self.min
    }

    pub fn get_max(&self) -> f32 {
        self.max
    }
}

pub struct AudioConfig {
    sample_rate: u32,
    buffer_size: u32,
    smoothing_factor: f32,
    device: String,
    sub_bass: BandConfig,
    bass: BandConfig,
    mid: BandConfig,
    high: BandConfig,
}

impl AudioConfig {
    pub fn new(
        sample_rate: u32,
        buffer_size: u32,
        smoothing_factor: f32,
        device: String,
        sub_bass: BandConfig,
        bass: BandConfig,
        mid: BandConfig,
        high: BandConfig) -> Self {
        Self{
            sample_rate,
            buffer_size,
            smoothing_factor,
            device,
            sub_bass,
            bass,
            mid,
            high
        }
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn get_buffer_size(&self) -> u32 {
        self.buffer_size
    }

    pub fn get_smoothing_factor(&self) -> f32 {
        self.smoothing_factor
    }

    pub fn get_device(&self) -> &str {
        &self.device
    }

    pub fn get_sub_bass(&self) -> &BandConfig {
        &self.sub_bass
    }

    pub fn get_bass(&self) -> &BandConfig {
        &self.bass
    }

    pub fn get_mid(&self) -> &BandConfig {
        &self.mid
    }

    pub fn get_high(&self) -> &BandConfig {
        &self.high
    }
}