advent_of_code::solution!(2);

fn parse_string(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(level: &[i32]) -> bool {
    let mut last_delta: Option<i32> = None;

    for i in 0..level.len() - 1 {
        let delta: i32 = level[i] - level[i + 1];

        if delta.abs() < 1 || delta.abs() > 3 {
            return false;
        }

        if last_delta.is_some() && delta.signum() != last_delta.unwrap().signum() {
            return false;
        }

        last_delta = Some(delta);
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let levels = parse_string(input);

    Some(
        levels
            .iter()
            .fold(0, |acc, level| acc + (is_safe(level) as u32)),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let levels = parse_string(input);

    fn get_substitions(level: &[i32]) -> Vec<Vec<i32>> {
        let mut substitutions = Vec::new();
        substitutions.push(level.to_owned());

        for i in 0..level.len() {
            let mut new_level = level.to_owned();
            new_level.remove(i);
            substitutions.push(new_level);
        }

        substitutions
    }

    fn test_level(level: &[i32]) -> bool {
        for substitution in get_substitions(level) {
            if is_safe(&substitution) {
                return true;
            }
        }

        false
    }

    Some(
        levels
            .iter()
            .fold(0, |acc, level| acc + (test_level(level) as u32)),
    )
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
