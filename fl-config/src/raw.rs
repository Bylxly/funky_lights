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



#[derive(Deserialize)]
pub(crate) struct RawBandConfig {
    pub(crate) min: f32,
    pub(crate) max: f32,
}

#[derive(Deserialize)]
pub(crate) struct RawAudioConfig {
    pub(crate) sample_rate: u32,
    pub(crate) buffer_size: u32,
    pub(crate) smoothing_factor: f32,
    pub(crate) device: String,
    pub(crate) sub_bass: RawBandConfig,
    pub(crate) bass: RawBandConfig,
    pub(crate) mid: RawBandConfig,
    pub(crate) high: RawBandConfig,
}