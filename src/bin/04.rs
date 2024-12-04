use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::point::*;

advent_of_code::solution!(4);

// You could probably optimize this with bitboard as it looks a lot like
// line scan for chess pieces.
pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut result = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            let point = Point::new(x, y);

            if grid[point] == b'X' {
                for step in DIAGONAL {
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
