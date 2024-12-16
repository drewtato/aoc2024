use std::ops::{Add, AddAssign, Index, IndexMut, Mul};
use std::sync::atomic::AtomicUsize;

use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).part_one()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).part_two()
    }
}

type Int = i16;

#[derive(Debug, Clone)]
struct Solver {
    map: Vec<Spot>,
    width: Int,
    height: Int,
    start: [Int; 2],
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut start = None;
        let mut map = Vec::with_capacity(input.len());
        let mut y = 0;
        let mut iter = input.iter();
        let mut last_rocks_north = Vec::with_capacity(input.len().isqrt());

        // First row
        let mut last_rock_west = -1;
        for (x, &b) in iter.by_ref().enumerate() {
            let (spot, rock) = match b {
                b'\n' => {
                    break;
                }
                b'.' => (Empty([-1, 0, 0, last_rock_west]), -1),
                b'^' => {
                    start = Some([y, x as Int]);
                    (Empty([-1, 0, 0, last_rock_west]), -1)
                }
                b'#' => {
                    last_rock_west = x as Int + 1;
                    (Rock, y + 1)
                }
                other => panic!("input contained {other:?}"),
            };
            map.push(spot);
            last_rocks_north.push(rock);
        }
        y += 1;

        let width = input.len() - iter.as_slice().len() - 1;

        while !iter.as_slice().is_empty() {
            let mut last_rock_west = -1;
            for (x, &b) in iter.by_ref().enumerate() {
                let spot = match b {
                    b'\n' => {
                        break;
                    }
                    b'.' => Empty([last_rocks_north[x], 0, 0, last_rock_west]),
                    b'^' => {
                        start = Some([y, x as Int]);
                        Empty([last_rocks_north[x], 0, 0, last_rock_west])
                    }
                    b'#' => {
                        last_rock_west = x as Int + 1;
                        last_rocks_north[x] = y + 1;
                        Rock
                    }
                    other => panic!("input contained {other:?}"),
                };
                map.push(spot);
            }
            y += 1;
        }

        let height = y;
        let width = width as Int;

        for row in map.chunks_mut(width as usize) {
            let mut last_rock = -1;
            for (x, spot) in row.iter_mut().enumerate().rev() {
                match spot {
                    Empty(nearest) => nearest[East] = last_rock,
                    Rock => last_rock = x as Int - 1,
                }
            }
        }

        let mut last_rocks_south: Vec<_> = last_rocks_north.into_iter().map(|_| -1).collect();
        for (y, row) in map.chunks_mut(width as usize).enumerate().rev() {
            for (x, spot) in row.iter_mut().enumerate() {
                match spot {
                    Empty(nearest) => nearest[South] = last_rocks_south[x],
                    Rock => last_rocks_south[x] = y as Int - 1,
                }
            }
        }

        Self {
            map,
            start: start.expect("no starting position found"),
            width,
            height,
        }
    }

    fn get(&self, pos: [Int; 2]) -> Option<Spot> {
        map_get(&self.map, self.width, self.height, pos).copied()
    }

    fn rows(&self) -> impl Iterator<Item = &[Spot]> {
        self.map.chunks(self.width as usize)
    }

    fn part_one(&self) -> usize {
        let mut pos = self.start;
        let mut dir = North;
        let mut visited = vec![false; self.map.len()];
        let mut visited_count = 0;
        loop {
            let next_coord = self.get(pos).unwrap().unwrap_empty()[dir];

            if next_coord == -1 {
                while let Some(v) = map_get_mut(&mut visited, self.width, self.height, pos) {
                    if !replace(v, true) {
                        visited_count += 1;
                    }
                    pos += dir;
                }
                break;
            }

            let dist = dir.distance_to_coord(pos, next_coord);
            for _ in 0..dist {
                let v = map_get_mut(&mut visited, self.width, self.height, pos).unwrap();
                if !replace(v, true) {
                    visited_count += 1;
                }
                pos += dir;
            }

            dir.turn_right();
        }
        visited_count
    }

    fn part_two(&self) -> usize {
        let mut pos = self.start;
        let mut dir = North;
        let mut visited = vec![false; self.map.len()];
        *map_get_mut(&mut visited, self.width, self.height, pos).unwrap() = true;
        let count = AtomicUsize::new(0);
        rayon::scope(|scope| {
            let count = &count;
            loop {
                let next_coord = self.get(pos).unwrap().unwrap_empty()[dir];

                if next_coord == -1 {
                    let next_coord = match dir {
                        North => 0,
                        East => self.width - 1,
                        South => self.height - 1,
                        West => 0,
                    };
                    let dist = dir.distance_to_coord(pos, next_coord);
                    for _ in 0..dist {
                        let new_pos = pos + dir;
                        let v =
                            map_get_mut(&mut visited, self.width, self.height, new_pos).unwrap();
                        if !replace(v, true) {
                            scope.spawn(move |_| {
                                if self.detect_cycle(pos, dir) {
                                    count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                }
                            });
                        }
                        pos = new_pos;
                    }

                    break;
                }

                let dist = dir.distance_to_coord(pos, next_coord);
                for _ in 0..dist {
                    let new_pos = pos + dir;
                    let v = map_get_mut(&mut visited, self.width, self.height, new_pos).unwrap();
                    if !replace(v, true) {
                        scope.spawn(move |_| {
                            if self.detect_cycle(pos, dir) {
                                count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                        });
                    }
                    pos = new_pos;
                }
                dir.turn_right();
            }
        });
        count.into_inner()
    }

    // Returns true when a cycle has been detected
    fn detect_cycle(&self, pos: [Int; 2], dir: Direction) -> bool {
        let mut pos1 = pos;
        let mut dir1 = dir;
        let mut pos2 = pos;
        let mut dir2 = dir;
        let new_rock = pos + dir;

        loop {
            // 1 (two steps)
            if self.step(&mut pos1, &mut dir1, new_rock) {
                break false;
            }
            if self.step(&mut pos1, &mut dir1, new_rock) {
                break false;
            }

            // Because dirs have a period of 4, and it rotates every step, these will only
            // be equal when 2 moves 2n and 1 moves 6n.
            if (pos1, dir1) == (pos2, dir2) {
                break true;
            }

            // 2 (one step)
            // Will never leave the grid before 1
            self.step(&mut pos2, &mut dir2, new_rock);
        }
    }

    // Returns true when pos has left the grid
    fn step(&self, pos: &mut [Int; 2], dir: &mut Direction, new_rock: [Int; 2]) -> bool {
        dir.turn_right();
        let next_coord = self.get(*pos).unwrap().unwrap_empty()[*dir];
        if next_coord == -1 {
            let next_coord = match dir {
                North => 0,
                East => self.width - 1,
                South => self.height - 1,
                West => 0,
            };
            let offset = *dir * dir.distance_to_coord(*pos, next_coord);
            if let Some(new_pos) = offset.overlaps(*pos, new_rock) {
                *pos = new_pos;
                return false;
            } else {
                return true;
            }
        }
        let offset = *dir * dir.distance_to_coord(*pos, next_coord);
        if let Some(new_pos) = offset.overlaps(*pos, new_rock) {
            *pos = new_pos;
        } else {
            *pos += offset;
        }
        false
    }

    #[allow(dead_code)]
    fn print_point(&self, point: [Int; 2], c: char) {
        for (y, row) in self.rows().enumerate() {
            for (x, s) in row.iter().enumerate() {
                if [y as Int, x as Int] == point {
                    eprint!("{c}");
                } else {
                    eprint!("{}", s.to_char());
                }
            }
            eprintln!();
        }
        eprintln!();
    }
}

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        let mut line = [const { String::new() }; 3];
        for row in self.rows() {
            let [top, mid, bot] = &mut line;
            for spot in row {
                // Each spot takes 7 spaces
                match *spot {
                    Empty([n, e, s, w]) => {
                        write!(top, "  {n:3}  ").unwrap();
                        write!(mid, "{w:3} {e:3}").unwrap();
                        write!(bot, "  {s:3}  ").unwrap();
                    }
                    Rock => {
                        top.push_str("       ");
                        mid.push_str("    #  ");
                        bot.push_str("       ");
                    }
                }
            }
            for subline in &mut line {
                subline.pop();
                subline.push('\n');
                f.write_str(subline)?;
                subline.clear();
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spot {
    /// The array stores the absolute coordinate next to the nearest rock in the
    /// map, or -1 if there is no rock in that direction.
    Empty([Int; 4]),
    Rock,
}
use Spot::*;

impl Spot {
    #[track_caller]
    fn unwrap_empty(self) -> [Int; 4] {
        let Empty(c) = self else {
            panic!("spot was not empty")
        };
        c
    }

    fn to_char(self) -> char {
        match self {
            Empty(_) => '.',
            Rock => '#',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use self::Direction::*;

impl Direction {
    fn to_coord(self) -> [Int; 2] {
        match self {
            North => [-1, 0],
            East => [0, 1],
            South => [1, 0],
            West => [0, -1],
        }
    }

    fn to_index(self) -> usize {
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            North => East,
            East => South,
            South => West,
            West => North,
        };
    }

    #[allow(dead_code)]
    fn turn_left(&mut self) {
        *self = match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    #[allow(dead_code)]
    fn is_vertical(self) -> bool {
        matches!(self, North | South)
    }

    fn distance_to_coord(self, pos: [Int; 2], coord: Int) -> Int {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    dir: Direction,
    mag: Int,
}

impl Vector {
    fn overlaps(self, pos: [Int; 2], new_rock: [Int; 2]) -> Option<[Int; 2]> {
        match self.dir {
            North => {
                if pos[1] == new_rock[1] && (0..=self.mag).contains(&(pos[0] - new_rock[0])) {
                    return Some([new_rock[0] + 1, pos[1]]);
                }
            }
            East => {
                if pos[0] == new_rock[0] && (0..=self.mag).contains(&(new_rock[1] - pos[1])) {
                    return Some([pos[0], new_rock[1] - 1]);
                }
            }
            South => {
                if pos[1] == new_rock[1] && (0..=self.mag).contains(&(new_rock[0] - pos[0])) {
                    return Some([new_rock[0] - 1, pos[1]]);
                }
            }
            West => {
                if pos[0] == new_rock[0] && (0..=self.mag).contains(&(pos[1] - new_rock[1])) {
                    return Some([pos[0], new_rock[1] + 1]);
                }
            }
        }
        None
    }
}

impl Mul<Int> for Direction {
    type Output = Vector;

    fn mul(self, rhs: Int) -> Self::Output {
        Vector {
            dir: self,
            mag: rhs,
        }
    }
}

impl Add<Vector> for [Int; 2] {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        let [dy, dx] = rhs.dir.to_coord();
        let [y, x] = self;
        [y + dy * rhs.mag, x + dx * rhs.mag]
    }
}

impl AddAssign<Vector> for [Int; 2] {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs
    }
}

fn map_get<T>(slice: &[T], width: Int, height: Int, pos: [Int; 2]) -> Option<&T> {
    if !(0..height).contains(&pos[0]) || !(0..width).contains(&pos[1]) {
        return None;
    }
    let index = pos[0] * width + pos[1];
    slice.get(index as usize)
}

fn map_get_mut<T>(slice: &mut [T], width: Int, height: Int, pos: [Int; 2]) -> Option<&mut T> {
    if !(0..height).contains(&pos[0]) || !(0..width).contains(&pos[1]) {
        return None;
    }
    let index = pos[0] * width + pos[1];
    slice.get_mut(index as usize)
}
