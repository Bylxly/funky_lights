use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use libftdi1_sys::*;

pub struct DmxController {
    context: Arc<Mutex<FtdiContext>>,
    universe: Arc<Mutex<[u8; 513]>>,
    running: Arc<AtomicBool>
}

struct FtdiContext(*mut ftdi_context);
unsafe impl Send for FtdiContext {}
unsafe impl Sync for FtdiContext {}

// move Outside {
pub struct DmxFixture {
    dimmer: u8,
    red: u8,
    green: u8,
    blue: u8,
    white: u8,
    programme: u8,
    macro_mode: u8,
    strobe: u8,
}

impl DmxFixture {
    pub fn to_channels(&self) -> [u8; 8] {
        [self.dimmer, self.red, self.green, self.blue,
            self.white, self.programme, self.macro_mode, self.strobe]
    }
}
// }

impl DmxController {
    pub unsafe fn new() -> Self {
        let context = FtdiContext(ftdi_new());
        unsafe {
            ftdi_usb_open(context.0, 0x0403, 0x6001);
            ftdi_set_baudrate(context.0, 250000);
            ftdi_set_line_property(
                context.0, ftdi_bits_type(8), ftdi_stopbits_type(2), ftdi_parity_type(0));
        }
        Self {
            context: Arc::new(Mutex::new(context)),
            universe: Arc::new(Mutex::new([0u8; 513])),
            running: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn start_send(&self) {
        let context = Arc::clone(&self.context);
        let universe = Arc::clone(&self.universe);
        let running = Arc::clone(&self.running);
        running.store(true, Ordering::Relaxed);

        thread::spawn(move || unsafe {
            while running.load(Ordering::Relaxed) {
                let frame = *universe.lock().unwrap();
                let context = context.lock().unwrap();
                send_frame(context.0, frame);
            }
        });
    }

    pub fn stop_send(&self) {
        self.running.store(false, Ordering::Relaxed)
    }

    pub fn set_channel(&mut self, channel: usize, value: u8) {
        self.universe.lock().unwrap()[channel] = value;
    }

    pub fn set_channels(&mut self, start: usize, values: &[u8]) {
        let mut universe = self.universe.lock().unwrap();
        for (i, &value) in values.iter().enumerate() {
            universe[start + i] = value
        }
    }

    pub fn clear(&mut self) {
        *self.universe.lock().unwrap() = [0u8; 513];
    }

}


unsafe fn send_frame(context: *mut ftdi_context, frame: [u8; 513]) {
    unsafe {
        ftdi_set_line_property2(context, ftdi_bits_type(8), ftdi_stopbits_type(2), ftdi_parity_type(0), ftdi_break_type(1));
        sleep(Duration::from_micros(88));
        ftdi_set_line_property2(context, ftdi_bits_type(8), ftdi_stopbits_type(2), ftdi_parity_type(0), ftdi_break_type(0));
        sleep(Duration::from_micros(8)); // MAB
        ftdi_write_data(context, frame.as_ptr(), frame.len() as i32);
        sleep(Duration::from_millis(25));
    }
}