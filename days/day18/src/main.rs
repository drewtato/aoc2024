use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver {
    map: Vec<bool>,
    width: i32,
    height: i32,
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).one(input)
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).two(input)
    }
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let (width, height) = if input.len() <= 100 { (7, 7) } else { (71, 71) };
        let map = vec![true; (width * height) as usize];
        Self { map, width, height }
    }

    fn get_mut(&mut self, [y, x]: [i32; 2]) -> Option<&mut bool> {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            return None;
        }
        Some(&mut self.map[(y * self.width + x) as usize])
    }

    fn one(&mut self, input: &[u8]) -> usize {
        let mut con = Consume::new(input);
        let limit = if self.width == 7 { 12 } else { 1024 };
        for _ in 0..limit {
            let x = con.int().unwrap();
            con.consume_byte();
            let y = con.int().unwrap();
            con.newline();
            *self.get_mut([y, x]).expect("coordinates not in grid") = false;
        }

        self.find_exit().unwrap()
    }

    fn two(&mut self, input: &[u8]) -> Output {
        let limit = if self.width == 7 { 12 } else { 1024 };
        for steps in limit.. {
            self.map.fill(true);
            let mut con = Consume::new(input);
            let mut x = 0;
            let mut y = 0;
            for _ in 0..steps {
                x = con.int().unwrap();
                con.consume_byte();
                y = con.int().unwrap();
                con.newline();
                *self.get_mut([y, x]).expect("coordinates not in grid") = false;
            }
            if self.find_exit().is_none() {
                return Output { y, x };
            }
        }
        unreachable!()
    }

    fn find_exit(&mut self) -> Option<usize> {
        let mut frontier = vec![[0, 0]];
        let mut frontier_next = Vec::new();
        let exit = [self.height - 1, self.width - 1];
        for steps in 0.. {
            if frontier.is_empty() {
                return None;
            }
            for pos in frontier.drain(..) {
                for dir in Direction::all() {
                    let pos_next = pos + dir;
                    if pos_next == exit {
                        return Some(steps + 1);
                    }
                    let Some(tile) = self.get_mut(pos_next) else {
                        continue;
                    };
                    if replace(tile, false) {
                        frontier_next.push(pos_next);
                    }
                }
            }
            swap(&mut frontier, &mut frontier_next);
        }
        unreachable!()
    }
}

struct Output {
    y: i32,
    x: i32,
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
