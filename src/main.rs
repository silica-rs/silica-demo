//#![feature(lang_items)]
#![feature(alloc)]
#![feature(collections)]

#![no_main]
#![no_std]

#[cfg(feature = "olimex-p207")]
extern crate silica_olimex_p207; // board specific

#[cfg(feature = "arduino-uno")]
extern crate silica_arduino_uno;

#[cfg(feature = "arduino-mega")]
extern crate silica_arduino_mega;

extern crate silica;    // generic code
extern crate alloc;     // even more generic code

#[macro_use]
extern crate collections;

#[cfg(feature = "olimex-p207")]
mod olimex_p207 {
    use silica_olimex_p207;
    use silica_olimex_p207::usart::Serial;
    use collections::String;
    pub fn get_serial() -> Result<Serial<'static>, String> {
        Serial::from(&silica_olimex_p207::RS232_1)
    }
    pub fn lowlevel_init() -> Result<u32, &'static str> {
        silica_olimex_p207::init()
    }
}

#[cfg(feature = "arduino-uno")]
mod arduino_uno {
    struct Pin {
    }
    pub fn get_leds<'a>() -> (Pin, Pin) {
        Pin {}
    }
    pub fn lowlevel_init() -> Result<u32, &'static str> {
        Ok(0)
    }
}

#[cfg(feature = "arduino-mega")]
mod arduino_mega {
    struct Pin {
    }
    pub fn get_leds<'a>() -> (Pin, Pin) {
        Pin {}
    }
    pub fn lowlevel_init() -> Result<u32, &'static str> {
        Ok(0)
    }
}

#[cfg(feature = "olimex-p207")]
use olimex_p207::*;

#[cfg(feature = "arduino-uno")]
use arduino_uno::*;

#[cfg(feature = "arduino-mega")]
use arduino_mega::*;

/**
 * board features :
 * RS232_1
 */

#[no_mangle]
pub fn main() {
    // board init : speed up clock etc
    let lowlevel_status = if let Err(msg) = lowlevel_init() {
        msg
    } else {
        &"Ok"
    };

    // os init : initialize memory manager
    silica::alloc::init();

    use silica::peripheral::serial::*;
    use silica::io::Read;
    use silica::io::Write;

    let _ = lowlevel_status;
    let mut serial = get_serial().unwrap();
    let _ = serial.open(9600, BitCount::EightBits, Parity::Even, StopBit::OneBit);

    let mut buf = vec![0; 512];
    serial.write(b"Hello world !\r\ncomment allez vous par ici, c'est un message assez long quand meme dit-donc !\r\n");
    let tout_reload = 100000usize;
    let mut timeout = tout_reload;
    let mut read = 0;
    loop {
        read += match serial.read(&mut buf[read..]) {
            Ok(n) => {
                if n == 0 {
                    timeout = timeout.saturating_sub(1);
                } else {
                    timeout = tout_reload;
                }
                n
            },
            Err(msg) => {
                //let _ = serial.write(msg.as_bytes());
                0
            }
        };
        if (timeout == 0) && (read > 0) {
            serial.write(&buf[..read]).unwrap();
            read = 0;
        }
    }
}
