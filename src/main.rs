//#![feature(lang_items)]
#![feature(alloc)]

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

#[cfg(feature = "olimex-p207")]
mod olimex_p207 {
    use silica_olimex_p207;
    use silica_olimex_p207::gpio::Out;
    pub fn get_leds() -> (Out<'static>, Out<'static>) {
        (
            Out::from(&silica_olimex_p207::STAT1_E).unwrap(),
            Out::from(&silica_olimex_p207::STAT2_E).unwrap()
        )
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
        Serial {}
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
        Serial {}
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
 * STAT1_E : PF6
 * STAT2_E : PF7 (CAN_CTRL)
 * STAT3_E : PF8 (CS_UEXT)
 * STAT4_E : PF9 (CAM_PWR)
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

    use silica::peripheral::gpio::Output;
    let _ = lowlevel_status;
    let mut state = true;
    let (mut stat1, mut stat2) = get_leds();
    loop {
        stat1.write(state);
        stat2.write(state);
        state = !state;

        let mut cnt = 0xFFFFFF;
        while cnt > 0 {
            cnt -= 1;
        }
    }
}
