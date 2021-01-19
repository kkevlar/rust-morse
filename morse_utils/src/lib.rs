#![no_std]


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

pub mod morse_utils {

    extern crate heapless;

    use core::convert::TryFrom;
    use heapless::consts::*;
    use heapless::FnvIndexMap;
    use heapless::Vec;

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum LightState {
        Light,
        Dark,
    }

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum MorseErr {
        TooFewTLEs,
    }

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct Scored<T> {
        pub item: T,
        pub score: i64,
    }

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct TimedLightEvent {
        pub light_state: LightState,
        pub duration: i64,
    }

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct MorseCandidate {
        pub light_state: LightState,
        pub units: i64,
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

    pub fn calc_error(
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

    fn poisoned_min<T>(
        min_so_far: Option<Result<Scored<T>, MorseErr>>,
        next: Result<Scored<T>, MorseErr>,
    ) -> Option<Result<Scored<T>, MorseErr>> {
        Some(match (min_so_far, next) {
            (None, next) => next,
            (Some(Err(prev_error)), _) => Err(prev_error),
            (Some(Ok(_)), Err(next_error)) => Err(next_error),
            (Some(Ok(msf)), Ok(next)) => {
                if msf.score < next.score {
                    Ok(msf)
                } else {
                    Ok(next)
                }
            }
        })
    }

    pub fn best_error(
        event: &TimedLightEvent,
        unit_millis: i64,
    ) -> Result<Scored<&MorseCandidate>, MorseErr> {
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
            .filter_map(|s| s)
            .min_by_key(|s| s.score)
            .ok_or(MorseErr::TooFewTLEs)
    }

    pub fn score_possible_unit_millis(
        unit_millis: i64,
        timings: &[TimedLightEvent],
    ) -> Result<Scored<i64>, MorseErr> {
        let score: i64 = timings
            .iter()
            .map(|event| -> Result<i64, MorseErr> { Ok(best_error(event, unit_millis)?.score) })
            .fold(Ok(0), |l, r| Ok(l? + r?))?;
        Ok(Scored {
            item: unit_millis,
            score,
        })
    }

    pub fn estimate_unit_time(
        timings: &[TimedLightEvent],
        min_millis: i64,
        max_millis: i64,
    ) -> Result<Scored<i64>, MorseErr> {
        // Iterate over possible unit times from 1 to 5000 ms
        (min_millis..max_millis)
            // For each time, score it by summing the scores of the best candidate for each event
            .map(|unit_millis| {
                score_possible_unit_millis(unit_millis, timings) 
            })
            // Converge on the minimum scoring unit time
            .fold(None, poisoned_min)
            // Ignore possible errors and pull out the best scoring unit time
            .unwrap_or(Err(MorseErr::TooFewTLEs))
    }

    fn fill_unit_time_possibilities() {
        for i in 1..100 {
            let i: f32 = i as f32;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calc_error_spoton() {
            assert_eq!(
                0,
                calc_error(
                    &TimedLightEvent {
                        light_state: LightState::Dark,
                        duration: 600,
                    },
                    &MorseCandidate {
                        light_state: LightState::Dark,
                        units: 3,
                    },
                    200
                )
                .unwrap()
            );
        }

        #[test]
        fn test_calc_error_confused() {
            assert_eq!(
                200,
                calc_error(
                    &TimedLightEvent {
                        light_state: LightState::Light,
                        duration: 300,
                    },
                    &MorseCandidate {
                        light_state: LightState::Light,
                        units: 1,
                    },
                    100
                )
                .unwrap()
            );
        }

        fn best_error_helper(light_state: LightState, duration: i64, units: i64) -> i64 {
            best_error(
                &TimedLightEvent {
                    light_state,
                    duration,
                },
                units,
            )
            .unwrap()
            .score
        }

        #[test]
        fn test_best_error() {
            use super::LightState::*;

            assert_eq!(100, best_error_helper(Dark, 200, 100));
            assert_eq!(80, best_error_helper(Dark, 180, 100));
            assert_eq!(50, best_error_helper(Dark, 50, 100));
            assert_eq!(100, best_error_helper(Dark, 0, 100));
            assert_eq!(1, best_error_helper(Dark, 701, 100));
            assert_eq!(1, best_error_helper(Dark, 6, 1));

            assert_eq!(200, best_error_helper(Light, 800, 200));
            assert_eq!(400, best_error_helper(Light, 700, 100));
            assert_eq!(1000, best_error_helper(Light, 0, 1000));
            assert_eq!(100, best_error_helper(Light, 200, 100));
            assert_eq!(2, best_error_helper(Light, 1502, 500));
            assert_eq!(0, best_error_helper(Light, 75, 25));
        }

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

        #[test]
        fn test_estimate() {
            let test_durations = [
                700, 300, 100, 100, 100, 100, 100, 100, 300, 300, 100, 300, 100, 300, 300, 100,
                100, 100, 100, 300, 300, 300, 300, 300, 300, 100, 300, 300, 300, 100, 100, 700,
                300, 100, 300, 100, 300, 300, 300, 100, 300, 100, 300, 300, 100, 100, 100, 100,
                300, 100, 100, 700,
            ];
            let mut timed_light_events: Vec<TimedLightEvent, U128> = Vec::new();
            helper_fill_events_slice(&test_durations, &mut timed_light_events);
            assert_eq!(
                Scored {
                    item: 100,
                    score: 0
                },
                estimate_unit_time(&timed_light_events, 0, 10000).unwrap()
            );
        }
    }
}

// fn char_to_morse(c: char) -> Morse {
//     use Morse::*;
//     match c {
//         '0' => Dot,
//         '1' => Dash,
//         _ => Error,
//     }
// }

// fn string_to_code(code: &str) -> Vec<Morse, U8> {
//     code.chars().map(char_to_morse).collect()
// }

// fn ez(o: (&str, &char)) -> (Vec<Morse, U8>, char) {
//     match o {
//         (str, c) => (string_to_code(str), *c),
//     }
// }

fn main() -> () {}
