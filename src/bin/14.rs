//! # Day 14: Restroom Redoubt
//!
//! Part can be directly calculated by using the rem euclid function to simulate the
//! movement of the robots. You can directly calculate the position after 100 seconds.
//!
//! Once you have the iteration function, you can solve part 2 by looking at iterations
//! with a large number of robots in the same x and y coordinates.

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

pub fn part_two(input: &str) -> Option<u32> {
    let robots = parse(input);
    let grid_width = robots.iter().map(|&r| r[0]).max().unwrap() + 1;
    let grid_height = robots.iter().map(|&r| r[1]).max().unwrap() + 1;

    for i in 1..10_000 {
        let current_robots: Vec<Robot> = robots
            .iter()
            .map(|[x, y, dx, dy]| {
                let x = (x + i as i32 * dx).rem_euclid(101);
                let y = (y + i as i32 * dy).rem_euclid(103);
                [x, y, *dx, *dy]
            })
            .collect();

        let count_x =
            current_robots
                .iter()
                .fold(vec![0; grid_width as usize], |mut acc, &robot| {
                    acc[robot[0] as usize] += 1;
                    acc
                });
        let max_x = *count_x.iter().max().unwrap();

        if max_x > 20 {
            let count_y =
                current_robots
                    .iter()
                    .fold(vec![0; grid_height as usize], |mut acc, &robot| {
                        acc[robot[1] as usize] += 1;
                        acc
                    });
            let max_y = *count_y.iter().max().unwrap();

            if max_y > 20 {
                return Some(i);
            }
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
