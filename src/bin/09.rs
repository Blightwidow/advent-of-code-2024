//! # Day 9: Disk Fragmenter
//!
//! For part one, we can simply unpile the disk as once we use a digit,
//! its position is not counter in the final checksum. This will speed up the checksum
//! calculation.
//!
//! In part two we can precompute the gaps and files and then iterate over the files
//! in reverse order, trying to find a gap that fits the file. We just need to update the
//! gaps when we use them.

advent_of_code::solution!(9);

fn checksum(disk: Vec<i32>) -> u64 {
    let mut total = 0;

    for (i, &bit) in disk.iter().enumerate() {
        if bit != -1 {
            total += i as u64 * bit as u64;
        }
    }

    total
}

fn parse_disk(input: &str) -> (Vec<i32>, Vec<(usize, u32)>, Vec<(usize, u32)>) {
    let mut disk: Vec<i32> = Vec::new();
    let mut files: Vec<(usize, u32)> = Vec::new();
    let mut gaps: Vec<(usize, u32)> = Vec::new();

    for (i, char) in input.chars().enumerate() {
        if let Some(value) = char.to_digit(10) {
            if i % 2 == 0 {
                files.push((disk.len(), value));
            } else {
                gaps.push((disk.len(), value));
            }

            for _ in 0..value {
                if i % 2 == 0 {
                    disk.push((i / 2) as i32);
                } else {
                    disk.push(-1);
                }
            }
        } else {
            break;
        }
    }

    (disk, files, gaps)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut disk, _, _) = parse_disk(input);
    let mut i = 0;

    while i < disk.len() {
        if disk[i] == -1 {
            match disk.pop().unwrap() {
                -1 => continue,
                value => disk[i] = value,
            }
        }

        i += 1;
    }

    Some(checksum(disk))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut disk, unfragged_files, mut gaps) = parse_disk(input);

    for (id, &(file_start, filesize)) in unfragged_files.iter().enumerate().rev() {
        let matching_gap = gaps
            .iter_mut()
            .take_while(|&&mut (start, _)| start < file_start)
            .find(|&&mut (_, size)| size >= filesize);

        if let Some(gap) = matching_gap {
            let (gap_start, gap_size) = *gap;
            for k in 0..filesize {
                disk[gap_start + k as usize] = id as i32;
                disk[file_start + k as usize] = -1;
            }

            *gap = (gap_start + filesize as usize, gap_size - filesize);
        }
    }

    Some(checksum(disk))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
