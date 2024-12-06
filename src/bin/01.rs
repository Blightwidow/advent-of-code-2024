use advent_of_code::utils::iter::*;
use advent_of_code::utils::parse::*;

advent_of_code::solution!(1);

fn parse_string(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .iter_unsigned::<u32>()
        .chunk::<2>()
        .map(|[x, y]| (x, y))
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = parse_string(input);

    left_list.sort_unstable();
    right_list.sort_unstable();

    Some(
        left_list
            .iter()
            .zip(right_list)
            .map(|(x, y)| x.abs_diff(y))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list) = parse_string(input);

    Some(
        left_list
            .iter()
            .map(|&x| x * (right_list.iter().fold(0, |acc, &y| acc + (x == y) as u32)))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
