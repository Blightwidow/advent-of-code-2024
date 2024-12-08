//! # Day 8: Resonant Collinearity
//!
//! In order to avoid searching the string constantly for antenna of the same values,
//! we can directly store the antennas in a hashmap and just iterates on groups of antennas.

use advent_of_code::utils::{
    grid::Grid,
    hash::{FastMap, FastMapBuilder, FastSet, FastSetBuilder},
    point::Point,
};

advent_of_code::solution!(8);

fn parse(input: &str) -> (Grid<u8>, FastMap<u8, Vec<Point>>) {
    let grid = Grid::parse(input);
    let mut antennas: FastMap<u8, Vec<Point>> = FastMap::new();

    for point in grid.find_all(|c| c != b'.') {
        antennas.entry(grid[point]).or_default().push(point);
    }

    (grid, antennas)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, antennas) = parse(input);
    let mut antinodes: FastSet<Point> = FastSet::new();

    for points in antennas.values() {
        for &first_antenna in points {
            for &second_antenna in points {
                if first_antenna != second_antenna {
                    let antinode = second_antenna * 2 - first_antenna;
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
    let (grid, antennas) = parse(input);
    let mut antinodes: FastSet<Point> = FastSet::new();

    for points in antennas.values() {
        for &first_antenna in points {
            for &second_antenna in points {
                if first_antenna != second_antenna {
                    let delta = second_antenna - first_antenna;
                    let mut next_point = second_antenna;

                    while grid.contains(next_point) {
                        antinodes.insert(next_point);
                        next_point += delta;
                    }
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
