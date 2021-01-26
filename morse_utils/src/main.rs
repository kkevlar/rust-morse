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
    T: core::fmt::Debug + std::cmp::PartialEq  ,
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
    let mystr = std::fs::read_to_string("fastcar.txt").unwrap();
    let list: std::vec::Vec<&str> = mystr.split('\n').collect();

    let intensities: std::vec::Vec<(morse_utils::Time, morse_utils::LightIntensity)> = list
        .iter()
        .enumerate()
        .map(
            |(count, s)| -> Result<
                (morse_utils::Time, morse_utils::LightIntensity),
                core::num::ParseIntError,
            > {
                let ss = s.to_string();
                let s = ss.trim();
                Ok(((count) as morse_utils::Time, (1024u16 - s.parse::<u16>()?)))
            },
        )
        .filter_map(Result::ok)
        .collect();

    //     for i in intensities.iter()
    // { println!("int = {:?}", i); }
    let mut timed_light_events: Vec<_, U4096> = Vec::new();
    morse_utils::convert(&intensities, &mut timed_light_events, 0).unwrap();
    // println!("Tles = {:?}", timed_light_events);

    let unit = morse_utils::estimate_unit_time(&timed_light_events, 0, 1000)
        .unwrap()
        .item;

    let r: std::vec::Vec<morse_utils::Scored<&morse_utils::MorseCandidate>> = timed_light_events
        .iter()
        .map(|tle| morse_utils::best_error(tle, unit))
        .filter_map(Result::ok)
        .collect();

    let r: std::vec::Vec<morse_utils::Morse> = r
        .into_iter()
        .map(|s| morse_utils::mc_to_morse(s.item))
        .collect();

    for r in r.iter().take(100) {
        match r {
            morse_utils::Morse::Space => (),
            morse_utils::Morse::LongSpace => {
                println!("");
                println!("");
            }
            _ => println!("{:?}", r),
        };
    }

    let morse_key: HashMap<String, char> = hashmap![
    "01".to_string() => 'a',
    "1000".to_string() => 'b',
    "1010".to_string() => 'c',
    "100".to_string() => 'd',
    "0".to_string() => 'e',
    "0010".to_string() => 'f',
    "110".to_string() => 'g',
    "0000".to_string() => 'h',
    "00".to_string() => 'i',
    "0111".to_string() => 'j',
    "101".to_string() => 'k',
    "0100".to_string() => 'l',
    "11".to_string() => 'm',
    "10".to_string() => 'n',
    "111".to_string() => 'o',
    "0110".to_string() => 'p',
    "1101".to_string() => 'q',
    "010".to_string() => 'r',
    "000".to_string() => 's',
    "1".to_string() => 't',
    "001".to_string() => 'u',
    "0001".to_string() => 'v',
    "011".to_string() => 'w',
    "1001".to_string() => 'x',
    "1011".to_string() => 'y',
    "1100".to_string() => 'z'
    ];

    let morse_key: HashMap<std::vec::Vec<u8>, char> = morse_key
        .into_iter()
        .map(|(k, c)| {
            let v = k
                .chars()
                .into_iter()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => 2,
                })
                .collect();
            (v, c)
        })
        .collect();

    let v: std::vec::Vec<String> =
        split_slice(&r, &morse_utils::Morse::Error)
            .iter()
            .map(|v| {
                split_slice(v, &&morse_utils::Morse::LongSpace)
                    .into_iter()
                    .map(|v| {
                        let nums : std::vec::Vec<u8> = v.into_iter()
                            .map(|m| {
                                use morse_utils::Morse::*;
                                match m {
                                    Dot => Some(0),
                                    Dash => Some(1),
                                    Space => None, 
                                    _ => Some(2),
                                }
                            })
                            .filter_map(|x| x)
                            .collect();
                            morse_key.get(&nums).unwrap_or(&'?')
                    })
                    .collect()
            })
            .collect();

    println!("{:?}", v);
}
