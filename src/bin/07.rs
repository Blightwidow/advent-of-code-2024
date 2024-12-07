use advent_of_code::utils::parse::ParseOps;

advent_of_code::solution!(7);

fn check(target: u64, numbers: &[u64], index: usize, bonus_op: bool) -> bool {
    if index == 1 {
        return numbers[1] == target;
    }

    let mut concatenated_target: u64 = 0;
    let mut remainder: u64 = 0;

    if bonus_op {
        // You can get the multiple with ilog10 but its way more expensive
        // As we know numbers in the input are < 10k we can use 3 ifs
        // It results in a ± 30% speedup (from 194µs to 137µs)
        // let multiplier = ;
        concatenated_target = target / next_power_of_ten(numbers[index]);
        remainder = target % next_power_of_ten(numbers[index]);
    }

    (bonus_op
        && remainder == numbers[index]
        && check(concatenated_target, numbers, index - 1, bonus_op))
        || target % numbers[index] == 0
            && check(target / numbers[index], numbers, index - 1, bonus_op)
        || target > numbers[index] && check(target - numbers[index], numbers, index - 1, bonus_op)
}

#[inline]
fn next_power_of_ten(n: u64) -> u64 {
    if n < 10 {
        10
    } else if n < 100 {
        100
    } else {
        1000
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    let mut equation = Vec::new();

    for line in input.lines() {
        equation.extend(line.iter_unsigned::<u64>());
        let result = equation[0];

        if check(result, &equation, equation.len() - 1, false) {
            total += result
        }

        equation.clear();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    let mut equation = Vec::new();

    for line in input.lines() {
        equation.extend(line.iter_unsigned::<u64>());
        let result = equation[0];

        if check(result, &equation, equation.len() - 1, true) {
            total += result
        }

        equation.clear();
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
