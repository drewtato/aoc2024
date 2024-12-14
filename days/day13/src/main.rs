use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver {
    rules: Vec<Rule>,
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).one()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).two()
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    ay: i64,
    ax: i64,
    by: i64,
    bx: i64,
    py: i64,
    px: i64,
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut rules = Vec::new();
        let mut con = Consume::new(input);
        while !con.is_empty() {
            con.non_digits();
            let ax = con.signed_int().unwrap();
            con.non_digits();
            let ay = con.signed_int().unwrap();
            con.newline();

            con.non_digits();
            let bx = con.signed_int().unwrap();
            con.non_digits();
            let by = con.signed_int().unwrap();
            con.newline();

            // this is not held
            // assert_ne!(ax > ay, bx > by, "{ax} > {ay} and {bx} > {by}");

            con.non_digits();
            let px = con.int().unwrap();
            con.non_digits();
            let py = con.int().unwrap();
            con.newline();

            rules.push(Rule {
                ax,
                ay,
                bx,
                by,
                px,
                py,
            });

            con.newline();
        }
        Self { rules }
    }

    fn one(&self) -> i64 {
        self.rules
            .iter()
            .copied()
            .filter_map(Rule::least_tokens)
            .sum()
    }

    fn two(&self) -> i64 {
        self.rules
            .iter()
            .copied()
            .filter_map(|mut rule| {
                rule.py += ERROR;
                rule.px += ERROR;
                rule.least_tokens()
            })
            .sum()
    }
}

const A_TOKENS: i64 = 3;
const B_TOKENS: i64 = 1;
const ERROR: i64 = 10_000_000_000_000;

impl Rule {
    #[allow(dead_code)]
    fn least_tokens_verified(self) -> Option<i64> {
        let mut tokens = 0;
        let mut y = 0;
        let mut x = 0;
        let mut bs = 0i64;

        while y < self.py && x < self.px {
            y += self.by;
            x += self.bx;
            tokens += B_TOKENS;
            bs += 1;
        }

        while y != self.py || x != self.px {
            if bs == 0 {
                return None;
            }
            y -= self.by;
            x -= self.bx;
            tokens -= B_TOKENS;
            bs -= 1;
            while y < self.py && x < self.px {
                y += self.ay;
                x += self.ax;
                tokens += A_TOKENS;
            }
        }

        Some(tokens)
    }

    fn least_tokens(self) -> Option<i64> {
        let Self {
            ay,
            ax,
            by,
            bx,
            py,
            px,
        } = self;
        let a_count = (bx * py - by * px) / (ay * bx - ax * by);
        let remaining_x = px - a_count * ax;
        let b_count = remaining_x / bx;
        if a_count >= 0 // ensure positive
            && b_count >= 0
            && a_count * ax + b_count * bx == px // ensure whole number
            && a_count * ay + b_count * by == py
        {
            Some(a_count * A_TOKENS + b_count * B_TOKENS)
        } else {
            None
        }
    }
}
