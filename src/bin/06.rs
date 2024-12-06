use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::point::*;

use std::str;

const DIRECTIONS: [(u8, Point); 4] = [(b'>', RIGHT), (b'v', DOWN), (b'<', LEFT), (b'^', UP)];

fn process_grid(grid: &mut Grid<u8>) -> bool {
    let mut current_point = grid.find(b'^').unwrap();
    let start = current_point;
    let mut i = 0;

    while grid.contains(current_point) && i < grid.width * grid.height {
        for (i, &(guard_char, dir)) in DIRECTIONS.iter().enumerate() {
            if grid[current_point] == guard_char {
                let next_point = current_point + dir;

                // We are in a loop
                if next_point == start && guard_char == b'^' {
                    return true;
                }

                // If the next point is out of bounds, we are done
                if !grid.contains(next_point) {
                    grid[current_point] = b'X';
                    return false;
                }

                // If the next point is a wall, turn right
                if grid[next_point] == b'#' {
                    grid[current_point] = DIRECTIONS[(i + 1) % 4].0;
                    break;
                }

                // Otherwise, move forward
                grid[current_point] = b'X';
                grid[next_point] = guard_char;
                current_point = next_point;
                break;
            }
        }

        i += 1;
    }

    if i >= grid.width * grid.height {
        return true;
    }

    false
}

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::parse(input);
    process_grid(&mut grid);

    Some(grid.bytes.iter().filter(|&&b| b == b'X').count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::parse(input);
    let start = grid.find(b'^').unwrap();
    process_grid(&mut grid);
    grid[start] = b'^';
    let mut result = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid[Point::new(x, y)] == b'X' {
                let mut next_grid = grid.clone();
                next_grid[Point::new(x, y)] = b'#';
                result += process_grid(&mut next_grid) as u32;
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
