use regex::Regex;
use std::sync::LazyLock;

advent_of_code::solution!(3);

static NUMBER_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

fn parse_mul(input: &str) -> u32 {
    let numbers = NUMBER_RE.find_iter(input);

    numbers.fold(1, |acc, number| {
        acc * number.as_str().parse::<u32>().unwrap()
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let hits = mul_re.find_iter(input);

    Some(hits.fold(0, |acc, hit| acc + parse_mul(hit.as_str())))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_re = Regex::new(r"^mul\(\d{1,3},\d{1,3}\)").unwrap();
    let do_re = Regex::new(r"^do[^n][^'][^t]").unwrap();
    let dont_re = Regex::new(r"^don't").unwrap();
    let mut total = 0;
    let mut enabled = true;

    for i in 0..input.len() {
        if do_re.is_match(&input[i..]) {
            enabled = true;
            continue; // Can only match one regex at each iteration
        }

        if dont_re.is_match(&input[i..]) {
            enabled = false;
            continue; // Can only match one regex at each iteration
        }

        if enabled {
            let mul_hit = mul_re.find(&input[i..]);
            if let Some(hit) = mul_hit {
                total += parse_mul(hit.as_str());
            }
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
