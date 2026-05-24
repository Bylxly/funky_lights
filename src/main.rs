use fl_audio::AudioAnalyzer;
use fl_config::loader::{load_audio_config, load_fixture_config};
use fl_config::BandEnergies;
use fl_dmx::DmxController;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (profiles, fixtures) = load_fixture_config(Path::new("config/fixtures.toml"))
        .expect("Failed to load fixtures");

    let audio_config = load_audio_config(Path::new("config/audio.toml"))
        .expect("Failed to load audio config");

    let band_energies = Arc::new(Mutex::new(BandEnergies::new()));
    let analyzer = AudioAnalyzer::new(audio_config, Arc::clone(&band_energies));

    loop {
        sleep(Duration::from_millis(100));
        let energies = band_energies.lock().unwrap();
        println!("sub_bass: {:.2}, bass: {:.2}, mid: {:.2}, high: {:.2}",
                 energies.get_sub_bass(),
                 energies.get_bass(),
                 energies.get_mid(),
                 energies.get_high());
    }

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