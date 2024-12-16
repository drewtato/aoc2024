#![feature(let_chains)]

use std::fs::File;
use std::io::BufWriter;

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
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).best_path()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).visited_tiles()
    }
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut start = 0;
        let mut end = 0;
        let mut height = 0;
        let mut index = 0;
        let map: Vec<_> = input
            .iter()
            .copied()
            .filter_map(|b| {
                match b {
                    b'S' => start = index,
                    b'E' => end = index,
                    b'\n' => height += 1,
                    _ => (),
                }
                if b != b'\n' {
                    index += 1;
                }
                Tile::from_u8(b)
            })
            .collect();

        let width = map.len() as i32 / height;
        let start = [start / width, start % width];
        let end = [end / width, end % width];

        Self {
            map,
            width,
            height,
            start,
            end,
        }
    }

    fn best_path(&self) -> usize {
        let mut paths = BinaryHeap::new();
        paths.push(Path::new(0, self.start, East));
        let mut visited: Vec<_> = self.map.iter().map(|_| [usize::MAX; 4]).collect();

        while let Some(Path {
            weight,
            position,
            direction,
        }) = paths.pop()
        {
            if position == self.end {
                return weight;
            }
            for (nw, np, nd) in [
                (1000 + weight, position, direction.left()),
                (1 + weight, position + direction, direction),
                (1000 + weight, position, direction.right()),
            ] {
                let visit = get_mut(&mut visited, self.width, self.height, np).unwrap();
                let tile = self.get(np).unwrap();
                if tile != Empty || visit[nd.to_index()] < nw {
                    continue;
                }
                visit[nd.to_index()] = nw;
                paths.push(Path::new(nw, np, nd));
            }
        }
        panic!("no path found")
    }

    fn get(&self, [y, x]: [i32; 2]) -> Option<Tile> {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            return None;
        }
        Some(self.map[(y * self.width + x) as usize])
    }

    fn visited_tiles(&self) -> usize {
        let mut paths = BinaryHeap::new();
        paths.push(Path::new(0, self.start, East));
        let mut visited: Vec<_> = vec![[usize::MAX; 4]; self.map.len()];

        let mut max_weight = None;
        let mut endings = [false; 4];

        while let Some(Path {
            weight,
            position,
            direction,
        }) = paths.pop()
        {
            if let Some(max) = max_weight
                && weight > max
            {
                continue;
            }
            if position == self.end {
                max_weight = Some(weight);
                endings[direction.to_index()] = true;
                continue;
            }
            for (nw, np, nd) in [
                (1000 + weight, position, direction.left()),
                (1 + weight, position + direction, direction),
                (1000 + weight, position, direction.right()),
            ] {
                let visit = get_mut(&mut visited, self.width, self.height, np).unwrap();
                let tile = self.get(np).unwrap();
                if tile != Empty || visit[nd.to_index()] <= nw {
                    continue;
                }
                visit[nd.to_index()] = nw;
                paths.push(Path::new(nw, np, nd));
            }
        }

        let mut best_tiles = vec![[false; 4]; self.map.len()];
        *get_mut(&mut best_tiles, self.width, self.height, self.end).unwrap() = endings;
        get_mut(&mut best_tiles, self.width, self.height, self.start).unwrap()[East.to_index()] =
            true;
        let mut endings: Vec<_> = endings
            .into_iter()
            .enumerate()
            .filter(|&(_, b)| b)
            .map(|(i, _)| Path::new(max_weight.unwrap(), self.end, Direction::from_index(i)))
            .collect();

        while let Some(Path {
            weight,
            position,
            direction,
        }) = endings.pop()
        {
            for (dw, np, nd) in [
                (1000, position, direction.left()),
                (1, position - direction, direction),
                (1000, position, direction.right()),
            ] {
                let Some(nw) = weight.checked_sub(dw) else {
                    continue;
                };
                let visit = get(&visited, self.width, self.height, np).unwrap()[nd.to_index()];
                if visit != nw {
                    continue;
                }
                let tile = &mut get_mut(&mut best_tiles, self.width, self.height, np).unwrap()
                    [nd.to_index()];
                if replace(tile, true) {
                    continue;
                }
                endings.push(Path::new(nw, np, nd));
            }
        }

        let mut file = BufWriter::new(File::create("outputs/day16/out.txt").unwrap());
        use std::io::Write;
        for (mrow, brow) in self
            .map
            .chunks(self.width as usize)
            .zip(best_tiles.chunks(self.width as usize))
        {
            for (&tile, &bs) in mrow.iter().zip(brow) {
                match tile {
                    Empty => {
                        let bits = bs.into_iter().filter(|&b| b).count();
                        if bits != 0 {
                            write!(file, "{bits}").unwrap();
                        } else {
                            write!(file, " ").unwrap();
                        }
                    }
                    Wall => {
                        if bs != [false; 4] {
                            panic!("visited a wall")
                        }
                        write!(file, ".").unwrap();
                    }
                }
            }
            writeln!(file).unwrap();
        }

        best_tiles
            .iter()
            .filter(|&&tile| tile.into_iter().any(identity))
            .count()
    }
}

fn get<T>(map: &[T], width: i32, height: i32, [y, x]: [i32; 2]) -> Option<&T> {
    if !(0..width).contains(&x) || !(0..height).contains(&y) {
        return None;
    }
    Some(&map[(y * width + x) as usize])
}

fn get_mut<T>(map: &mut [T], width: i32, height: i32, [y, x]: [i32; 2]) -> Option<&mut T> {
    if !(0..width).contains(&x) || !(0..height).contains(&y) {
        return None;
    }
    Some(&mut map[(y * width + x) as usize])
}

use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
}
use Tile::*;

impl Tile {
    fn from_u8(b: u8) -> Option<Self> {
        let s = match b {
            b'.' => Empty,
            b'#' => Wall,
            b'S' => Empty,
            b'E' => Empty,
            b'\n' => return None,
            _ => panic!("bad byte"),
        };
        Some(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Path {
    weight: usize,
    position: [i32; 2],
    direction: Direction,
}

impl Path {
    fn new(weight: usize, position: [i32; 2], direction: Direction) -> Self {
        Self {
            weight,
            position,
            direction,
        }
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}
