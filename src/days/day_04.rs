use std::fs::read_to_string;

pub fn challenge() {
    println!("-- Running Day 04 --");
    let input = read_to_string("inputs/04.txt").unwrap().lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    let mut total_1 = 0;
    let mut total_2 = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            // Part 1
            if j < input[i].len() - 3 {
                if input[i][j] == 'X' && input[i][j + 1] == 'M' && input[i][j + 2] == 'A' && input[i][j + 3] == 'S' ||
                    input[i][j] == 'S' && input[i][j + 1] == 'A' && input[i][j + 2] == 'M' && input[i][j + 3] == 'X' {
                    total_1 += 1;
                }
            }
            if i < input.len() - 3 {
                if input[i][j] == 'X' && input[i + 1][j] == 'M' && input[i + 2][j] == 'A' && input[i + 3][j] == 'S' ||
                    input[i][j] == 'S' && input[i + 1][j] == 'A' && input[i + 2][j] == 'M' && input[i + 3][j] == 'X' {
                    total_1 += 1;
                }
            }
            if i < input.len() - 3 && j < input[i].len() - 3 {
                if input[i][j] == 'X' && input[i + 1][j + 1] == 'M' && input[i + 2][j + 2] == 'A' && input[i + 3][j + 3] == 'S' ||
                    input[i][j] == 'S' && input[i + 1][j + 1] == 'A' && input[i + 2][j + 2] == 'M' && input[i + 3][j + 3] == 'X' {
                    total_1 += 1;
                }
            }
            if i > 2 && j < input[i].len() - 3 {
                if input[i][j] == 'X' && input[i - 1][j + 1] == 'M' && input[i - 2][j + 2] == 'A' && input[i - 3][j + 3] == 'S' ||
                    input[i][j] == 'S' && input[i - 1][j + 1] == 'A' && input[i - 2][j + 2] == 'M' && input[i - 3][j + 3] == 'X' {
                    total_1 += 1;
                }
            }
            // Part 2
            if input[i][j] == 'A' {
                if i > 0 && i < input.len() - 1 && j > 0 && j < input[i].len() - 1 {
                    if input[i - 1][j - 1] == 'M' && input[i + 1][j - 1] == 'M' && input[i - 1][j + 1] == 'S' && input[i + 1][j + 1] == 'S' ||
                        input[i - 1][j - 1] == 'M' && input[i - 1][j + 1] == 'M' && input[i + 1][j - 1] == 'S' && input[i + 1][j + 1] == 'S' ||
                        input[i + 1][j + 1] == 'M' && input[i + 1][j - 1] == 'M' && input[i - 1][j + 1] == 'S' && input[i - 1][j - 1] == 'S' ||
                        input[i + 1][j + 1] == 'M' && input[i - 1][j + 1] == 'M' && input[i + 1][j - 1] == 'S' && input[i - 1][j - 1] == 'S' {
                        total_2 += 1;
                    }
                }
            }
        }
    }
    println!("Part 1: {}", total_1);
    println!("Part 2: {}", total_2);
}