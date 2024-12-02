use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut reports = Vec::new();
        let mut con = Consume::new(input);
        while !con.is_empty() {
            let mut row = Vec::new();
            while !con.newline() {
                con.whitespace();
                let n: i32 = con.int().unwrap();
                row.push(n);
            }
            reports.push(row);
        }
        let mut safe = 0;
        'l: for report in reports {
            let first = report[0];
            let mut last = report[1];
            if first < last {
                if !(first + 1..=first + 3).contains(&last) {
                    continue;
                }
                for &next in &report[2..] {
                    if !(last + 1..=last + 3).contains(&next) {
                        continue 'l;
                    }
                    last = next;
                }
            } else {
                if !(first - 3..=first - 1).contains(&last) {
                    continue;
                }
                for &next in &report[2..] {
                    if !(last - 3..=last - 1).contains(&next) {
                        continue 'l;
                    }
                    last = next;
                }
            };
            safe += 1;
        }
        safe
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut reports = Vec::new();
        let mut con = Consume::new(input);
        while !con.is_empty() {
            let mut row = Vec::new();
            while !con.newline() {
                con.whitespace();
                let n: i32 = con.int().unwrap();
                row.push(n);
            }
            reports.push(row);
        }
        let mut safe = 0;
        for report in reports {
            let mut this_safe = false;
            'l: for missing in 0..report.len() {
                let mut report = report.clone();
                report.remove(missing);

                let first = report[0];
                let mut last = report[1];
                if first < last {
                    if !(first + 1..=first + 3).contains(&last) {
                        continue;
                    }
                    for &next in &report[2..] {
                        if !(last + 1..=last + 3).contains(&next) {
                            continue 'l;
                        }
                        last = next;
                    }
                } else {
                    if !(first - 3..=first - 1).contains(&last) {
                        continue;
                    }
                    for &next in &report[2..] {
                        if !(last - 3..=last - 1).contains(&next) {
                            continue 'l;
                        }
                        last = next;
                    }
                };
                this_safe = true;
                break;
            }
            if this_safe {
                safe += 1;
            }
        }
        safe
    }
}
