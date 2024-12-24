use std::collections::HashSet;
use std::fs::read_to_string;

pub fn challenge() {
    println!("-- Running Day 10 --");
    let input = read_to_string("inputs/10.txt").unwrap();
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    map.iter().enumerate().for_each(|(i, row)| {
        (*row)
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == 0)
            .for_each(|(j, _)| trailheads.push((i, j)));
    });
    let mut total_1 = 0;
    trailheads.iter().for_each(|&init_pos| {
        let mut expect = 1;
        let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
        let mut positions: HashSet<(usize, usize)> = HashSet::new();
        positions.insert(init_pos);
        while expect <= 9 && positions.len() > 0 {
            positions.iter().for_each(|&pos| {
                if pos.0 < map.len() - 1 && map[pos.0 + 1][pos.1] == expect {
                    new_positions.insert((pos.0 + 1, pos.1));
                }
                if pos.1 < map.len() - 1 && map[pos.0][pos.1 + 1] == expect {
                    new_positions.insert((pos.0, pos.1 + 1));
                }
                if pos.0 > 0 && map[pos.0 - 1][pos.1] == expect {
                    new_positions.insert((pos.0 - 1, pos.1));
                }
                if pos.1 > 0 && map[pos.0][pos.1 - 1] == expect {
                    new_positions.insert((pos.0, pos.1 - 1));
                }
            });
            positions = new_positions.clone();
            new_positions = HashSet::new();
            expect += 1;
        }
        total_1 += positions.len();
    });

    let mut total_2 = 0;
    trailheads.iter().for_each(|&init_pos| {
        let mut expect = 1;
        let mut new_positions: Vec<(usize, usize)> = Vec::new();
        let mut positions: Vec<(usize, usize)> = Vec::new();
        positions.push(init_pos);
        while expect <= 9 && positions.len() > 0 {
            positions.iter().for_each(|&pos| {
                if pos.0 < map.len() - 1 && map[pos.0 + 1][pos.1] == expect {
                    new_positions.push((pos.0 + 1, pos.1));
                }
                if pos.1 < map.len() - 1 && map[pos.0][pos.1 + 1] == expect {
                    new_positions.push((pos.0, pos.1 + 1));
                }
                if pos.0 > 0 && map[pos.0 - 1][pos.1] == expect {
                    new_positions.push((pos.0 - 1, pos.1));
                }
                if pos.1 > 0 && map[pos.0][pos.1 - 1] == expect {
                    new_positions.push((pos.0, pos.1 - 1));
                }
            });
            positions = new_positions.clone();
            new_positions = Vec::new();
            expect += 1;
        }
        total_2 += positions.len();
    });
    println!("Part 1: {}", total_1);
    println!("Part 2: {}", total_2);
}
