#![no_std]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_code() {
        use Morse::*;
        let v: Vec<Morse, U8> = Vec::from_slice(&[Dot]).unwrap();
        assert_eq!(string_to_code(&"0"), v);
    }
}

extern crate heapless;

use heapless::consts::*;
use heapless::FnvIndexMap;
use heapless::Vec;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = heapless::FnvIndexMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Morse {
    Dot,
    Dash,
    Space,
    Error,
    LongSpace,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum LightState {
    Light,
    Dark,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Scored<T> {
    item: T,
    score: i64,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct TimedLightEvent {
    light_state: LightState,
    duration: i64,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct MorseCandidate {
    light_state: LightState,
    units: i64,
}

const MORSE_CANDIDATES: [MorseCandidate; 5] = [
    MorseCandidate {
        light_state: LightState::Light,
        units: 1,
    },
    MorseCandidate {
        light_state: LightState::Light,
        units: 3,
    },
    MorseCandidate {
        light_state: LightState::Dark,
        units: 1,
    },
    MorseCandidate {
        light_state: LightState::Dark,
        units: 3,
    },
    MorseCandidate {
        light_state: LightState::Dark,
        units: 7,
    },
];

fn calc_error(
    event: &TimedLightEvent,
    candidate: &MorseCandidate,
    unit_millis: i64,
) -> Option<i64> {
    if event.light_state == candidate.light_state {
        Some((event.duration - candidate.units * unit_millis).abs())
    } else {
        None
    }
}

fn make_score(
    event: &TimedLightEvent,
    mc: &'static MorseCandidate,
    unit_millis: i64,
) -> Option<Scored<&'static MorseCandidate>> {
    Some(Scored {
        item: mc,
        score: calc_error(event, mc, unit_millis)?,
    })
}

fn best_error(event: &TimedLightEvent, unit_millis: i64) -> Result<Scored<&MorseCandidate>, ()> {
    // Turns all of the possible morse candidates into an iterator
    MORSE_CANDIDATES
        .iter()
        // Scores each possible morse candidate using calc_error
        .map(|mc| {
            Some(Scored {
                item: mc,
                score: calc_error(event, mc, unit_millis)?,
            })
        })
        // Unwraps each optional score, leaving only scores which weren't failures
        .filter_map(|opt| opt)
        // Starts with none, but folds and returns the highest scoring Scored struct
        .fold(
            None,
            |min_so_far: Option<Scored<&MorseCandidate>>, scored| match min_so_far {
                Some(m) if m.score < scored.score => Some(m),
                _ => Some(scored),
            },
        )
        // Returns the Err variant if the Scored struct was not present
        .ok_or(())
}

fn unnamed(timings: &Vec<TimedLightEvent, U2048>) {
    // Iterate over possible unit times from 1 to 5000 ms
    let bestunitmillis: i64 = (1..5000)
        // For each time, score it by summing the scores of the best candidate for each event
        .map(|unit_millis: i64| -> Scored<i64> {
            Scored {
                item: unit_millis,
                score: timings
                    .into_iter()
                    .map(|event| best_error(event, unit_millis).unwrap().score)
                    .sum(),
            }
        })
        // Converge on the minimum scoring unit time
        .fold(
            None,
            |min_so_far: Option<Scored<i64>>, score| match min_so_far {
                Some(m) if m.score < score.score => Some(m),
                _ => Some(score),
            },
        )
        // Ignore possible errors and pull out the best scoring unit time
        .unwrap()
        .item;
}

fn char_to_morse(c: char) -> Morse {
    use Morse::*;
    match c {
        '0' => Dot,
        '1' => Dash,
        _ => Error,
    }
}

fn string_to_code(code: &str) -> Vec<Morse, U8> {
    code.chars().map(char_to_morse).collect()
}

fn ez(o: (&str, &char)) -> (Vec<Morse, U8>, char) {
    match o {
        (str, c) => (string_to_code(str), *c),
    }
}

fn main() -> () {}
