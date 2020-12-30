// main.rs

#![no_std]
#![no_main]

extern crate heapless;
extern crate morse_utils;
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

use morse_utils::morse_utils::*;

fn helper_fill_events_slice<T>(durations: &[i64], vec: &mut Vec<TimedLightEvent, T>)
where
    T: heapless::ArrayLength<TimedLightEvent>,
{
    for (i, duration) in durations.iter().enumerate() {
        vec.push(TimedLightEvent {
            light_state: {
                if i % 2 == 0 {
                    LightState::Dark
                } else {
                    LightState::Dark
                }
            },
            duration: *duration,
        })
        .unwrap();
    }
}

fn best_error_helper(light_state: LightState, duration: i64, units: i64) -> i64 {
       
    match best_error(
        &TimedLightEvent {
            light_state,
            duration,
        },
        units,
    )
    {
       Ok(s) => s.score,
       _ => 200000, 
    }
} 

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);

    let mut led = pins.d13.into_output(&mut pins.ddr);

    let test_durations = [
        700, 300, 100, 100, 100, 100, 100, 100, 300, 300, 100, 300, 100, 300, 300, 100, 100, 100,
        100, 300, 300, 300, 300, 300, 300, 100, 300, 300, 300, 100, 100, 700, 300, 100, 300, 100,
        300, 300, 300, 100, 300, 100, 300, 300, 100, 100, 100, 100, 300, 100, 100, 700,
    ];

    stutter_blink(&mut led, 1);
    arduino_uno::delay_ms(1000);
    stutter_blink(&mut led, 2);

    let mut timed_light_events: Vec<TimedLightEvent, U128> = Vec::new();
    helper_fill_events_slice(&test_durations, &mut timed_light_events);

    arduino_uno::delay_ms(1000);
    stutter_blink(&mut led, 3);

    let expected: Scored<i64> = Scored {
        item: 100,
        score: 0,
    };
    let myb = 
         match estimate_unit_time(&timed_light_events[0..1]) {
            Ok(actual) => expected == actual,
            _ => false,
        };

    if myb {
        loop {
            stutter_blink(&mut led, 4);
    arduino_uno::delay_ms(1000);
        }
    } else {
        loop {

            stutter_blink(&mut led, 1);
    arduino_uno::delay_ms(1000);
        }
    }
    loop {}
}
