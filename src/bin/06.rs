use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::point::*;

use std::str;

fn process_grid(grid: &mut Grid<u8>, ghost: bool) -> u32 {
    let mut current_point = grid.find(b'^').unwrap();
    let start = current_point;
    let mut history: Vec<(Point, Point)> = Vec::new();
    let mut direction = UP;
    let mut result = 0;

    fn turn_right(direction: Point) -> Point {
        match direction {
            UP => RIGHT,
            LEFT => UP,
            DOWN => LEFT,
            RIGHT => DOWN,
            _ => panic!("Invalid direction"),
        }
    }

    loop {
        grid[current_point] = b'X';
        let next_point = current_point + direction;

        // If the next point is out of bounds, we are done
        if !grid.contains(next_point) {
            break;
        }

        // If the next point is a wall, turn right
        if grid[next_point] == b'#' {
            history.push((direction, current_point));
            direction = turn_right(direction);
            continue;
        }

        if ghost && next_point != start {
            let mut ghost_history: Vec<(Point, Point)> = Vec::new();
            ghost_history.push((direction, current_point));
            let mut ghost_direction = turn_right(direction);
            let mut ghost_point = current_point;

            while grid.contains(ghost_point) {
                let next_point = ghost_point + ghost_direction;

                // If the next point is out of bounds, we are done
                if !grid.contains(next_point) {
                    break;
                }

                // If the next point is a wall, turn right
                if grid[next_point] == b'#' {
                    if history.contains(&(ghost_direction, ghost_point))
                        || ghost_history.contains(&(ghost_direction, ghost_point))
                    {
                        // We are looping
                        result += 1;
                        break;
                    }

                    ghost_history.push((ghost_direction, ghost_point));
                    ghost_direction = turn_right(ghost_direction);
                    continue;
                }

                ghost_point = next_point;
            }
        }

        // Otherwise, move forward
        current_point = next_point;
    }

    result
}

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::parse(input);
    process_grid(&mut grid, false);

    Some(grid.bytes.iter().filter(|&&b| b == b'X').count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::parse(input);

    Some(process_grid(&mut grid, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
