//! # Day 3: Mull It Over
//!
//! We can compute both at the same time.
//! We can just run along the input and parse the numbers.
//! It is faster to just parse each digit and multiply them together.
//! than to try to understand where the number stop and start in the string
//! and convert that to a number.
//!
//! Part two is the exact same as part one, but we need to keep track of
//! the state of the `enabled` flag.

advent_of_code::solution!(3);

fn parse(input: &str) -> (u32, u32) {
    let memory = input.as_bytes();
    let mut index = 0;
    let mut enabled = true;
    let mut part_one = 0;
    let mut part_two = 0;

    while index < memory.len() {
        if memory[index] != b'm' && memory[index] != b'd' {
            index += 1;
            continue;
        }

        if memory[index..].starts_with(b"mul(") {
            index += 4;
        } else if memory[index..].starts_with(b"do()") {
            index += 4;
            enabled = true;
            continue;
        } else if memory[index..].starts_with(b"don't()") {
            index += 7;
            enabled = false;
            continue;
        } else {
            index += 1;
            continue;
        }

        // First number
        let mut first = 0;

        while memory[index].is_ascii_digit() {
            first = 10 * first + (memory[index] - b'0') as u32;
            index += 1;
        }

        // First delimiter
        if memory[index] != b',' {
            continue;
        }
        index += 1;

        // Second number
        let mut second = 0;

        while memory[index].is_ascii_digit() {
            second = 10 * second + (memory[index] - b'0') as u32;
            index += 1;
        }

        // Second delimiter
        if memory[index] != b')' {
            continue;
        }
        index += 1;

        // Multiply
        let product = first * second;
        part_one += product;
        if enabled {
            part_two += product;
        }
    }

    (part_one, part_two)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input).0)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse(input).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
