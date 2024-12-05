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

    for i in 0..grid.width {
        result += scan_line(&grid, Point::new(i, 0), DOWN, grid.height);
    }
    for i in 0..grid.height {
        result += scan_line(&grid, Point::new(0, i), RIGHT, grid.width);
    }

    for x in 0..grid.width {
        for y in 0..grid.height {
            let point = Point::new(x, y);

            if grid[point] == b'X' {
                for &step in [DOWN + RIGHT, DOWN + LEFT, UP + RIGHT, UP + LEFT].iter() {
                    result += (grid.contains(point + step * 3)
                        && grid[point + step] == b'M'
                        && grid[point + step * 2] == b'A'
                        && grid[point + step * 3] == b'S') as u32;
                }
            }
        }
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
                let ul = grid[point + Point::new(-1, -1)];
                let ur = grid[point + Point::new(1, -1)];
                let dl = grid[point + Point::new(-1, 1)];
                let dr = grid[point + Point::new(1, 1)];

                let horizontal = ul == ur && dl == dr && ul.abs_diff(dl) == 6;
                let vertical = ul == dl && ur == dr && ul.abs_diff(ur) == 6;

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
