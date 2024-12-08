//! # Day 8: Resonant Collinearity

use advent_of_code::utils::{
    grid::Grid,
    hash::{FastSet, FastSetBuilder},
    point::Point,
};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut antinodes: FastSet<Point> = FastSet::new();

    for point in grid.find_all(|c| c != b'.') {
        for other_antenna in grid.find_all(|c| c == grid[point]) {
            if point != other_antenna {
                for antinode in [other_antenna * 2 - point, point * 2 - other_antenna] {
                    if grid.contains(antinode) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut antinodes: FastSet<Point> = FastSet::new();

    for point in grid.find_all(|c| c != b'.') {
        for other_antenna in grid.find_all(|c| c == grid[point]) {
            if point != other_antenna {
                let mut delta = other_antenna - point;
                let mut next_point = other_antenna;

                while grid.contains(next_point) {
                    antinodes.insert(next_point);
                    next_point += delta;
                }

                delta = point - other_antenna;
                next_point = point;

                while grid.contains(next_point) {
                    antinodes.insert(next_point);
                    next_point += delta;
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
