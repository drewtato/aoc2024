use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut l1 = Vec::new();
        let mut l2 = Vec::new();

        let mut con = Consume::new(input);
        while !con.is_empty() {
            let n1: u32 = con.int().unwrap();
            l1.push(n1);
            con.whitespace();
            let n2 = con.int().unwrap();
            l2.push(n2);
            con.newline();
        }

        l1.sort();
        l2.sort();

        l1.iter().zip(&l2).map(|(&a, &b)| a.abs_diff(b)).bsum()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut l1 = Vec::new();
        let mut l2 = Counter::new();

        let mut con = Consume::new(input);
        while !con.is_empty() {
            let n1: u32 = con.int().unwrap();
            l1.push(n1);
            con.whitespace();
            let n2 = con.int().unwrap();
            l2.add(n2);
            con.newline();
        }

        l1.into_iter().map(|n| l2.get(n) * n as usize).bsum()
    }
}
