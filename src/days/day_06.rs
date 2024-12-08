use std::collections::HashSet;
use std::fs::read_to_string;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn check_loop(
    map: &Vec<Vec<char>>,
    initial_pos: [i32; 2],
    initial_direction: usize,
) -> bool {
    let mut pos = initial_pos;
    let mut direction = initial_direction;
    let mut steps = 0;
    while pos[0] >= 0 && pos[1] >= 0 && pos[0] < map.len() as i32 && pos[1] < map.len() as i32 {
        if map[pos[0] as usize][pos[1] as usize] == '#' {
            pos[0] -= DIRECTIONS[direction].0;
            pos[1] -= DIRECTIONS[direction].1;
            direction = (direction + 1) % 4;
        }
        pos[0] += DIRECTIONS[direction].0;
        pos[1] += DIRECTIONS[direction].1;
        steps += 1;
        if steps > 2 * map.len() * map.len() {
            return true;
        }
    }
    false
}

pub fn challenge() {
    println!("-- Running Day 06 --");
    let input = read_to_string("inputs/06.txt").unwrap();
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut pos: [i32; 2] = [0, 0];
    pos[0] = map
        .iter()
        .enumerate()
        .find(|line| {
            let initial = match line.1.iter().position(|&c| c == '^') {
                Some(x) => x as i32,
                None => -1,
            };
            return if initial < 0 {
                false
            } else {
                pos[1] = initial;
                true
            };
        })
        .unwrap()
        .0 as i32;

    let initial_pos = pos.clone();

    let mut total_1 = 0;
    let mut obstacles: HashSet<[i32; 2]> = HashSet::new();
    let mut visited: HashSet<[i32; 2]> = HashSet::new();
    let mut direction: usize = 0;
    let mut map_2 = map.clone();

    while pos[0] >= 0 && pos[1] >= 0 && pos[0] < map.len() as i32 && pos[1] < map.len() as i32 {
        if map[pos[0] as usize][pos[1] as usize] == '#' {
            pos[0] -= DIRECTIONS[direction].0;
            pos[1] -= DIRECTIONS[direction].1;
            direction = (direction + 1) % 4;
        } else {
            if !visited.contains(&pos) {
                let state = map[pos[0] as usize][pos[1] as usize];
                map_2[pos[0] as usize][pos[1] as usize] = '#';
                if check_loop(&map_2, pos, direction) {
                    obstacles.insert(pos);
                }
                visited.insert(pos);
                map_2[pos[0] as usize][pos[1] as usize] = state;
            }
        }
        if map[pos[0] as usize][pos[1] as usize] != 'X' {
            total_1 += 1;
        }
        map[pos[0] as usize][pos[1] as usize] = 'X';
        pos[0] += DIRECTIONS[direction].0;
        pos[1] += DIRECTIONS[direction].1;
    }
    obstacles.remove(&initial_pos);

    println!("Part 1: {}", total_1);
    println!("Part 2: {}", obstacles.len());
}
