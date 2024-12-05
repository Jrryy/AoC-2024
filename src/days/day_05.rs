use std::collections::HashMap;
use std::fs::read_to_string;

pub fn challenge() {
    println!("-- Running Day 05 --");
    let input = read_to_string("inputs/05.txt").unwrap();
    let split_input = input.split("\n\n").collect::<Vec<&str>>();
    let rules = split_input[0].lines().collect::<Vec<&str>>();
    let updates = split_input[1].lines().collect::<Vec<&str>>();

    let mut rule_map: HashMap<&str, Vec<&str>> = HashMap::new();
    rules.iter().for_each(|&rule| {
        let split_rule = rule.split('|').collect::<Vec<&str>>();
        rule_map.entry(split_rule[1]).or_insert(Vec::new()).push(split_rule[0]);
    });

    let mut total_1 = 0;
    let mut total_2 = 0;

    updates.iter().for_each(|&raw_update| {
        let mut required: Vec<&str> = Vec::new();
        let split_updates = raw_update.split(',').collect::<Vec<&str>>();
        let mut fixed_updates: Vec<&str> = Vec::new();
        let mut correct = true;
        split_updates.iter().for_each(|&update| {
            if required.contains(&update) {
                correct = false;
                let wrong_pos = fixed_updates.iter().position(|&x| rule_map.entry(&x).or_default().contains(&update)).unwrap();
                fixed_updates.insert(wrong_pos, update);
            } else {
                fixed_updates.push(update);
            }
            required.extend(rule_map.entry(update).or_default().clone());
        });
        if correct {
            total_1 += split_updates[split_updates.len() / 2].parse::<i32>().unwrap();
        } else {
            total_2 += fixed_updates[fixed_updates.len() / 2].parse::<i32>().unwrap();
        }
    });

    println!("Part 1: {}", total_1);
    println!("Part 2: {}", total_2);
}