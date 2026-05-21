use crate::raw::*;
use crate::ConfigError::{InvalidBandRange, InvalidValue};
use crate::{AudioConfig, BandConfig, ChannelDef, ConfigError, FixtureProfile};

impl TryFrom<RawChannelDef> for ChannelDef {
    type Error = ConfigError;

    fn try_from(value: RawChannelDef) -> Result<Self, Self::Error> {
        ChannelDef::new(&value.name, value.offset, value.default)
    }
}

impl TryFrom<RawFixtureProfile> for FixtureProfile {
    type Error = ConfigError;

    fn try_from(value: RawFixtureProfile) -> Result<Self, Self::Error> {
        FixtureProfile::new(&value.name,
                            value.channel_list
                                .into_iter()
                                .map(|c| ChannelDef::try_from(c))
                                .collect::<Result<Vec<_>, _>>()?)
    }
}



impl TryFrom<RawAudioConfig> for AudioConfig {
    type Error = ConfigError;

    fn try_from(value: RawAudioConfig) -> Result<Self, Self::Error> {
        for (name, band) in [
            ("sub_bass", &value.sub_bass),
            ("bass",     &value.bass),
            ("mid",      &value.mid),
            ("high",     &value.high),
        ] {
            if band.min >= band.max || band.min < 0.0 {
                return Err(InvalidBandRange { band: name.to_string() })
            }
        }

        if value.sample_rate == 0 {
            return Err(InvalidValue {field: "sample_rate".into(),
                reason: "needs to be greater than 0".into()});
        }

        if value.buffer_size == 0 {
            return Err(InvalidValue {field: "buffer_size".into(),
                reason: "needs to be greater than 0".into()});
        }
        
        if value.smoothing_factor > 1.0 || value.smoothing_factor < 0.0 {
            return Err(InvalidValue {field: "smoothing_factor".into(),
                reason: "needs to be beween 0.0 and 1.0".into()})
        }

        Ok(AudioConfig::new(
            value.sample_rate,
            value.buffer_size,
            value.smoothing_factor,
            value.device,
            BandConfig::new(value.sub_bass.min, value.sub_bass.max),
            BandConfig::new(value.bass.min, value.bass.max),
            BandConfig::new(value.mid.min, value.mid.max),
            BandConfig::new(value.high.min, value.high.max),
        ))
    }
}