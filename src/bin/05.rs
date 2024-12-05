advent_of_code::solution!(5);

fn parse(input: &str) -> ([Vec<usize>; 100], Vec<Vec<usize>>) {
    let (couples, lists) = input.split_once("\n\n").unwrap();

    let mut forbidden = [const { Vec::new() }; 100];

    for couple in couples.lines() {
        let (i, j) = couple.split_once("|").unwrap();
        let i: usize = i.parse().unwrap();
        let j: usize = j.parse().unwrap();

        forbidden[i].push(j);
    }

    let mut lines = Vec::new();

    for line in lists.lines() {
        let line = line.split(",").map(|x| x.parse().unwrap()).collect();
        lines.push(line);
    }

    (forbidden, lines)
}

fn check(line: &[usize], forbidden: &[Vec<usize>; 100]) -> usize {
    for (i, &value) in line[1..].iter().enumerate() {
        for j in line.iter().take(i + 1) {
            if forbidden[value].contains(j) {
                return i + 1;
            }
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let (forbidden, lines) = parse(input);

    let mut result = 0;

    for line in lines {
        if check(&line, &forbidden) == 0 {
            result += line[line.len() / 2];
        }
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (forbidden, lines) = parse(input);

    let mut result = 0;
    let mut queue = Vec::new();

    for line in lines {
        let faulty = check(&line, &forbidden);
        if faulty != 0 {
            queue.push(line);
        }
    }

    while let Some(line) = queue.pop() {
        let faulty = check(&line, &forbidden);
        if faulty != 0 {
            let mut list: Vec<usize> = line.clone();
            list.swap(faulty - 1, faulty);
            queue.push(list);
        } else {
            result += line[line.len() / 2];
        }
    }

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
