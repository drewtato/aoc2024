#![feature(let_chains)]
// use std::ops::{Add, AddAssign};

use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver {
    map: Vec<Tile>,
    width: i32,
    height: i32,
    start: [i32; 2],
    end: [i32; 2],
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], debug: u8) -> impl Display + 'static {
        let target_save = if debug == 1 { 20 } else { 100 };
        Self::new(input).cheat_iter(2, |save| save >= target_save)
    }

    fn part_two(input: &[u8], debug: u8) -> impl Display + 'static {
        let target_save = if debug == 1 { 74 } else { 100 };
        Self::new(input).cheat_iter(20, |save| save >= target_save)
    }
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut map = Vec::with_capacity(input.len());
        let mut start = [0; 2];
        let mut end = [0; 2];
        let mut x = 0;
        let mut y = 0;
        map.extend(
            input
                .iter()
                .copied()
                .inspect(|&b| {
                    match b {
                        b'S' => start = [y, x],
                        b'E' => end = [y, x],
                        b'\n' => {
                            x = -1;
                            y += 1
                        }
                        _ => (),
                    }
                    x += 1;
                })
                .filter_map(Tile::from_u8),
        );
        let height = y;
        let width = map.len() as i32 / height;
        Self {
            map,
            width,
            height,
            start,
            end,
        }
    }

    // fn pathfind(&self) -> Option<u32> {
    //     let mut states = vec![State { pos: self.start }];
    //     let mut states_tmp = Vec::new();
    //     let mut visited = vec![false; self.map.len()];
    //     *get_mut(&mut visited, self.width, self.height, self.start).unwrap() =
    // true;

    //     for picoseconds in 1.. {
    //         if states.is_empty() {
    //             return None;
    //         }
    //         for state in states.drain(..) {
    //             for next in state.successors(self) {
    //                 if next.pos == self.end {
    //                     return Some(picoseconds);
    //                 }
    //                 if !replace(
    //                     get_mut(&mut visited, self.width, self.height,
    // next.pos).unwrap(),                     true,
    //                 ) {
    //                     states_tmp.push(next);
    //                 }
    //             }
    //         }
    //         swap(&mut states, &mut states_tmp);
    //     }
    //     unreachable!()
    // }

    fn get(&self, [y, x]: [i32; 2]) -> Option<Tile> {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            return None;
        }
        Some(self.map[(y * self.width + x) as usize])
    }

    // fn get_mut(&mut self, [y, x]: [i32; 2]) -> Option<&mut Tile> {
    //     if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
    //         return None;
    //     }
    //     Some(&mut self.map[(y * self.width + x) as usize])
    // }

    // fn one(&mut self, debug: u8) -> u32 {
    //     use Direction::*;

    //     let honest_path = self.pathfind().expect("no path found");
    //     let target_save = honest_path.saturating_sub(100);
    //     let mut cheats_that_save_at_least_target = 0;

    //     for y in 1..self.height - 1 {
    //         for x in 1..self.width - 1 {
    //             let pos = [y, x];
    //             let tile = replace(self.get_mut(pos).unwrap(), Track);
    //             if tile == Track {
    //                 continue;
    //             }
    //             let res = self
    //                 .pathfind()
    //                 .expect("no path found after removing walls???");
    //             if res <= target_save {
    //                 cheats_that_save_at_least_target += 1;
    //             }
    //             *self.get_mut(pos).unwrap() = tile;
    //         }
    //     }

    //     cheats_that_save_at_least_target
    // }

    fn cheat_iter(&self, cheat_length: i32, f: impl Fn(i32) -> bool + Sync) -> usize {
        let mut time_map = vec![0; self.map.len()];
        macro_rules! get_time_map {
            ($pos:expr) => {
                get_mut(&mut time_map, self.width, self.height, $pos)
            };
        }
        let mut pos = self.start;
        *get_time_map!(pos).unwrap() = 1;
        let mut time = 2;

        while pos != self.end {
            for dir in Direction::all() {
                let next_pos = pos + dir;
                if let Some(next_map) = get_time_map!(next_pos)
                    && *next_map == 0
                    && self.get(next_pos) == Some(Track)
                {
                    *next_map = time;
                    pos = next_pos;
                    break;
                }
            }
            time += 1;
        }

        // for row in time_map.chunks(self.width as usize) {
        //     for &tile in row {
        //         if tile == 0 {
        //             eprint!("  ");
        //         } else {
        //             eprint!("{tile:02}")
        //         }
        //     }
        //     eprintln!();
        // }

        macro_rules! get_time_map {
            ($pos:expr) => {
                get(&time_map, self.width, self.height, $pos)
            };
        }

        time_map
            .par_chunks(self.width as usize)
            .enumerate()
            .map(|(y, row)| {
                let mut count = 0;
                for (x, &start_time) in row.iter().enumerate() {
                    let start_pos = [y as i32, x as i32];
                    if start_time == 0 {
                        continue;
                    }
                    for distance in 2..=cheat_length {
                        for [dy, dx] in upper_diamond(distance) {
                            let end_pos = [start_pos[0] + dy, start_pos[1] + dx];
                            let Some(end_time) = get_time_map!(end_pos) else {
                                continue;
                            };
                            if end_time == 0 {
                                continue;
                            }
                            if f(end_time - start_time - distance) {
                                count += 1;
                            }
                            if f(start_time - end_time - distance) {
                                count += 1;
                            }
                        }
                    }
                }
                count
            })
            .sum()
    }
}

fn upper_diamond(distance: i32) -> impl Iterator<Item = [i32; 2]> {
    [].into_iter()
        .chain((1..distance).map(move |d| [-d, d - distance]))
        .chain((0..distance + 1).map(move |d| [-d, distance - d]))
}

fn get_mut<T>(map: &mut [T], width: i32, height: i32, [y, x]: [i32; 2]) -> Option<&mut T> {
    if !(0..width).contains(&x) || !(0..height).contains(&y) {
        return None;
    }
    Some(&mut map[(y * width + x) as usize])
}

fn get<T>(map: &[T], width: i32, height: i32, [y, x]: [i32; 2]) -> Option<T>
where
    T: Copy,
{
    if !(0..width).contains(&x) || !(0..height).contains(&y) {
        return None;
    }
    Some(map[(y * width + x) as usize])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Track,
    Wall,
}
use Tile::*;
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::slice::ParallelSlice;

impl Tile {
    fn from_u8(b: u8) -> Option<Self> {
        let tile = match b {
            b'.' | b'S' | b'E' => Track,
            b'#' => Wall,
            _ => return None,
        };
        Some(tile)
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct State {
//     pos: [i32; 2],
// }

// impl State {
//     fn successors(self, map: &Solver) -> impl Iterator<Item = Self> {
//         self.neighbors()
//             .map(|new_state| {
//                 if map.get(new_state.pos) == Some(Track) {
//                     Some(new_state)
//                 } else {
//                     None
//                 }
//             })
//             .into_iter()
//             .flatten()
//     }

//     fn neighbors(mut self) -> [Self; 4] {
//         let mut directions = Direction::all().into_iter();
//         from_fn_array(|_| self + directions.next().unwrap())
//     }
// }

// impl Add<Direction> for State {
//     type Output = State;

//     fn add(mut self, rhs: Direction) -> Self::Output {
//         self.pos += rhs;
//         self
//     }
// }

// impl AddAssign<Direction> for State {
//     fn add_assign(&mut self, rhs: Direction) {
//         *self = *self + rhs;
//     }
// }
