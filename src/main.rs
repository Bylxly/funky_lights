use fl_config::loader::load;
use fl_dmx::DmxController;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let result = load(Path::new("config/fixtures.toml"));

    match result {
        Ok((profiles, fixtures)) => {
            println!("Loaded {} profiles", profiles.len());
            println!("Loaded {} fixtures", fixtures.len());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }


    let mut controller = unsafe { DmxController::new().unwrap() };
    controller.set_channel(1, 255);
    controller.set_channel(3, 255);
    controller.start_send();
    let mut i = 0;
    loop {
        controller.set_channels(1, &[100, 255, 255, i, 0, 0, 0]);
        i = i.wrapping_add(1);
        sleep(Duration::from_millis(10));
    }
}