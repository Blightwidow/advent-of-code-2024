//! # Day 7: Bridge Repair
//!
//! I spent more than I would like to admit figuring out the solutiong required a u64.
//!
//! Instead of brute forcing every possible combination through a recursive function
//! We can check from the last number and eliminate impossible states as we go.
//!
//! We can compute both at the same time by adding a flag to the check function
//! to enable the third operator in part two.
//! The key to performance here is:
//! 1. Just use the index instead of creating and passing a new vector for each iteration
//! 2. Avoiding the use of `ilog10` to get the multiplier (~30% more performance)

use advent_of_code::utils::parse::ParseOps;

advent_of_code::solution!(7);

/// Check if a target can be reached with the given numbers recursively
fn check(target: u64, numbers: &[u64], index: usize, bonus_op: bool) -> bool {
    if index == 1 {
        return numbers[1] == target;
    }

    let mut concatenated_target: u64 = 0;
    let mut remainder: u64 = 0;

    if bonus_op {
        // You can get the multiple with ilog10 but its way more expensive
        // As we know numbers in the input are < 10k we can use 3 ifs
        // It results in a ± 30% speedup (from 194µs to 137µs)
        // let multiplier = ;
        concatenated_target = target / exponant(numbers[index]);
        remainder = target % exponant(numbers[index]);
    }

    (bonus_op
        && remainder == numbers[index]
        && check(concatenated_target, numbers, index - 1, bonus_op))
        || target % numbers[index] == 0
            && check(target / numbers[index], numbers, index - 1, bonus_op)
        || target > numbers[index] && check(target - numbers[index], numbers, index - 1, bonus_op)
}

/// Limited to 3 exponants but faster than using ilog10 in this case
#[inline]
fn exponant(n: u64) -> u64 {
    if n < 10 {
        10
    } else if n < 100 {
        100
    } else {
        1000
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    let mut equation = Vec::new();

    for line in input.lines() {
        equation.extend(line.iter_unsigned::<u64>());
        let result = equation[0];

        if check(result, &equation, equation.len() - 1, false) {
            total += result
        }

        equation.clear();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    let mut equation = Vec::new();

    for line in input.lines() {
        equation.extend(line.iter_unsigned::<u64>());
        let result = equation[0];

        if check(result, &equation, equation.len() - 1, true) {
            total += result
        }

        equation.clear();
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
