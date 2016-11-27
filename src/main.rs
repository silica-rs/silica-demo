#![feature(lang_items)]
#![feature(alloc)]

#![no_main]
#![no_std]

#![allow(dead_code)]

extern crate silica_olimex_p207; // board specific
extern crate silica;    // generic code
extern crate alloc;     // even more generic code

use silica::peripheral::serial::{Serial as ISerial, BitCount, StopBit, Parity};
use silica::io::Write;
use silica::io::Read;
use silica_olimex_p207::usart::*;

#[no_mangle]
pub fn main() {
    // os init
    silica::alloc::init();

    // board init
    silica_olimex_p207::init();

    // app init
    let mut ext = Serial::from(&silica_olimex_p207::RS232_1);
    if let Err(ref err) = ext.setup(115200, BitCount::EightBits, Parity::None, StopBit::OneBit) {
        panic!("{:?}", err);
    }

    if let Err(ref err) = ext.write("bonjour".as_bytes()) {
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
}
