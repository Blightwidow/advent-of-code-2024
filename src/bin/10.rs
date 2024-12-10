//! # Day 10: Hoof It

use advent_of_code::utils::{grid::Grid, point::*};

advent_of_code::solution!(10);

fn depth_first_search(
    grid: &Grid<u8>,
    point: Point,
    history: &mut Grid<i32>,
    start_id: i32,
    distinct: bool,
) -> u32 {
    let mut total = 0;

    for next_point in ORTHOGONAL.map(|direction| point + direction) {
        if grid.contains(next_point)
            && grid[next_point] + 1 == grid[point]
            && (distinct || history[next_point] != start_id)
        {
            history[next_point] = start_id;

            if grid[next_point] == b'0' {
                total += 1;
            } else {
                total += depth_first_search(grid, next_point, history, start_id, distinct);
            }
        }
    }

    total
}

// Do not forget to check if the results can fit into u32
pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut history = grid.copy_with(-1);
    let mut total = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == b'9' {
                total += depth_first_search(&grid, point, &mut history, x * grid.width + y, false);
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut history = grid.copy_with(-1);
    let mut total = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == b'9' {
                total += depth_first_search(&grid, point, &mut history, x * grid.width + y, true);
            }
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
