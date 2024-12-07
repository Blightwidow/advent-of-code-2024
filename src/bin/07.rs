use advent_of_code::utils::parse::ParseOps;

advent_of_code::solution!(7);

fn check(result: u64, numbers: &[u64], bonus_op: bool) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == result;
    }

    let mut sum_numbers = vec![numbers[0] + numbers[1]];
    sum_numbers.extend(numbers.iter().skip(2).collect::<Vec<_>>());

    if check(result, &sum_numbers, bonus_op) {
        return true;
    }

    if bonus_op {
        let mut append_numbers = vec![
            numbers[0] * 10_u64.pow(numbers[1].checked_ilog10().unwrap_or(0) + 1) + numbers[1],
        ];
        append_numbers.extend(numbers.iter().skip(2).collect::<Vec<_>>());

        if check(result, &append_numbers, bonus_op) {
            return true;
        }
    }

    let mut product_numbers = vec![numbers[0] * numbers[1]];
    product_numbers.extend(numbers.iter().skip(2).collect::<Vec<_>>());

    check(result, &product_numbers, bonus_op)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;

    for line in input.lines() {
        let mut numbers = line.iter_unsigned::<u64>();
        let result = numbers.next().unwrap();

        if check(result, &numbers.collect::<Vec<_>>(), false) {
            total += result
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;

    for line in input.lines() {
        let mut numbers = line.iter_unsigned::<u64>();
        let result = numbers.next().unwrap();

        if check(result, &numbers.collect::<Vec<_>>(), true) {
            total += result
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
