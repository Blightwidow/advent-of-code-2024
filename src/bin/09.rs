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
    disk.iter().enumerate().fold(0, |acc, (i, &bit)| {
        if bit != -1 {
            acc + i as u64 * bit as u64
        } else {
            acc
        }
    })
}

type Input = (Vec<i32>, Vec<(usize, u32)>, Vec<(usize, u32)>);

fn parse(input: &str, truncated: bool) -> Input {
    let mut disk: Vec<i32> = Vec::new();
    let mut files: Vec<(usize, u32)> = Vec::new();
    let mut gaps: Vec<(usize, u32)> = Vec::new();

    for (i, char) in input.chars().enumerate() {
        if let Some(value) = char.to_digit(10) {
            if !truncated {
                if i % 2 == 0 {
                    files.push((disk.len(), value));
                } else {
                    gaps.push((disk.len(), value));
                }
            }

            disk.extend(
                std::iter::repeat(if i % 2 == 0 { i as i32 / 2 } else { -1 }).take(value as usize),
            );
        } else {
            break;
        }
    }

    (disk, files, gaps)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut disk, _, _) = parse(input, true);
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
    let (mut disk, unfragged_files, mut gaps) = parse(input, false);

    for (id, &(file_start, filesize)) in unfragged_files.iter().enumerate().rev() {
        let matching_gap = gaps
            .iter_mut()
            .enumerate()
            .take_while(|&(_, &mut (start, _))| start < file_start)
            .find(|&(_, &mut (_, size))| size >= filesize);

        if let Some((i, gap)) = matching_gap {
            let (gap_start, gap_size) = *gap;
            for k in 0..filesize {
                disk[gap_start + k as usize] = id as i32;
                disk[file_start + k as usize] = -1;
            }

            if gap_size - filesize == 0 {
                gaps.remove(i);
            } else {
                *gap = (gap_start + filesize as usize, gap_size - filesize);
            }
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
