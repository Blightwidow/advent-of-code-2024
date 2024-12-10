//! # Day 10: Hoof It

use advent_of_code::utils::{grid::Grid, hash::*, point::*};

advent_of_code::solution!(10);

fn check_ends(grid: &Grid<u8>, point: Point, ends: &mut FastSet<Point>) {
    if grid[point] == b'9' {
        ends.insert(point);
        return;
    }

    for direction in ORTHOGONAL {
        let next_point = point + direction;
        if grid.contains(next_point) && grid[point] + 1 == grid[next_point] {
            check_ends(grid, next_point, ends);
        }
    }
}

fn check_paths(grid: &Grid<u8>, point: Point, solution: &mut Grid<u32>) {
    if grid[point] == b'9' {
        solution[point] += 1;
        return;
    }

    for direction in ORTHOGONAL {
        let next_point = point + direction;
        if grid.contains(next_point) && grid[point] + 1 == grid[next_point] {
            check_paths(grid, next_point, solution);
        }
    }
}

// Do not forget to check if the results can fit into u32
pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut ends: FastSet<Point> = FastSet::new();
    let mut total = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == b'0' {
                check_ends(&grid, point, &mut ends);
                total += ends.len() as u32;
                ends.clear();
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut solution = grid.copy_with(0u32);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == b'0' {
                check_paths(&grid, point, &mut solution);
            }
        }
    }

    Some(solution.bytes.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
