mod days;

use crate::days::*;
use std::env::args;
use std::time::Instant;

const CHALLENGES: [fn(); 11] = [
    day_01::challenge,
    day_02::challenge,
    day_03::challenge,
    day_04::challenge,
    day_05::challenge,
    day_06::challenge,
    day_07::challenge,
    day_08::challenge,
    day_09::challenge,
    day_10::challenge,
    day_11::challenge,
];

fn main() {
    let day = args().nth(1);
    match day {
        Some(n) => {
            let day = n.parse::<usize>().unwrap();
            if day > CHALLENGES.len() {
                println!("Day {} not yet implemented", n);
            } else {
                let now = Instant::now();
                CHALLENGES[day - 1]();
                println!("{} µs", now.elapsed().as_micros());
            }
        }
        None => {
            CHALLENGES.iter().for_each(|challenge| {
                let now = Instant::now();
                challenge();
                println!("{} µs", now.elapsed().as_micros());
            });
        }
    }
}
