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
use core::convert::TryFrom;

fn stutter_blink(led: &mut PB5<Output>, times: i16) {
    for i in 0..times {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(100);
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(100);
    }
    arduino_uno::delay_ms(1000);
}

use morse_utils::*;

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
    ) {
        Ok(s) => s.score,
        _ => 200000,
    }
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::heapless::FnvIndexMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn split_slice<'a, T>(sl: &'a [T], on: &T) -> Vec<Vec<&'a T, U32>,U32>
where
    T: core::fmt::Debug + core::cmp::PartialEq,
{
    let mut v = Vec::new();

    v.push(Vec::new());
    let mut count = 0;

    for item in sl.iter() {
        if item == on {
            v.push(Vec::new());
            count += 1;
        } else {
            v[count].push(item);
        }
    }
    v
}

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);

    let mut led = pins.d13.into_output(&mut pins.ddr);

    let intensities = [
        (4480, 14),
        (4500, 51),
        (4520, 57),
        (4540, 58),
        (4560, 59),
        (9760, 59),
        (9770, 59),
        (9780, 59),
    ];

    stutter_blink(&mut led, 2);
    arduino_uno::delay_ms(1000);
    stutter_blink(&mut led, 2);

    let mut timed_light_events: Vec<_, U512> = Vec::new();
    convert(&intensities[..], &mut timed_light_events, 0).unwrap();

  let unit = estimate_unit_time(&timed_light_events, 0, 1000)
        .unwrap()
        .item;

    let r: Vec<Scored<&MorseCandidate>, U256> = timed_light_events
        .iter()
        .map(|tle| morse_utils::best_error(tle, unit))
        .filter_map(Result::ok)
        .collect();   

  let r: Vec<morse_utils::Morse, U256> = r
        .into_iter()
        .map(|s| morse_utils::mc_to_morse(s.item))
        .collect();

  let morse_key: FnvIndexMap<&str, char, U64> = hashmap![
    "01" => 'a',
    "1000" => 'b',
    "1010" => 'c',
    "100" => 'd',
    "0" => 'e',
    "0010" => 'f',
    "110" => 'g',
    "0000" => 'h',
    "00" => 'i',
    "0111" => 'j',
    "101" => 'k',
    "0100" => 'l',
    "11" => 'm',
    "10" => 'n',
    "111" => 'o',
    "0110" => 'p',
    "1101" => 'q',
    "010" => 'r',
    "000" => 's',
    "1" => 't',
    "001" => 'u',
    "0001" => 'v',
    "011" => 'w',
    "1001" => 'x',
    "1011" => 'y',
    "1100" => 'z'
    ];

  let v: Vec<Vec<char,U32>, U32> = split_slice(&r, &Morse::WordSpace)
        .iter()
        .map(|v| {
            split_slice(v, &&Morse::LetterSpace)
                .into_iter()
                .map(|v| {
                    let my_vec: Vec<char,U128> = v
                        .into_iter()
                        .map(|m| {
                            use morse_utils::Morse::*;
                            match m {
                                Dot => Some('0'),
                                Dash => Some('1'),
                                TinySpace => None,
                                _ => Some('2'),
                            }
                        })
                        .filter_map(|x| x)
                        .collect();
                    let mut my_string : heapless::String<U128> = heapless::String::new();
                    for m in my_vec.iter()
                    {
                        my_string.push(*m);
                    }
                    let k = my_string.as_str();
                    *morse_key.get(k).unwrap_or(&'?')
                })
                .collect()
        })
        .collect();

        loop {
    stutter_blink(&mut led, i16::try_from(v.len()).unwrap() + 1);
    arduino_uno::delay_ms(1000);
        }

    // stutter_blink(&mut led, 2);

    // let mut timed_light_events: Vec<TimedLightEvent, U128> = Vec::new();
    // helper_fill_events_slice(&test_durations, &mut timed_light_events);

    // arduino_uno::delay_ms(1000);
    // stutter_blink(&mut led, 3);

    // let expected: Scored<i64> = Scored {
    //     item: 100,
    //     score: 0,
    // };
    // match estimate_unit_time(&timed_light_events, 90, 110) {
    //     Ok(actual) if expected == actual => loop {
    //         stutter_blink(&mut led, 5);
    //         arduino_uno::delay_ms(1000);
    //     },
    //     Err(_) => loop {
    //         stutter_blink(&mut led, 3);
    //         arduino_uno::delay_ms(1000);
    //     },
    //     _ => loop {
    //         stutter_blink(&mut led, 1);
    //         arduino_uno::delay_ms(1000);
    //     },
    // };
    loop {}
}
