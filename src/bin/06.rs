use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::hash::{FastSet, FastSetBuilder};
use advent_of_code::utils::point::*;

use std::str;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::parse(input);
    let mut current_point = grid.find(b'^').unwrap();
    let mut direction = UP;
    let mut result = 0;

    while grid.contains(current_point + direction) {
        let next_point = current_point + direction;

        if grid[next_point] == b'#' {
            direction = direction.clockwise();
            continue;
        }

        if grid[next_point] == b'.' {
            result += 1;
            grid[next_point] = b'X';
        }
        // Otherwise, move forward
        current_point += direction;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::parse(input);

    let start = grid.find(b'^').unwrap();
    let mut current_point = start;
    let mut history: FastSet<(Point, Point)> = FastSet::new();
    let mut ghost_history: FastSet<(Point, Point)> = FastSet::new();
    let mut direction = UP;
    let mut result = 0;

    while grid.contains(current_point + direction) {
        grid[current_point] = b'X';
        let next_point = current_point + direction;

        if grid[next_point] == b'#' {
            history.insert((current_point, direction));
            direction = direction.clockwise();
            continue;
        } else if next_point != start && grid[next_point] != b'X' {
            grid[next_point] = b'#';
            ghost_history.clear();
            ghost_history.insert((current_point, direction));
            let mut ghost_direction = direction.clockwise();
            let mut ghost_point = current_point;

            while grid.contains(ghost_point + ghost_direction) {
                if grid[ghost_point + ghost_direction] == b'#' {
                    // If we visited this point before then we are looping
                    if ghost_history.contains(&(ghost_point, ghost_direction))
                        || history.contains(&(ghost_point, ghost_direction))
                    {
                        result += 1;
                        break;
                    }

                    ghost_history.insert((ghost_point, ghost_direction));
                    ghost_direction = ghost_direction.clockwise();
                    continue;
                }

                ghost_point += ghost_direction;
            }

            grid[next_point] = b'.';
        }

        // Otherwise, move forward
        current_point += direction;
    }

    Some(result)
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
