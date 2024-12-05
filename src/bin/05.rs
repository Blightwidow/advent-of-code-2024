use advent_of_code::utils::iter::*;
use advent_of_code::utils::parse::*;

use std::cmp::Ordering::{self, *};

advent_of_code::solution!(5);

fn parse(input: &str) -> ([[Ordering; 100]; 100], &str) {
    let (couples, lists) = input.split_once("\n\n").unwrap();
    let mut order = [[Equal; 100]; 100];

    let test = couples.iter_unsigned::<usize>().chunk::<2>();
    for [i, j] in test {
        order[i][j] = Less;
        order[j][i] = Greater;
    }

    (order, lists)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (order, lists) = parse(input);

    let mut result = 0;
    let mut report = Vec::new();

    for line in lists.lines() {
        report.clear();
        report.extend(line.iter_unsigned::<usize>());
        let middle = report.len() / 2;

        if report.is_sorted_by(|&from, &to| order[from][to] == Less) {
            result += report[middle];
        }
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (order, lists) = parse(input);

    let mut result = 0;
    let mut report = Vec::new();

    for line in lists.lines() {
        report.clear();
        report.extend(line.iter_unsigned::<usize>());
        let middle = report.len() / 2;

        if !report.is_sorted_by(|&from, &to| order[from][to] == Less) {
            report.select_nth_unstable_by(middle, |&from, &to| order[from][to]);
            result += report[middle];
        }
    }

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
