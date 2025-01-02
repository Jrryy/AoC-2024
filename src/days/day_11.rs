use std::collections::HashMap;
use std::fs::read_to_string;

pub fn challenge() {
    println!("-- Running Day 11 --");
    let input = read_to_string("inputs/11.txt").unwrap();
    let mut stones_map: HashMap<u64, u64> = HashMap::new();
    input
        .strip_suffix("\n")
        .unwrap_or(input.as_str())
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .for_each(|s| *stones_map.entry(s).or_default() += 1);

    let mut new_stones: HashMap<u64, u64> = HashMap::new();

    for i in 0..75 {
        stones_map.iter().for_each(|(&k, &v)| {
            if k == 0 {
                *new_stones.entry(1).or_default() += v;
            } else {
                let string_k = k.to_string();
                if string_k.len() % 2 == 0 {
                    let split_string_k = string_k.split_at(string_k.len() / 2);
                    *new_stones
                        .entry(split_string_k.0.parse::<u64>().unwrap())
                        .or_default() += v;
                    *new_stones
                        .entry(split_string_k.1.parse::<u64>().unwrap())
                        .or_default() += v;
                } else {
                    *new_stones.entry(k * 2024).or_default() += v;
                }
            }
        });
        stones_map = new_stones.clone();
        new_stones = HashMap::new();
        if i == 24 {
            println!("Part 1: {}", stones_map.values().sum::<u64>());
        }
    }
    println!("Part 2: {}", stones_map.values().sum::<u64>());
}
