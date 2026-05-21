use fl_config::loader::{load_audio_config, load_fixture_config};
use fl_dmx::DmxController;
use std::path::Path;

fn main() {
    let (profiles, fixtures) = load_fixture_config(Path::new("config/fixtures.toml"))
        .expect("Failed to load fixtures");

    let audio_config = load_audio_config(Path::new("config/audio.toml"))
        .expect("Failed to load audio config");

    let mut controller = unsafe { DmxController::new().unwrap() };
    controller.start_send();
    loop {
        for fixture in &fixtures {
            controller.set_channels(
                fixture.get_start_address().into(),
                &[100, 255, 0, 0, 0, 0, 0, 0]
            )
        }
    }
}