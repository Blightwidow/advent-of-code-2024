use advent_of_code::utils::{grid::Grid, point::*};

advent_of_code::solution!(12);

fn get_regions(grid: &Grid<u8>) -> Vec<Vec<Point>> {
    let mut seen = vec![vec![false; grid.height as usize]; grid.width as usize];
    let mut regions = Vec::new();

    for x in 0..grid.width {
        for y in 0..grid.height {
            if seen[y as usize][x as usize] {
                continue;
            }

            let mut region = Vec::new();
            let mut queue = vec![Point::new(x, y)];

            while let Some(point) = queue.pop() {
                if seen[point.y as usize][point.x as usize] {
                    continue;
                }

                region.push(point);
                seen[point.y as usize][point.x as usize] = true;

                for next_point in ORTHOGONAL.map(|dir| point + dir) {
                    if grid.contains(next_point) && grid[next_point] == grid[point] {
                        queue.push(next_point);
                    }
                }
            }

            regions.push(region);
        }
    }

    regions
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let regions = get_regions(&grid);
    let mut total: u32 = 0;

    for region in regions {
        let mut perimeter = 0;

        for &point in region.iter() {
            let mut point_perimeter = 4;

            for next_point in ORTHOGONAL.map(|dir| point + dir) {
                if grid.contains(next_point) && grid[next_point] == grid[point] {
                    point_perimeter -= 1;
                }
            }

            perimeter += point_perimeter;
        }

        total += region.len() as u32 * perimeter;
    }

    Some(total)
}

fn get_regions_boundaries(region: &[Point]) -> (i32, i32, i32, i32) {
    let min_x = region.iter().map(|&point| point.x).min().unwrap();
    let max_x = region.iter().map(|&point| point.x).max().unwrap();
    let min_y = region.iter().map(|&point| point.y).min().unwrap();
    let max_y = region.iter().map(|&point| point.y).max().unwrap();

    (min_x, max_x, min_y, max_y)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let regions = get_regions(&grid);
    let mut total = 0;

    for region in regions {
        let (min_x, max_x, min_y, max_y) = get_regions_boundaries(&region);
        let mut angles = 0;

        for x in min_x..=max_x + 1 {
            for y in min_y..=max_y + 1 {
                let values: Vec<Point> = [
                    Point::new(x, y),
                    Point::new(x, y) + UP,
                    Point::new(x, y) + LEFT,
                    Point::new(x, y) + UP_LEFT,
                ]
                .iter()
                .filter(|&point| region.contains(point))
                .copied()
                .collect();

                if values.len() == 3 || values.len() == 1 {
                    angles += 1;
                } else if values.len() == 2
                    && values[0].x != values[1].x
                    && values[0].y != values[1].y
                {
                    angles += 2;
                }
            }
        }

        total += angles * region.len() as u32;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_part_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_part_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_part_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_part_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1206));
    }
}
