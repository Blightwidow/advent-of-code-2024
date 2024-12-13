use regex::Regex;

advent_of_code::solution!(13);

const A_COST: u64 = 3;
const B_COST: u64 = 1;

type Button = (f64, f64);
type Machine = (Button, Button, Button);

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    let number_regex: Regex = Regex::new(r"(\d+)[^\d]+(\d+)").unwrap();

    for block in input.split("\n\n") {
        let mut buttons: Vec<Button> = Vec::new();
        for line in block.lines() {
            let captures = number_regex.captures(line).unwrap();
            buttons.push((
                captures[1].parse::<f64>().unwrap(),
                captures[2].parse::<f64>().unwrap(),
            ));
        }
        machines.push((buttons[0], buttons[1], buttons[2]));
    }

    machines
}

fn solve(machine: Machine, max_presses: f64, offset: f64) -> u64 {
    let (a, j) = machine.0;
    let (b, k) = machine.1;
    let (mut c, mut l) = machine.2;

    // For part two
    c += offset;
    l += offset;

    let x = (c * k - b * l) / (a * k - b * j);
    let y = (c * j - a * l) / (b * j - a * k);

    if x % 1.0 == 0.0 && y % 1.0 == 0.0 && x <= max_presses && y <= max_presses {
        return x.trunc() as u64 * A_COST + y.trunc() as u64 * B_COST;
    }

    0
}

// Do not forget to check if the results can fit into u32
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .map(|machine| solve(*machine, 100.0, 0.0))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .map(|machine| solve(*machine, f64::MAX, 10000000000000.0))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
