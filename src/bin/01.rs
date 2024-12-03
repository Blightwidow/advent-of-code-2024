advent_of_code::solution!(1);

fn parse_string(input: &str, sorted: bool) -> (Vec<u32>, Vec<u32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left_list.push(parts.next().unwrap().parse::<u32>().unwrap());
        right_list.push(parts.next().unwrap().parse::<u32>().unwrap());
    }

    if sorted {
        left_list.sort();
        right_list.sort();
    }

    (left_list, right_list)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left_list, right_list) = parse_string(input, true);

    Some(
        left_list
            .iter()
            .enumerate()
            .map(|(i, x)| x.abs_diff(right_list[i]))
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list) = parse_string(input, false);

    Some(
        left_list
            .iter()
            .map(
                |&x|
                    x *
                    (
                        right_list
                            .iter()
                            .filter(|&&y| x == y)
                            .count() as u32
                    )
            )
            .sum()
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
