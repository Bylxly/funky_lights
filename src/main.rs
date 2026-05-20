use fl_config::loader::load;
use fl_dmx::DmxController;
use std::path::Path;

fn main() {
    let (profiles, fixtures) = load(Path::new("config/fixtures.toml"))
        .expect("Failed to load fixtures");

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