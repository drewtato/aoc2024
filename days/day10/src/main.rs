#![feature(let_chains)]
use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver {
    map: Vec<u8>,
    trailheads: Vec<[i32; 2]>,
    height: i32,
    width: i32,
}
impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut y = 0;
        let mut x = 0;
        let mut trailheads = Vec::new();
        let map: Vec<_> = input
            .iter()
            .copied()
            .filter(|&b| {
                match b {
                    b'\n' => {
                        y += 1;
                        x = 0;
                        return false;
                    }
                    b'0' => {
                        trailheads.push([y, x]);
                    }
                    _ => (),
                };
                x += 1;
                true
            })
            .map(|b| b.wrapping_sub(b'0'))
            .collect();

        let height = y;
        let width = map.len() as i32 / y;

        Self {
            map,
            trailheads,
            height,
            width,
        }
    }

    fn get(&self, [y, x]: [i32; 2]) -> Option<u8> {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            return None;
        }
        Some(self.map[(y * self.width + x) as usize])
    }

    fn find_trails(&self, mut f: impl FnMut(&HashMap<[i32; 2], usize>) -> usize) -> usize {
        let mut stack = HashMap::default();
        let mut stack_tmp = HashMap::default();

        self.trailheads
            .iter()
            .map(|&pos| {
                stack.insert(pos, 1);
                for height in 1..=9 {
                    for ([y, x], count) in stack.drain() {
                        for neighbor_pos in [[y - 1, x], [y, x + 1], [y + 1, x], [y, x - 1]] {
                            if let Some(neighbor_height) = self.get(neighbor_pos)
                                && neighbor_height == height
                            {
                                *stack_tmp.entry(neighbor_pos).or_default() += count;
                                continue;
                            }
                        }
                    }

                    swap(&mut stack, &mut stack_tmp);
                }

                let r = f(&stack);
                stack.clear();
                r
            })
            .sum()
    }

    fn one(&self) -> usize {
        self.find_trails(|stack| stack.len())
    }

    fn two(&self) -> usize {
        self.find_trails(|stack| stack.values().sum())
    }
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).one()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).two()
    }
}
