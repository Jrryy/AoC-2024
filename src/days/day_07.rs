use std::fs::read_to_string;

fn is_equation_solvable(numbers: &[i64], partial_result: i64, wanted_result: i64) -> bool {
    if partial_result > wanted_result {
        return false;
    }
    if numbers.is_empty() {
        return partial_result == wanted_result;
    }
    is_equation_solvable(
        &numbers[1..],
        partial_result * numbers[0],
        wanted_result,
    ) || is_equation_solvable(
        &numbers[1..],
        partial_result + numbers[0],
        wanted_result,
    )
}

fn is_equation_solvable_with_concat(
    numbers: &[i64],
    partial_result: i64,
    wanted_result: i64,
) -> bool {
    if partial_result > wanted_result {
        return false;
    }
    if numbers.is_empty() {
        return partial_result == wanted_result;
    }
    is_equation_solvable_with_concat(
        &numbers[1..],
        partial_result * numbers[0],
        wanted_result,
    ) || is_equation_solvable_with_concat(
        &numbers[1..],
        partial_result + numbers[0],
        wanted_result,
    ) || is_equation_solvable_with_concat(
        &numbers[1..],
        partial_result * (10i64.pow(numbers[0].ilog10() + 1)) + numbers[0],
        wanted_result,
    )
}

pub fn challenge() {
    println!("-- Running Day 07 --");
    let input = read_to_string("inputs/07.txt").unwrap();
    let mut total_1 = 0;
    let mut total_2 = 0;
    for line in input.lines() {
        let numbers = line.split(' ').collect::<Vec<&str>>();
        let wanted_result = numbers[0]
            .strip_suffix(":")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let numbers = numbers[1..]
            .iter()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if is_equation_solvable(&numbers[1..], numbers[0], wanted_result) {
            total_1 += wanted_result;
        }
        if is_equation_solvable_with_concat(&numbers[1..], numbers[0], wanted_result) {
            total_2 += wanted_result;
        }
    }
    println!("Part 1: {}", total_1);
    println!("Part 2: {}", total_2);
}
