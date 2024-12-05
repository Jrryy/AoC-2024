use std::fs::read_to_string;

pub fn challenge() {
    println!("-- Running Day 03 --");
    let input = read_to_string("inputs/03.txt").unwrap();
    let mut mul_command_pointer: usize = 0;
    let mut first_number = "".to_string();
    let mut second_number = "".to_string();
    let mut after_comma = false;
    let mut total = 0;
    let mut total_2 = 0;
    let mut activated = true;
    let mut parsed_command = "".to_string();
    input.chars().for_each(|c| {
        if mul_command_pointer == 0 {
            // First position: expect an 'm' or a 'd'
            if c == 'm' || c == 'd' {
                mul_command_pointer = 1;
                parsed_command.push(c);
            }
        } else if mul_command_pointer == 1 {
            // Second position: expect an 'u' or an 'o'
            if c == 'u' && parsed_command == "m" {
                mul_command_pointer = 2;
                parsed_command.push(c);
            } else if c == 'o' && parsed_command == "d" {
                mul_command_pointer = 2;
                parsed_command.push(c);
            } else {
                mul_command_pointer = 0;
                parsed_command = "".to_string();
            }
        } else if mul_command_pointer == 2 {
            // Third position: expect an 'l', an 'n' or a '('
            if c == 'l' && parsed_command == "mu" {
                mul_command_pointer = 3;
                parsed_command.push(c);
            } else if parsed_command == "do" && (c == 'n' || c == '(') {
                mul_command_pointer = 3;
                parsed_command.push(c);
            } else {
                mul_command_pointer = 0;
                parsed_command = "".to_string();
            }
        } else if mul_command_pointer == 3 {
            // Fourth position: expect a '(', an '\'' or a ')'
            if c == '(' && parsed_command == "mul" {
                mul_command_pointer = 4;
                parsed_command.push(c);
            } else if c == ')' && parsed_command == "do(" {
                mul_command_pointer = 0;
                activated = true;
                parsed_command = "".to_string();
            } else if c == '\'' && parsed_command == "don" {
                mul_command_pointer = 4;
                parsed_command.push(c);
            } else {
                mul_command_pointer = 0;
                parsed_command = "".to_string();
            }
        } else if mul_command_pointer == 4 {
            if c == 't' && parsed_command == "don'" {
                mul_command_pointer = 5;
            } else if c >= '0' && c <= '9' && parsed_command == "mul(" {
                // If we get a number
                if after_comma {
                    // If we've already found a comma, append it to the second number if it's still
                    // smaller than 1000. Otherwise, reset numbers and command pointer.
                    if second_number.len() == 3 {
                        mul_command_pointer = 0;
                        first_number = "".to_string();
                        second_number = "".to_string();
                        after_comma = false;
                    } else {
                        second_number.push(c);
                    }
                } else {
                    // If we haven't found a comma, append it to the first number if it's still
                    // smaller than 1000. Otherwise, reset command pointer.
                    if first_number.len() == 3 {
                        mul_command_pointer = 0;
                    } else {
                        first_number.push(c);
                    }
                }
            } else if c == ',' && parsed_command == "mul(" {
                // If we find a comma, make sure that we're in the first number and set state to
                // after comma. Otherwise, reset numbers and command pointer.
                if first_number.is_empty() {
                    mul_command_pointer = 0;
                } else if after_comma {
                    mul_command_pointer = 0;
                    after_comma = false;
                    first_number = "".to_string();
                    second_number = "".to_string();
                } else {
                    after_comma = true;
                }
            } else if c == ')' && parsed_command == "mul(" {
                // If we find a ')', make sure that both the numbers are not empty and are smaller
                // than 1000. If so, parse as int, multiply and add them to total. Otherwise,
                // reset pointer, numbers and after comma state.
                if !second_number.is_empty() {
                    let first_number = first_number.parse::<i32>().unwrap();
                    let second_number = second_number.parse::<i32>().unwrap();
                    total += first_number * second_number;
                    if activated {
                        total_2 += first_number * second_number;
                    }
                }
                parsed_command = "".to_string();
                mul_command_pointer = 0;
                after_comma = false;
                first_number = "".to_string();
                second_number = "".to_string();
            } else {
                parsed_command = "".to_string();
                mul_command_pointer = 0;
                after_comma = false;
                first_number = "".to_string();
                second_number = "".to_string();
            }
        } else if mul_command_pointer == 5 {
            if c == '(' {
                mul_command_pointer = 6;
            } else {
                mul_command_pointer = 0;
                parsed_command = "".to_string();
            }
        } else if mul_command_pointer == 6 {
            if c == ')' {
                activated = false;
            }
            mul_command_pointer = 0;
            parsed_command = "".to_string();
        }
    });
    println!("Part 1: {}", total);
    println!("Part 2: {}", total_2);
}
