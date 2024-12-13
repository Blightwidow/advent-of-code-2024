//! # Day 13: Claw Contraption
//!
//! Nothing peculiar here. Just a simple system of linear equations.
//! The types are chosen to be `f64` to avoid overflow in part two.

use advent_of_code::utils::parse::ParseOps;

advent_of_code::solution!(13);

const A_COST: u64 = 3;
const B_COST: u64 = 1;

type Machine = Vec<f64>;

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();

    for block in input.split("\n\n") {
        machines.push(
            block
                .iter_unsigned::<u32>()
                .map(|x| x as f64)
                .collect::<Vec<_>>(),
        );
    }

    machines
}

fn solve(machine: &Machine, max_presses: f64, offset: f64) -> u64 {
    let [a, j, b, k, mut c, mut l] = machine[..] else {
        panic!("Invalid input")
    };

    // For part two
    c += offset;
    l += offset;

    let x = (c * k - b * l) / (a * k - b * j);
    let y = (c * j - a * l) / (b * j - a * k);

    if x % 1.0 == 0.0 && y % 1.0 == 0.0 && x <= max_presses && y <= max_presses {
        return x.trunc() as u64 * A_COST + y.trunc() as u64 * B_COST;
    }

    0
}

// Do not forget to check if the results can fit into u32
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .map(|machine| solve(machine, 100.0, 0.0))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .map(|machine| solve(machine, f64::MAX, 10000000000000.0))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
