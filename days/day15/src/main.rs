use std::fs::File;
use std::io::{BufWriter, Write};

use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

#[derive(Debug)]
struct Solver {
    map: Vec<u8>,
    width: i32,
    height: i32,
    ry: i32,
    rx: i32,
    movements: Vec<Direction>,
}

#[derive(Debug)]
struct Solver2 {
    map: Vec<u8>,
    width: i32,
    height: i32,
    ry: i32,
    rx: i32,
    movements: Vec<Direction>,
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], debug: u8) -> impl Display + 'static {
        let mut solver = Self::new(input);
        if debug == 1 {
            for _ in 0..1000 {
                solver.simulate_all();
                solver.print(&mut BufWriter::new(
                    File::create("outputs/day15/out01.txt").unwrap(),
                ));
            }
        } else {
            solver.simulate_all();
        }
        solver.sum_of_gps()
    }

    fn part_two(input: &[u8], debug: u8) -> impl Display + 'static {
        let mut solver = Solver2::new(input);
        if debug == 1 {
            for _ in 0..1000 {
                solver.simulate_all();
                solver.print(&mut BufWriter::new(
                    File::create("outputs/day15/out02.txt").unwrap(),
                ));
            }
        } else {
            solver.simulate_all();
        }
        solver.sum_of_gps()
    }
}

impl Solver2 {
    fn new(input: &[u8]) -> Self {
        let mut con = Consume::new(input);
        let mut map = Vec::new();
        let mut height = 0;
        let mut ry = 0;
        let mut rx = 0;
        loop {
            let line = con.next_newline();
            let line = &line[..line.len() - 1];
            if line.is_empty() {
                break;
            }
            map.extend(line.iter().copied().enumerate().flat_map(|(x, b)| match b {
                b'@' => {
                    rx = x as i32 * 2;
                    ry = height;
                    [b'.', b'.']
                }
                BOX => [LEFT_BOX, RIGHT_BOX],
                b => [b, b],
            }));
            height += 1;
        }
        let width = map.len() as i32 / height;
        let movements = con.slice().iter().copied().filter_map(from_u8).collect();
        Self {
            map,
            width,
            height,
            movements,
            ry,
            rx,
        }
    }

    fn get(&self, [y, x]: [i32; 2]) -> Option<u8> {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            return None;
        }
        Some(self.map[(y * self.width + x) as usize])
    }

    fn simulate(&mut self, steps: usize) {
        // let mut file = &mut
        // BufWriter::new(File::create("outputs/day15/out.txt").unwrap());
        let mut will_be_box = Vec::new();
        let mut frontier = Vec::new();
        let mut frontier_tmp = Vec::new();
        'l: for step in 0..steps {
            will_be_box.clear();
            frontier.clear();
            frontier_tmp.clear();
            // self.print(file);
            let dir = self.movements[step % self.movements.len()];
            // writeln!(file, "{dir:?} ({}, {})", self.ry, self.rx);
            let old_robot = [self.ry, self.rx];
            let will_be_robot = old_robot + dir;

            frontier.push(will_be_robot);
            while !frontier.is_empty() {
                frontier.dedup();
                for coord in frontier.drain(..) {
                    if dir.is_vertical() {
                        match self.get(coord).unwrap() {
                            LEFT_BOX => {
                                frontier_tmp.push(coord + dir);
                                will_be_box.push((coord + dir, LEFT_BOX));

                                frontier_tmp.push(coord + dir + East);
                                will_be_box.push((coord + dir + East, RIGHT_BOX));

                                will_be_box.push((coord + East, EMPTY));
                            }
                            RIGHT_BOX => {
                                frontier_tmp.push(coord + dir + West);
                                will_be_box.push((coord + dir + West, LEFT_BOX));

                                frontier_tmp.push(coord + dir);
                                will_be_box.push((coord + dir, RIGHT_BOX));

                                will_be_box.push((coord + West, EMPTY));
                            }
                            WALL => continue 'l,
                            EMPTY => (),
                            _ => unreachable!(),
                        }
                    } else {
                        match self.get(coord).unwrap() {
                            tile @ (LEFT_BOX | RIGHT_BOX) => {
                                frontier_tmp.push(coord + dir);
                                will_be_box.push((coord + dir, tile))
                            }
                            WALL => continue 'l,
                            EMPTY => (),
                            _ => unreachable!(),
                        }
                    }
                }
                swap(&mut frontier, &mut frontier_tmp);
            }

            for &(coord, tile) in will_be_box.iter().rev() {
                *self.get_mut(coord).unwrap() = tile;
            }

            [self.ry, self.rx] = will_be_robot;
            *self.get_mut(will_be_robot).unwrap() = EMPTY;
        }
        // self.print(file);
    }

    fn simulate_all(&mut self) {
        self.simulate(self.movements.len());
    }

    fn sum_of_gps(&self) -> i32 {
        let mut sum = 0;
        for (y, row) in self.map.chunks(self.width as usize).enumerate() {
            for (x, tile) in row.iter().copied().enumerate() {
                if tile == LEFT_BOX {
                    sum += gps(y as i32, x as i32);
                }
            }
        }
        sum
    }

    fn get_mut(&mut self, [y, x]: [i32; 2]) -> Option<&mut u8> {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            return None;
        }
        Some(&mut self.map[(y * self.width + x) as usize])
    }

    #[allow(dead_code)]
    fn print(&self, f: &mut impl Write) {
        for (y, row) in self.map.chunks(self.width as usize).enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if y as i32 == self.ry && x as i32 == self.rx {
                    write!(f, "@").unwrap();
                } else {
                    write!(f, "{}", tile as char).unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        writeln!(f).unwrap();
    }
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut con = Consume::new(input);
        let mut map = Vec::new();
        let mut height = 0;
        let mut ry = 0;
        let mut rx = 0;
        loop {
            let line = con.next_newline();
            let line = &line[..line.len() - 1];
            if line.is_empty() {
                break;
            }
            map.extend(line.iter().copied().enumerate().map(|(x, b)| {
                if b == b'@' {
                    rx = x as i32;
                    ry = height;
                    b'.'
                } else {
                    b
                }
            }));
            height += 1;
        }
        let width = map.len() as i32 / height;
        let movements = con.slice().iter().copied().filter_map(from_u8).collect();
        Self {
            map,
            width,
            height,
            movements,
            ry,
            rx,
        }
    }

    fn get(&self, [y, x]: [i32; 2]) -> Option<u8> {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            return None;
        }
        Some(self.map[(y * self.width + x) as usize])
    }

    fn simulate(&mut self, steps: usize) {
        // let mut file = &mut
        // BufWriter::new(File::create("outputs/day15/out.txt").unwrap());
        let mut stack = Vec::new();
        for step in 0..steps {
            stack.clear();
            // self.print(file);
            let dir = self.movements[step % self.movements.len()];
            // writeln!(file, "{dir:?} ({}, {})", self.ry, self.rx);
            let old_robot = [self.ry, self.rx];
            let will_be_robot = old_robot + dir;
            let mut new_coords = will_be_robot;
            while self.get(new_coords).unwrap() == BOX {
                new_coords += dir;
                stack.push(new_coords);
            }
            if self.get(new_coords).unwrap() == WALL {
                continue;
            }
            while let Some(coords) = stack.pop() {
                *self.get_mut(coords).unwrap() = BOX;
            }
            [self.ry, self.rx] = will_be_robot;
            *self.get_mut(will_be_robot).unwrap() = EMPTY;
        }
        // self.print(file);
    }

    fn simulate_all(&mut self) {
        self.simulate(self.movements.len());
    }

    fn sum_of_gps(&self) -> i32 {
        let mut sum = 0;
        for (y, row) in self.map.chunks(self.width as usize).enumerate() {
            for (x, tile) in row.iter().copied().enumerate() {
                if tile == BOX {
                    sum += gps(y as i32, x as i32);
                }
            }
        }
        sum
    }

    fn get_mut(&mut self, [y, x]: [i32; 2]) -> Option<&mut u8> {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            return None;
        }
        Some(&mut self.map[(y * self.width + x) as usize])
    }

    #[allow(dead_code)]
    fn print(&self, f: &mut impl Write) {
        for (y, row) in self.map.chunks(self.width as usize).enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if y as i32 == self.ry && x as i32 == self.rx {
                    write!(f, "@").unwrap();
                } else {
                    write!(f, "{}", tile as char).unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        writeln!(f).unwrap();
    }
}

const BOX: u8 = b'O';
const LEFT_BOX: u8 = b'[';
const RIGHT_BOX: u8 = b']';
const WALL: u8 = b'#';
const EMPTY: u8 = b'.';

fn gps(y: i32, x: i32) -> i32 {
    y * 100 + x
}

use Direction::*;

fn from_u8(b: u8) -> Option<Direction> {
    let s = match b {
        b'^' => North,
        b'>' => East,
        b'v' => South,
        b'<' => West,
        _ => return None,
    };
    Some(s)
}
