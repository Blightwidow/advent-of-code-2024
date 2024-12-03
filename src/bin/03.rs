use regex::Regex;
use std::sync::LazyLock;

advent_of_code::solution!(3);

static NUMBER_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Some(
        re
            .captures_iter(input)
            .map(|cap| {
                let a = cap[1].parse::<u32>().unwrap();
                let b = cap[2].parse::<u32>().unwrap();
                a * b
            })
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut total = 0;
    let mut enabled = true;

    for hit in re.find_iter(input) {
        if hit.as_str() == "do()" {
            enabled = true;
        } else if hit.as_str() == "don't()" {
            enabled = false;
        } else if enabled {
            let numbers = NUMBER_RE.find_iter(hit.as_str());
            total += numbers.fold(1, |acc, number| {
                acc * number.as_str().parse::<u32>().unwrap()
            });
        }
    }

    Some(total)
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
