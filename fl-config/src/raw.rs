use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct RawChannelDef {
    pub(crate) name: String,
    pub(crate) offset: u8,
    pub(crate) default: u8,
}

#[derive(Deserialize)]
pub(crate) struct RawFixtureProfile {
    pub(crate) name: String,
    pub(crate) channel_list: Vec<RawChannelDef>,
}

#[derive(Deserialize)]
pub(crate) struct RawFixture {
    pub(crate) label: String,
    pub(crate) start_address: u16,
    pub(crate) profile: String
}

#[derive(Deserialize)]
pub(crate) struct RawConfig {
    pub(crate) fixture_profiles: Vec<RawFixtureProfile>,
    pub(crate) fixtures: Vec<RawFixture>,
}