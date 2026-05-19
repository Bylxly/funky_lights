pub struct ChannelDef {
    pub name: String,
    pub offset: u8,
    pub default: u8,
}

impl ChannelDef {
    pub fn new(name: &str, offset: u8, default: u8) -> Result<Self, &'static str> {
        if name.is_empty() {return Err("ChannelDef Name cannot be empty")};

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
    pub fn new(name: &str) -> Result<Self, &'static str> {
        if name.is_empty() {return Err("FixtureProfile Name cannot be empty")}

        Ok(Self {
            name: name.into(),
            channel_list: Vec::new(),
        })
    }

    pub fn channels(&self) -> &[ChannelDef] {
        &self.channel_list
    }
}

pub struct Fixture {
    pub label: String,
    start_address: u16,
    profile: String
}

impl Fixture {
    pub fn new(label: &str, start_address: u16, profile: &str) -> Result<Self, &'static str> {
        if label.is_empty() {return Err("Fixture label cannot be empty")}
        if start_address > 512 {return Err("Fixture start address cannot be larger than 512")}

        Ok(Self{
            label: label.into(),
            start_address,
            profile: profile.into()
        })
    }
}