//! # Day 14: Restroom Redoubt
//!
//! Part can be directly calculated by using the rem euclid function to simulate the
//! movement of the robots. You can directly calculate the position after 100 seconds.
//!
//! For part 2, we can calculate the count of robots on each row and column at each time step.
//! As the positions on each axis loops every `grid_width` and `grid_height` respectively, we just
//! need the first `grid_width` and `grid_height` times steps to every possible positions.
//! Then we can find the first time step where there are more than 20 robots on a row and column.
//!
//! Some people solved it by checking the robots do not stack, but my input failed that approach.

use advent_of_code::utils::{iter::ChunkOps, parse::*};

advent_of_code::solution!(14);

type Robot = [i32; 4];

fn parse(input: &str) -> Vec<Robot> {
    input.iter_signed().chunk::<4>().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = parse(input);
    let grid_width = robots.iter().map(|&r| r[0]).max().unwrap() + 1;
    let grid_height = robots.iter().map(|&r| r[1]).max().unwrap() + 1;
    let mut quadrants = [0_u32; 4];

    for [x, y, dx, dy] in robots {
        let x = (x + 100 * dx).rem_euclid(101);
        let y = (y + 100 * dy).rem_euclid(103);

        if x < grid_width / 2 {
            if y < grid_height / 2 {
                quadrants[0] += 1;
            }
            if y > grid_height / 2 {
                quadrants[1] += 1;
            }
        }
        if x > grid_width / 2 {
            if y < grid_height / 2 {
                quadrants[2] += 1;
            }
            if y > grid_height / 2 {
                quadrants[3] += 1;
            }
        }
    }

    Some(quadrants.iter().product())
}

pub fn part_two(input: &str) -> Option<usize> {
    let robots = parse(input);
    let grid_width = robots.iter().map(|&r| r[0]).max().unwrap() as usize + 1;
    let grid_height = robots.iter().map(|&r| r[1]).max().unwrap() as usize + 1;
    let mut xs = vec![vec![0; grid_width]; grid_width];
    let mut ys = vec![vec![0; grid_height]; grid_height];

    for (time, row) in xs.iter_mut().enumerate() {
        for [x, _, dx, _] in robots.iter() {
            let next_x = (x + dx * time as i32).rem_euclid(grid_width as i32) as usize;
            row[next_x] += 1;
        }
    }

    for (time, row) in ys.iter_mut().enumerate() {
        for [_, y, _, dy] in robots.iter() {
            let next_y = (y + dy * time as i32).rem_euclid(grid_height as i32) as usize;
            row[next_y] += 1;
        }
    }

    for time in 1..grid_height * grid_width {
        if xs[time % grid_width].iter().any(|&row| row > 20)
            && ys[time % grid_height].iter().any(|&row| row > 20)
        {
            return Some(time);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
