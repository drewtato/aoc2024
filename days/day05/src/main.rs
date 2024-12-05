use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let (rules, updates) = parse(input);

        let mut total = 0;

        'l: for up in &updates {
            for (index, n) in up.iter().enumerate() {
                let Some(rule) = rules.get(n) else {
                    continue;
                };
                if up[..index].iter().any(|prev| rule.contains(prev)) {
                    continue 'l;
                }
            }
            let mid = up.len() / 2;
            total += up[mid];
        }

        total
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let (rules, mut updates) = parse(input);

        let mut total = 0;

        for up in &mut updates {
            let mut index = 0;
            let mut was_incorrect = false;
            while let Some(n) = up.get(index) {
                let Some(rule) = rules.get(n) else {
                    index += 1;
                    continue;
                };
                for (i, prev) in up[..index].iter().enumerate() {
                    if rule.contains(prev) {
                        was_incorrect = true;
                        up[i..=index].rotate_right(1);
                        index = i;
                        break;
                    }
                }
                index += 1;
            }
            if was_incorrect {
                let mid = up.len() / 2;
                total += up[mid];
            }
        }

        total
    }
}

fn parse(input: &[u8]) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut con = Consume::new(input);

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::default();
    while !con.newline() {
        let n1 = con.int().unwrap();
        assert!(con.byte(b'|'));
        let n2 = con.int().unwrap();
        assert!(con.newline());
        rules.entry(n1).or_default().insert(n2);
    }

    let mut updates = Vec::new();
    while !con.is_empty() {
        let mut up = Vec::new();
        loop {
            let i = con.int().unwrap();
            up.push(i);
            if con.consume_byte() != Some(b',') {
                break;
            }
        }
        updates.push(up);
    }

    (rules, updates)
}
