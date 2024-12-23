use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn challenge() {
    println!("-- Running Day 08 --");
    let input = read_to_string("inputs/08.txt").unwrap();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes_1: HashSet<(i32, i32)> = HashSet::new();
    let mut antinodes_2: HashSet<(i32, i32)> = HashSet::new();
    let size = input.lines().count() as i32;
    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, ch)| match ch {
            '.' => (),
            c => {
                antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((x as i32, y as i32));
                antinodes_2.insert((x as i32, y as i32));
            }
        })
    });


    antennas.values().for_each(|positions| {
        positions[..positions.iter().len() - 1]
            .iter()
            .enumerate()
            .for_each(|(left, pos_1)| {
                positions[left + 1..].iter().for_each(|pos_2| {
                    let vector = (pos_1.0 - pos_2.0, pos_1.1 - pos_2.1);
                    let antinode = (pos_1.0 + vector.0, pos_1.1 + vector.1);
                    if !positions.contains(&antinode)
                        && antinode.0 >= 0
                        && antinode.1 >= 0
                        && antinode.0 < size
                        && antinode.1 < size
                    {
                        antinodes_1.insert(antinode);
                    }
                    let antinode = (pos_2.0 - vector.0, pos_2.1 - vector.1);
                    if !positions.contains(&antinode)
                        && antinode.0 >= 0
                        && antinode.1 >= 0
                        && antinode.0 < size
                        && antinode.1 < size
                    {
                        antinodes_1.insert(antinode);
                    }
                    let mut new_antinode = (pos_1.0 + vector.0, pos_1.1 + vector.1);
                    while new_antinode.0 >= 0
                        && new_antinode.1 >= 0
                        && new_antinode.0 < size
                        && new_antinode.1 < size
                    {
                        antinodes_2.insert(new_antinode);
                        new_antinode.0 += vector.0;
                        new_antinode.1 += vector.1;
                    }
                    new_antinode = (pos_1.0 - vector.0, pos_1.1 - vector.1);
                    while new_antinode.0 >= 0
                        && new_antinode.1 >= 0
                        && new_antinode.0 < size
                        && new_antinode.1 < size
                    {
                        antinodes_2.insert(new_antinode);
                        new_antinode.0 -= vector.0;
                        new_antinode.1 -= vector.1;
                    }
                })
            })
    });

    println!("Part 1: {}", antinodes_1.len());
    println!("Part 2: {}", antinodes_2.len());
}
