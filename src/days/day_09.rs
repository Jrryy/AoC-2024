use std::fs::read_to_string;

pub fn challenge() {
    println!("-- Running Day 09 --");
    let input = read_to_string("inputs/09.txt").unwrap();
    let mut blocks = input
        .strip_suffix("\n")
        .unwrap_or(input.as_str())
        .chars()
        .enumerate()
        .map(|(i, c)| match i % 2 {
            0 => (i as i32 / 2, c.to_digit(10).unwrap()),
            1 => (-1, c.to_digit(10).unwrap()),
            _ => panic!("Impossible"),
        })
        .collect::<Vec<(i32, u32)>>();
    let mut blocks_2 = blocks.clone();
    let mut i = 0;
    while i < blocks.len() {
        let size = blocks.len();
        match blocks[i] {
            (-1, amount) => {
                // [(-1, x), ..., (-1, y)] => [(-1, x), ...]
                if blocks[size - 1].0 == -1 {
                    blocks.pop();
                } else if blocks[size - 1].1 >= amount {
                    // [(-1, x), ..., (1, y)] & y >= x => [(1, x), ..., (1, y - x)]
                    blocks[i].0 = blocks[size - 1].0;
                    blocks[size - 1].1 -= amount;
                    i += 1;
                } else {
                    // [(-1, x), ..., (1, y)] & y < x => [(1, y), (-1, x - y), ...]
                    blocks[i].1 -= blocks[size - 1].1;
                    blocks.insert(i, blocks[size - 1]);
                    i += 1;
                    blocks.pop();
                }
            }
            _ => i += 1,
        }
    }
    let mut file_id: u64 = 0;
    let mut total_1: u64 = 0;
    blocks.iter().for_each(|(i, amount)| {
        if *amount != 0 {
            for _ in 0..*amount {
                total_1 += *i as u64 * file_id;
                file_id += 1;
            }
        }
    });
    println!("Part 1: {}", total_1);

    i = blocks_2.len() - 1;
    while i > 0 {
        match blocks_2[i] {
            (-1, _) => (),
            (x, amount_to_allocate) => {
                let blocks_copy = blocks_2.clone();
                for (j, block) in blocks_copy.iter().enumerate() {
                    if j == i {
                        break;
                    }
                    match block {
                        &(-1, amount_to_fill) => {
                            if amount_to_allocate == amount_to_fill {
                                blocks_2[j].0 = x;
                                blocks_2[i].0 = -1;
                                break;
                            } else if amount_to_allocate < amount_to_fill {
                                blocks_2[i].0 = -1;
                                blocks_2.insert(j, (x, amount_to_allocate));
                                blocks_2[j + 1].1 -= amount_to_allocate;
                                i += 1;
                                break;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        i -= 1;
    }
    let mut file_id: u64 = 0;
    let mut total_2: u64 = 0;
    blocks_2.iter().for_each(|(i, amount)| {
        if *i == -1 {
            file_id += *amount as u64;
        } else if *amount != 0 {
            for _ in 0..*amount {
                total_2 += *i as u64 * file_id;
                file_id += 1;
            }
        }
    });
    println!("Part 2: {}", total_2);
}
