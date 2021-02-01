use heapless::consts::*;
use heapless::Vec;
use morse_utils::TimedLightEvent;
use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn split_slice<'a, T>(sl: &'a [T], on: &T) -> std::vec::Vec<std::vec::Vec<&'a T>>
where
    T: core::fmt::Debug + std::cmp::PartialEq,
{
    let mut v = std::vec::Vec::new();

    v.push(std::vec::Vec::new());
    let mut count = 0;

    for item in sl.iter() {
        if item == on {
            v.push(std::vec::Vec::new());
            count += 1;
        } else {
            v[count].push(item);
        }
    }

    println!(" returning {:?}", v);
    v
}


fn main() -> () {
    use morse_utils::*;

    let intensities = [
        (5, 50),
        (10, 50),
        (15, 500),
        (20, 50),
        (25, 500),
        (30, 50),
        (35, 500),
        (40, 50),
        (60, 51),
        (70, 500),
    ];

    let mut timed_light_events: Vec<_, U128> = Vec::new();
    match convert(&intensities[..], &mut timed_light_events, 0) {
        Err(_) => loop {
        },
        _ => (),
    };
    println!("{:?}", timed_light_events);

    let r = estimate_unit_time(&timed_light_events, 5, 6);
    match r {
        Err(_) => loop {
        },
        _ => (),
    }
    println!("{:?}", r);
}