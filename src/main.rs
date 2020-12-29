// main.rs

#![no_std]
#![no_main]

extern crate heapless;
extern crate panic_halt; // v0.4.x

use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::prelude::*;

use heapless::consts::*;
use heapless::FnvIndexMap;
use heapless::Vec;

fn stutter_blink(led: &mut PB5<Output>, times: i16) {
    for i in 0..times {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(100);
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(100);
    }
    arduino_uno::delay_ms(1000);
}

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);

    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        stutter_blink(&mut led, 5);
    }
}
