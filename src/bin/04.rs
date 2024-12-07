//! # Day 4: Ceres Search
//!
//! In part one, we use a bitboard to store the last 4 bytes of the grid
//! as the targets are 4 bytes long. We then scan the grid in all directions
//! and check if the bytes match the targets.
//! We do not need to scan all diagonals as we test for both the "XMAS" and "SAMX"
//! at the same time.
//!
//! In part two, instead of scanning, we can simply check every "A" in the grid.
//! Instead of checking if the bytes around it are "S" and "M" we can check if the
//! bytes are the same either vertically or horizontally and the distance is 6 (as
//! it can be the only combination with the same distance in the "X", "M", "A", "S"
//! set).

use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::point::*;

advent_of_code::solution!(4);

// You could probably optimize this with bitboard for diagonal as well
pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut result = 0;

    fn scan_line(grid: &Grid<u8>, mut point: Point, step: Point, size: i32) -> u32 {
        let mut bytes = 0;
        let mut result = 0;

        for _ in 0..size {
            // Shift the bytes to the left and add the new byte.
            // Elminiate the first to limit the size at 4 bytes thanks to the type u32.
            bytes = (bytes << 8) | (grid[point] as u32);
            point += step;
            // "XMAS" and "SAMX" in hex.
            result += (bytes == 0x584d4153 || bytes == 0x53414d58) as u32;
        }

        result
    }

    let size = grid.height;
    for i in 0..size {
        result += scan_line(&grid, Point::new(i, 0), DOWN, grid.height);
        result += scan_line(&grid, Point::new(0, i), RIGHT, grid.width);
    }

    for i in 0..size - 3 {
        result += scan_line(&grid, Point::new(i, 0), DOWN_RIGHT, size - i);
        result += scan_line(&grid, Point::new(0, i + 1), DOWN_RIGHT, size - i - 1);
        result += scan_line(&grid, Point::new(size - i - 1, 0), DOWN_LEFT, size - i);
        result += scan_line(&grid, Point::new(size - 1, i + 1), DOWN_LEFT, size - i - 1);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut result = 0;

    for x in 1..grid.width - 1 {
        for y in 1..grid.height - 1 {
            let point = Point::new(x, y);

            if grid[point] == b'A' {
                let upper_left = grid[point + Point::new(-1, -1)];
                let upper_right = grid[point + Point::new(1, -1)];
                let down_left = grid[point + Point::new(-1, 1)];
                let down_right = grid[point + Point::new(1, 1)];

                let horizontal = upper_left == upper_right
                    && down_left == down_right
                    && upper_left.abs_diff(down_left) == 6;
                let vertical = upper_left == down_left
                    && upper_right == down_right
                    && upper_left.abs_diff(upper_right) == 6;

                result += (horizontal || vertical) as u32;
            }
        }
    }

    Some(result)
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
