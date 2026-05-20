use crate::raw::*;
use crate::{ChannelDef, ConfigError, FixtureProfile};

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