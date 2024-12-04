use regex::{ Regex, RegexBuilder };

advent_of_code::solution!(4);

fn get_xmas_regexes(width: usize) -> Vec<Regex> {
    let mut regexes = vec![Regex::new(r"^XMAS").unwrap(), Regex::new(r"^SAMX").unwrap()];

    for i in [width - 1, width, width + 1].iter() {
        regexes.push(
            RegexBuilder::new(format!(r"^X.{{{}}}M.{{{}}}A.{{{}}}S", i, i, i).as_str())
                .dot_matches_new_line(true)
                .build()
                .unwrap()
        );
        regexes.push(
            RegexBuilder::new(format!(r"^S.{{{}}}A.{{{}}}M.{{{}}}X", i, i, i).as_str())
                .dot_matches_new_line(true)
                .build()
                .unwrap()
        );
    }

    regexes
}

fn get_mas_regexes(width: usize) -> Vec<Regex> {
    let delta = width - 1;

    vec![
        RegexBuilder::new(format!(r"^M.M.{{{}}}A.{{{}}}S.S", delta, delta).as_str())
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
        RegexBuilder::new(format!(r"^S.M.{{{}}}A.{{{}}}S.M", delta, delta).as_str())
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
        RegexBuilder::new(format!(r"^S.S.{{{}}}A.{{{}}}M.M", delta, delta).as_str())
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
        RegexBuilder::new(format!(r"^M.S.{{{}}}A.{{{}}}M.S", delta, delta).as_str())
            .dot_matches_new_line(true)
            .build()
            .unwrap()
    ]
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let width = input.lines().next().unwrap().len();
    let regexes = get_xmas_regexes(width);

    for i in 0..input.len() {
        for regex in regexes.iter() {
            total += regex.is_match(&input[i..]) as u32;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let width = input.lines().next().unwrap().len();
    let regexes = get_mas_regexes(width);

    for i in 0..input.len() {
        for regex in regexes.iter() {
            total += regex.is_match(&input[i..]) as u32;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
