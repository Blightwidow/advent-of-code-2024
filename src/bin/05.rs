use std::cmp::Ordering::{self, *};

advent_of_code::solution!(5);

fn parse(input: &str) -> ([[Ordering; 100]; 100], Vec<Vec<usize>>) {
    let (couples, lists) = input.split_once("\n\n").unwrap();
    let mut order = [[Equal; 100]; 100];

    for couple in couples.lines() {
        let (i, j) = couple.split_once("|").unwrap();
        let i: usize = i.parse().unwrap();
        let j: usize = j.parse().unwrap();

        order[i][j] = Less;
        order[j][i] = Greater;
    }

    let mut lines = Vec::new();

    for line in lists.lines() {
        let line = line.split(",").map(|x| x.parse().unwrap()).collect();
        lines.push(line);
    }

    (order, lines)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (order, lines) = parse(input);

    let mut result = 0;

    for line in lines {
        let middle = line.len() / 2;

        if line.is_sorted_by(|&from, &to| order[from][to] == Less) {
            result += line[middle];
        }
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (order, lines) = parse(input);

    let mut result = 0;
    let mut update = Vec::new();

    for line in lines {
        let middle = line.len() / 2;
        update.clear();
        update.extend(line);

        if !update.is_sorted_by(|&from, &to| order[from][to] == Less) {
            update.select_nth_unstable_by(middle, |&from, &to| order[from][to]);
            result += update[middle];
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
