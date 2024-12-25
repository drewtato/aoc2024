use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver {
    locks: Vec<[u8; 5]>,
    keys: Vec<[u8; 5]>,
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let solver = Self::new(input);
        let mut fit = 0;
        for lock in &solver.locks {
            // eprintln!("lock: {lock:?}");
            for key in &solver.keys {
                // eprintln!("key: {key:?}");
                if lock.iter().zip(key).all(|(a, b)| a + b <= 5) {
                    fit += 1;
                }
            }
        }
        fit
    }

    fn part_two(_input: &[u8], _debug: u8) -> impl Display + 'static {
        "sleigh!"
    }
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut con = Consume::new(input);
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        while !con.is_empty() {
            if con.next_newline() == b"#####\n" {
                let mut bits = [0; 5];
                for _ in 0..5 {
                    for (bit, &b) in bits.iter_mut().zip(con.next_newline()) {
                        if b == b'#' {
                            *bit += 1;
                        }
                    }
                }
                con.next_newline();
                locks.push(bits);
            } else {
                let mut bits = [0; 5];
                for _ in 0..5 {
                    for (bit, &b) in bits.iter_mut().zip(con.next_newline()) {
                        if b == b'#' {
                            *bit += 1;
                        }
                    }
                }
                con.next_newline();
                keys.push(bits);
            }
            con.next_newline();
        }
        Self { locks, keys }
    }
}
