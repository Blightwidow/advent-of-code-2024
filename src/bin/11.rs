//! # Day 11: Plutonian Pebbles
//!
//! While we can brute force the solution for the first part, the second part is impossible to solve without a cache.
//!
//! The cache is a map of the form `(u64, u64) -> u64` where the first `u64` is the stone and the second `u64` is the number of blinks left to do.
//! We can simply populate the cache as we go deeper in the recursion.
//!
//! It might be possible to prewarm the cache in parallel to solve

use advent_of_code::utils::{hash::*, parse::ParseOps};

advent_of_code::solution!(11);

fn count(cache: &mut FastMap<(u64, u64), u64>, stone: u64, blinks: u64) -> u64 {
    if blinks == 1 {
        if stone == 0 {
            return 1;
        }

        if stone.ilog10() % 2 == 1 {
            return 2;
        } else {
            return 1;
        };
    }

    let cache_key = (stone, blinks);

    if let Some(&value) = cache.get(&cache_key) {
        return value;
    }

    let next_value: u64;

    if stone == 0 {
        next_value = count(cache, 1, blinks - 1)
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let exponant = 10_u64.pow(digits / 2);
            next_value = count(cache, stone / exponant, blinks - 1)
                + count(cache, stone % exponant, blinks - 1)
        } else {
            next_value = count(cache, stone * 2024, blinks - 1)
        }
    };

    cache.insert(cache_key, next_value);

    next_value
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut cache: FastMap<(u64, u64), u64> = FastMap::with_capacity(3_100);

    Some(
        input
            .iter_unsigned::<u64>()
            .map(|stone| count(&mut cache, stone, 25))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cache: FastMap<(u64, u64), u64> = FastMap::with_capacity(128_000);

    Some(
        input
            .iter_unsigned::<u64>()
            .map(|stone| count(&mut cache, stone, 75))
            .sum(),
    )
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
