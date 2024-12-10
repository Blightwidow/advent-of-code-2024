use crate::utils::point::Point;
use std::{fmt, ops::{Index, IndexMut}};

#[derive(Clone, Eq, PartialEq)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let mut bytes = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
        Grid {
            width,
            height,
            bytes,
        }
    }
}

impl<T> Grid<T> {
    /// Create a new grid with the same dimensions as the current grid, filled with the given `value`.
    #[inline]
    pub fn copy_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            bytes: vec![value; (self.width * self.height) as usize],
        }
    }

    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    /// Find the first point that satisfies the given `predicate`.
    pub fn find<F>(&self, predicate: F) -> Option<Point>
    where
        F: Fn(T) -> bool,
    {
        let to_point = |index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point::new(x, y)
        };
        self.bytes.iter().position(|&h| predicate(h)).map(to_point)
    }

    /// Find all points that satisfy the given `predicate`.
    pub fn find_all<F>(&self, predicate: F) -> Vec<Point>
    where
        F: Fn(T) -> bool,
    {
        let to_point = |index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point::new(x, y)
        };

        self.bytes
            .iter()
            .enumerate()
            .filter(|(_, &c)| predicate(c))
            .map(|(i, _)| to_point(i))
            .collect()
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.bytes[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.bytes[(self.width * index.y + index.x) as usize]
    }
}

impl fmt::Debug for Grid<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.bytes
                .chunks(self.width as usize)
                .map(|row| std::str::from_utf8(row).unwrap())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
