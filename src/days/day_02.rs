use std::fs::read_to_string;

fn check_levels(levels: Vec<i32>) -> Result<(), ()> {
    for i in 1..levels.len() {
        if levels[0] < levels[levels.len()-1] {
            if levels[i - 1] + 3 < levels[i] || levels[i - 1] >= levels[i] {
                return Err(());
            }
        } else {
            if levels[i - 1] - 3 > levels[i] || levels[i - 1] <= levels[i] {
                return Err(());
            }
        }
    }
    Ok(())
}

pub fn challenge() {
    println!("-- Running Day 2 --");
    let mut safe = 0;
    let input = read_to_string("inputs/02.txt").expect("Input not found.");
    let lines = input.lines();
    lines.clone().for_each(|line| {
        let line_numbers = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        match check_levels(line_numbers.clone()) {
            Ok(()) => safe += 1,
            Err(()) => ()
        }
    });
    println!("Part 1: {}", safe);

    safe = 0;
    lines.for_each(|line| {
        let line_numbers = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        match check_levels(line_numbers.clone()) {
            Ok(()) => safe += 1,
            Err(()) => {
                for i in 0..line_numbers.len() {
                    match check_levels([&line_numbers[..i], &line_numbers[i + 1..]].concat()) {
                        Ok(_) => {
                            safe += 1;
                            break
                        },
                        Err(_) => ()
                    }
                }
            }
        }
    });
    println!("Part 2: {}", safe);
}