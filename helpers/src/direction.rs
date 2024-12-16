#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use std::ops::{Add, AddAssign, Index, IndexMut};

use Direction::*;

type Int = i32;

impl Direction {
    pub fn to_coord(self) -> [Int; 2] {
        match self {
            North => [-1, 0],
            East => [0, 1],
            South => [1, 0],
            West => [0, -1],
        }
    }

    pub fn to_index(self) -> usize {
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            North => East,
            East => South,
            South => West,
            West => North,
        };
    }

    pub fn turn_left(&mut self) {
        *self = match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn is_vertical(self) -> bool {
        matches!(self, North | South)
    }

    pub fn distance_to_coord(self, pos: [Int; 2], coord: Int) -> Int {
        match self {
            North => pos[0] - coord,
            East => coord - pos[1],
            South => coord - pos[0],
            West => pos[1] - coord,
        }
    }
}

impl<T> Index<Direction> for [T; 4] {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        &self[index.to_index()]
    }
}

impl<T> IndexMut<Direction> for [T; 4] {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self[index.to_index()]
    }
}

impl Add<Direction> for [Int; 2] {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let [y, x] = self;
        let [dy, dx] = rhs.to_coord();
        [y + dy, x + dx]
    }
}

impl AddAssign<Direction> for [Int; 2] {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs
    }
}
