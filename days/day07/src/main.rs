use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut con = Consume::new(input);
        let mut sum = 0;
        let mut possibles: Vec<u64> = Vec::new();
        let mut possibles_temp = Vec::new();
        while !con.is_empty() {
            let total = con.int().unwrap();
            con.byte(b':');
            possibles.push(0);

            while !con.newline() {
                con.byte(b' ');
                let n: u64 = con.int().unwrap();
                for &p in &possibles {
                    for new in [p * n, p + n] {
                        if new <= total {
                            possibles_temp.push(new);
                        }
                    }
                }
                swap(&mut possibles_temp, &mut possibles);
                possibles_temp.clear();
            }

            if possibles.contains(&total) {
                sum += total;
            }

            possibles.clear();
        }
        sum
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut con = Consume::new(input);
        let mut sum = 0;
        let mut possibles: Vec<u64> = Vec::new();
        let mut possibles_temp = Vec::new();
        while !con.is_empty() {
            let total = con.int().unwrap();
            con.byte(b':');
            possibles.push(0);

            while !con.newline() {
                con.byte(b' ');
                let n: u64 = con.int().unwrap();
                for &p in &possibles {
                    // eprintln!("{p} || {n} = {:?}", concatenate(p, n));
                    for new in [p.checked_mul(n), p.checked_add(n), concatenate(p, n)]
                        .into_iter()
                        .flatten()
                    {
                        if new <= total {
                            possibles_temp.push(new);
                        }
                    }
                }
                swap(&mut possibles_temp, &mut possibles);
                possibles_temp.clear();
            }

            // eprintln!("{total}: {possibles:?}");
            if possibles.contains(&total) {
                sum += total;
            }

            possibles.clear();
        }
        sum
    }
}

fn concatenate(p: u64, n: u64) -> Option<u64> {
    if p == 0 {
        return None;
    }
    let len = n.checked_ilog10()?;
    p.checked_mul(10_u64.checked_pow(len + 1)?)?.checked_add(n)
}
