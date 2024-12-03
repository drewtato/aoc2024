use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut sum = 0;
        for m in re.captures_iter(input) {
            let n1: i32 = parse_ascii(&m[1]).unwrap();
            let n2: i32 = parse_ascii(&m[2]).unwrap();
            sum += n1 * n2;
        }
        sum
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
        let mut sum = 0;
        let mut enabled = true;
        for m in re.captures_iter(input) {
            if m.get(1).is_some() && enabled {
                let n1: i32 = parse_ascii(&m[2]).unwrap();
                let n2: i32 = parse_ascii(&m[3]).unwrap();
                sum += n1 * n2;
            } else if m.get(4).is_some() {
                enabled = true;
            } else if m.get(5).is_some() {
                enabled = false;
            }
        }
        sum
    }
}
