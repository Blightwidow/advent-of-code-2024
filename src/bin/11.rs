use advent_of_code::utils::parse::ParseOps;
use memoize::memoize;

advent_of_code::solution!(11);

#[memoize]
fn iterate_stone(stone: u64, depth: u64, max_depth: u64) -> u64 {
    if depth == max_depth {
        return 1;
    }

    if stone == 0 {
        return iterate_stone(1, depth + 1, max_depth);
    } else if stone.ilog10() % 2 == 1 {
        let num_str = stone.to_string();
        let (first, second) = num_str.split_at(num_str.len() / 2);
        let mut total = 0;
        total += iterate_stone(first.parse::<u64>().unwrap(), depth + 1, max_depth);
        total += iterate_stone(second.parse::<u64>().unwrap(), depth + 1, max_depth);
        return total;
    }

    iterate_stone(stone * 2024, depth + 1, max_depth)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: u64 = 0;

    for stone in input.iter_unsigned::<u64>() {
        total += iterate_stone(stone, 0, 25);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;

    for stone in input.iter_unsigned::<u64>() {
        total += iterate_stone(stone, 0, 75);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
