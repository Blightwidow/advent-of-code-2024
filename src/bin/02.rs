//! # Day 2: Red-Nosed Reports
//!
//! Instead of summing each report and checking if it is valid
//! we can directly compute the deltas with their sign or 0 if invalid.
//!
//! So for example `9 7 6 2 1` would translate `-1 -1 0 -1` and we can check
//! if the absolute sum of the deltas is equal to the size of the report - 1.

use advent_of_code::utils::parse::*;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    fn check(report: &[i32]) -> u32 {
        let size = report.len();
        let score: i32 = (1..size).map(|i| delta(report[i - 1], report[i])).sum();

        if score.abs() == (size - 1) as i32 {
            return 1;
        }

        0
    }

    let mut total = 0;
    let mut report = Vec::new();
    for line in input.lines() {
        report.extend(line.iter_signed::<i32>());

        total += check(&report);

        report.clear();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    fn check(report: &[i32]) -> u32 {
        let size = report.len();
        let score: i32 = (1..size).map(|i| delta(report[i - 1], report[i])).sum();

        if score.abs() == (size - 1) as i32 {
            return 1;
        }

        for i in 0..size {
            let mut score = score;

            // Snip out each level and replace with new level computed from neighbors to either side.
            if i > 0 {
                score -= delta(report[i - 1], report[i]);
            }
            if i < size - 1 {
                score -= delta(report[i], report[i + 1]);
            }
            if i > 0 && i < size - 1 {
                score += delta(report[i - 1], report[i + 1]);
            }

            if score.abs() == (size - 2) as i32 {
                return 1;
            }
        }

        0
    }

    let mut total = 0;
    let mut report = Vec::new();
    for line in input.lines() {
        report.extend(line.iter_signed::<i32>());

        total += check(&report);

        report.clear();
    }

    Some(total)
}

/// Convert each pair of levels to either +1 for increase, -1 for decrease or 0 for invalid range.
fn delta(a: i32, b: i32) -> i32 {
    let diff = b - a;

    if diff.abs() <= 3 {
        diff.signum()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
