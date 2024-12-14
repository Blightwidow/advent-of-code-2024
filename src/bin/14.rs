//! # Day 14: Restroom Redoubt
//!
//! Part is is simply to simulate the movement and use modulo to simulate the toroidal grid.
//!
//! Once you have the iteration function, you can solve part 2 by looking at iterations
//! with a large number of robots in the same x and y coordinates.

use advent_of_code::utils::{parse::*, point::*};

advent_of_code::solution!(14);

fn parse(input: &str) -> Vec<(Point, Point)> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let mut data = line.iter_signed::<i32>();

        robots.push((
            Point::new(data.next().unwrap(), data.next().unwrap()),
            Point::new(data.next().unwrap(), data.next().unwrap()),
        ));
    }

    robots
}

fn safe_translation(p: Point, v: Point, grid_width: i32, grid_height: i32) -> Point {
    let mut x = (p.x + v.x) % grid_width;
    let mut y = (p.y + v.y) % grid_height;

    if x < 0 {
        x += grid_width;
    }

    if y < 0 {
        y += grid_height;
    }

    Point::new(x, y)
}

fn iterate(robots: &mut [(Point, Point)], grid_width: i32, grid_height: i32) {
    for (pos, vel) in robots.iter_mut() {
        *pos = safe_translation(*pos, *vel, grid_width, grid_height);
    }
}

// Do not forget to check if the results can fit into u32
pub fn part_one(input: &str) -> Option<u64> {
    let mut robots = parse(input);
    let grid_width = robots.iter().map(|&r| r.0.x).max().unwrap() + 1;
    let grid_height = robots.iter().map(|&r| r.0.y).max().unwrap() + 1;

    for _ in 0..100 {
        iterate(&mut robots, grid_width, grid_height);
    }

    let mut quadrants = [0; 4];

    for robot in robots {
        let (pos, _) = robot;

        if pos.x < grid_width / 2 && pos.y < grid_height / 2 {
            quadrants[0] += 1;
        } else if pos.x > grid_width / 2 && pos.y < grid_height / 2 {
            quadrants[1] += 1;
        } else if pos.x < grid_width / 2 && pos.y > grid_height / 2 {
            quadrants[2] += 1;
        } else if pos.x > grid_width / 2 && pos.y > grid_height / 2 {
            quadrants[3] += 1;
        }
    }

    Some(quadrants.iter().product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut robots = parse(input);
    let grid_width = robots.iter().map(|&r| r.0.x).max().unwrap() + 1;
    let grid_height = robots.iter().map(|&r| r.0.y).max().unwrap() + 1;

    for i in 1..10000 {
        iterate(&mut robots, grid_width, grid_height);

        let count_x = robots
            .iter()
            .fold(vec![0; grid_width as usize], |mut acc, &(pos, _)| {
                acc[pos.x as usize] += 1;
                acc
            });
        let max_x = *count_x.iter().max().unwrap();

        if max_x > 20 {
            let count_y =
                robots
                    .iter()
                    .fold(vec![0; grid_height as usize], |mut acc, &(pos, _)| {
                        acc[pos.y as usize] += 1;
                        acc
                    });
            let max_y = *count_y.iter().max().unwrap();

            if max_y > 20 {
                return Some(i);
            }
        }
    }

    None
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
