use std::ffi::CStr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use libftdi1_sys::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DmxControllerError {
    #[error("ftdi_new() failed! Out of memory!")]
    FtdiNewFailed,

    #[error("USB open failed (code {code}): {message}")]
    UsbOpenFailed {code: i32, message: String},

    #[error("Set baudrate failed (code {code}): {message}")]
    BaudrateFailed {code: i32, message: String},

    #[error("Set line property failed (code {code}): {message}")]
    LinePropertyFailed {code: i32, message: String},

    #[error("Write failed (code {code}): {message}")]
    WriteFailed {code: i32, message: String},
}

// helper function: get error string from c ptr
unsafe fn ftdi_error_string(ctx: *mut ftdi_context) -> String { unsafe {
    CStr::from_ptr(ftdi_get_error_string(ctx))
        .to_string_lossy()
        .into_owned()
} }

pub struct DmxController {
    context: Arc<Mutex<FtdiContext>>,
    universe: Arc<Mutex<[u8; 513]>>,
    running: Arc<AtomicBool>,
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
    pub unsafe fn new() -> Result<Self, DmxControllerError> {
        unsafe {
            let context = ftdi_new();
            if context.is_null() { return Err(DmxControllerError::FtdiNewFailed) }
            let ret = ftdi_usb_open(context, 0x0403, 0x6001);
            if ret < 0 {
                return Err(DmxControllerError::UsbOpenFailed {
                    code: ret,
                    message: ftdi_error_string(context),
                })
            }

            let ret = ftdi_set_baudrate(context, 250000);
            if ret < 0 {
                return Err(DmxControllerError::BaudrateFailed {
                    code: ret,
                    message: ftdi_error_string(context),
                })
            }

            let ret = ftdi_set_line_property(
                context, ftdi_bits_type(8), ftdi_stopbits_type(2), ftdi_parity_type(0));
            if ret < 0 {
                return Err(DmxControllerError::LinePropertyFailed {
                    code: ret,
                    message: ftdi_error_string(context)
                })
            }

            Ok(Self {
                context: Arc::new(Mutex::new(FtdiContext(context))),
                universe: Arc::new(Mutex::new([0u8; 513])),
                running: Arc::new(AtomicBool::new(false))
            })
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


unsafe fn send_frame(context: *mut ftdi_context, frame: [u8; 513])
    -> Result<(), DmxControllerError> {
    unsafe {
        let check = |ret: i32| -> Result<(), DmxControllerError> {
            if ret < 0 {
                Err(DmxControllerError::LinePropertyFailed {
                    code: ret,
                    message: ftdi_error_string(context)
                })
            }
            else {Ok(())}
        };

        check(ftdi_set_line_property2(context, ftdi_bits_type(8), ftdi_stopbits_type(2),
                                    ftdi_parity_type(0), ftdi_break_type(1)))?;
        sleep(Duration::from_micros(88));
        check(ftdi_set_line_property2(context, ftdi_bits_type(8), ftdi_stopbits_type(2),
                                      ftdi_parity_type(0), ftdi_break_type(0)))?;
        sleep(Duration::from_micros(8)); // MAB
        let ret = ftdi_write_data(context, frame.as_ptr(), frame.len() as i32);
        if ret < 0 {
            return Err(DmxControllerError::WriteFailed {
                code: ret,
                message: ftdi_error_string(context)
            })
        }
        sleep(Duration::from_millis(25));
        Ok(())
    }
}