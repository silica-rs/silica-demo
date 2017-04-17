#![feature(lang_items)]
#![feature(alloc)]

#![no_main]
#![no_std]

#![allow(dead_code)]

extern crate silica_olimex_p207; // board specific
extern crate silica;    // generic code
extern crate alloc;     // even more generic code

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
    let lowlevel_status = if let Err(msg) = silica_olimex_p207::init() {
        msg
    } else {
        &"Ok"
    };

    // os init : initialize memory manager
    silica::alloc::init();

    use silica_olimex_p207::Peripheral;
    use silica::peripheral::gpio::Output;
    use silica_olimex_p207::gpio::*;
    let _ = lowlevel_status;
    let mut state = true;
    silica_olimex_p207::STAT1_E.init();
    silica_olimex_p207::STAT2_E.init();

    let mut stat1 = Out::from(&silica_olimex_p207::STAT1_E);
    let mut stat2 = Out::from(&silica_olimex_p207::STAT2_E);
    loop {
        stat1.write(state);
        stat2.write(state);
        state = !state;

        let mut cnt = 0xFFFFFF;
        while cnt > 0 {
            cnt -= 1;
        }
    }
/*
    use silica::peripheral::serial::{Serial as ISerial, BitCount, StopBit, Parity};
    use silica::io::Write;
    use silica::io::Read;
    use silica_olimex_p207::usart::*;
    // app init
    let mut ext = Serial::from(&silica_olimex_p207::RS232_1);
    if let Err(ref err) = ext.setup(115200, BitCount::EightBits, Parity::None, StopBit::OneBit) {
        panic!("{:?}", err);
    }

    if let Err(ref err) = ext.write(lowlevel_status.as_bytes()) {
        panic!("{:?}", err);
    }

    let mut buffer = [0; 128];
    loop {
        let _ = ext.read(&mut buffer);
        let _ = ext.write(&buffer);
    }
    // app launch
    // let mut t = silica::thread::Thread::new(Box::new(|| { loop {} }), "testmethod", 128, silica::thread::ThreadPriority::Lowest);
    // t.start().join(Duration::from(0u64));
    */
}
