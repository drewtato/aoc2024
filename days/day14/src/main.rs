use std::cmp::Ordering;
use std::io::Write;

use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver {
    bots: Vec<Bot>,
    width: i32,
    height: i32,
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut solver = Self::new(input);
        solver.simulate(100);
        solver.safety_factor()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut solver = Self::new(input);
        for seconds in 0..10_000 {
            if solver.is_tree() {
                return seconds;
            }
            solver.simulate(1);
        }
        9999
    }
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut con = Consume::new(input);
        let mut bots = Vec::new();
        while !con.is_empty() {
            con.non_digits();
            let a = con.signed_int().unwrap();
            con.non_digits();
            let b = con.signed_int().unwrap();
            con.non_digits();
            let c = con.signed_int().unwrap();
            con.non_digits();
            let d = con.signed_int().unwrap();
            con.next_newline();
            bots.push(Bot {
                py: b,
                px: a,
                vy: d,
                vx: c,
            });
        }

        let (width, height) = if bots.len() == 12 {
            eprintln!("test mode");
            (11, 7)
        } else {
            (WIDTH, HEIGHT)
        };

        Self {
            bots,
            width,
            height,
        }
    }

    fn safety_factor(&self) -> usize {
        let mut top_left = 0;
        let mut top_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;
        for bot in &self.bots {
            match (
                bot.py.cmp(&(self.height / 2)),
                bot.px.cmp(&(self.width / 2)),
            ) {
                (Ordering::Less, Ordering::Less) => top_left += 1,
                (Ordering::Less, Ordering::Greater) => top_right += 1,
                (Ordering::Greater, Ordering::Less) => bottom_left += 1,
                (Ordering::Greater, Ordering::Greater) => bottom_right += 1,
                _ => (),
            }
        }
        top_left * top_right * bottom_left * bottom_right
    }

    fn simulate(&mut self, seconds: usize) {
        for bot in &mut self.bots {
            bot.py = (bot.py + bot.vy * seconds as i32).rem_euclid(self.height);
            bot.px = (bot.px + bot.vx * seconds as i32).rem_euclid(self.width);
        }
    }

    #[allow(dead_code)]
    fn print(&self, file: &mut impl Write, second: usize) {
        writeln!(file, "{second}").unwrap();
        let mut grid = vec![b'0'; (self.width * self.height) as usize];
        for bot in &self.bots {
            grid[(bot.py * self.width + bot.px) as usize] += 1;
        }
        for row in grid.chunks(self.width as usize) {
            for &tile in row {
                if tile == b'0' {
                    write!(*file, ".").unwrap();
                } else {
                    write!(*file, "{}", tile as char).unwrap();
                }
            }
            writeln!(*file).unwrap();
        }
        writeln!(*file).unwrap();
    }

    fn is_tree(&self) -> bool {
        let mut rows = [0; HEIGHT as usize];
        let mut cols = [0; WIDTH as usize];
        for bot in &self.bots {
            rows[bot.py as usize] += 1;
            cols[bot.px as usize] += 1;
        }
        rows.iter().filter(|&&n| n >= 31).count() >= 2
            && cols.iter().filter(|&&n| n >= 33).count() >= 2
    }
}

#[derive(Debug, Clone, Copy)]
struct Bot {
    py: i32,
    px: i32,
    vy: i32,
    vx: i32,
}
