advent_of_code::solution!(2);

fn parse_string(input: &str) -> Vec<Vec<i16>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(level: &[i16]) -> bool {
    (level.windows(2).all(|window| window[0] < window[1])
        || level.windows(2).all(|window| window[0] > window[1]))
        && level
            .windows(2)
            .all(|window| (window[0] - window[1]).abs() <= 3 && (window[0] - window[1]).abs() >= 1)
}

pub fn part_one(input: &str) -> Option<u32> {
    let levels = parse_string(input);

    Some(levels.iter().map(|level| is_safe(level) as u32).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let levels = parse_string(input);

    fn test_level(level: &[i16]) -> bool {
        if is_safe(level) {
            return true;
        }

        for i in 0..level.len() {
            let mut new_level = level.to_owned();
            new_level.remove(i);

            if is_safe(&new_level) {
                return true;
            }
        }

        false
    }

    Some(levels.iter().map(|level| test_level(level) as u32).sum())
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
