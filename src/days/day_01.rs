use std::fs::read_to_string;
use std::iter::zip;

pub fn challenge() {
    println!("-- Running Day 1 --");
    let input = read_to_string("inputs/01.txt").expect("Input not found.");
    let lines = input.lines();
    let pairs: Vec<Vec<i32>> = lines
        .map(|x| {
            x.split_whitespace()
                .into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();
    pairs.iter().for_each(|x| {
        list_1.push(x[0]);
        list_2.push(x[1]);
    });
    list_1.sort();
    list_2.sort();
    let mut sum = 0;
    zip(list_1.clone(), list_2.clone()).for_each(|(x, y)| sum += (x - y).abs());
    println!("Part 1: {:?}", sum);
    let mut similarity = 0;
    list_1
        .iter()
        .for_each(|x| similarity += x * list_2.iter().filter(|y| *y == x).count() as i32);
    println!("Part 2: {:?}", similarity);
}
