use std::collections::HashMap;
use std::fs::read_to_string;

pub fn challenge() {
    println!("-- Running Day 11 --");
    let input = read_to_string("inputs/11.txt").unwrap();
    let mut stones_map: HashMap<u64, u64> = HashMap::new();
    let mut stones = input
        .strip_suffix("\n")
        .unwrap_or(input.as_str())
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .for_each(|s| stones_map.entry(s).or_insert(1) += 1);

    let mut new_stones: HashMap<u64, u64> = HashMap::new();

    for _ in 0..25 {
        stones_map..for_each(|&s| {
            if s == 0 {
                new_stones.push(1);
            } else {
                let string_s = s.to_string();
                if string_s.len() % 2 == 0 {
                    let split_string_s = string_s.split_at(string_s.len() / 2);
                    new_stones.push(split_string_s.0.parse::<u64>().unwrap());
                    new_stones.push(split_string_s.1.parse::<u64>().unwrap());
                } else {
                    new_stones.push(s * 2024);
                }
            }
        });
        stones = new_stones.clone();
        new_stones = Vec::new();
    }

    println!("Part 1: {}", stones.len());
    println!("Part 2: {}", 0);
}
