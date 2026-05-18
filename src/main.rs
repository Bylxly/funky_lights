use std::thread::sleep;
use std::time::Duration;

mod dmx_interface;

fn main() {
    let mut controller = unsafe { dmx_interface::DmxController::new() };
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
