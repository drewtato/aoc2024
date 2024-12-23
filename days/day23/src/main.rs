use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

#[derive(Debug, Default)]
struct Solver {
    graph: HashMap<[u8; 2], HashSet<[u8; 2]>>,
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let solver = Self::new(input);
        let threes = solver.find_threes();
        threes
            .iter()
            .filter(|set| set.iter().map(|[a, _]| a).contains(&b't'))
            .count()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let solver = Self::new(input);
        let threes = solver.find_threes();

        let mut party: HashSet<[u8; 2]> = HashSet::default();
        let mut best_party: HashSet<[u8; 2]> = HashSet::default();

        for &[a, b, c] in &threes {
            party.extend([a, b, c]);

            for comp in &solver.graph[&a] {
                if solver.graph[comp].is_superset(&party) {
                    party.insert(*comp);
                }
            }
            if party.len() > best_party.len() {
                best_party.clone_from(&party);
            }

            party.clear();
        }

        Output(best_party.into_iter().sorted_unstable().collect())
    }
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut con = Consume::new(input);
        let mut solver = Self::default();
        while !con.is_empty() {
            let &[a1, a2, _, b1, b2, _] = con.consume(6) else {
                panic!("bad row")
            };
            solver.graph.entry([a1, a2]).or_default().insert([b1, b2]);
            solver.graph.entry([b1, b2]).or_default().insert([a1, a2]);
        }
        solver
    }

    fn find_threes(&self) -> HashSet<[[u8; 2]; 3]> {
        let mut threes = HashSet::default();
        for (first, seconds) in &self.graph {
            for second in seconds {
                for third in &self.graph[second] {
                    if self.graph[third].contains(first) {
                        let mut set = [*first, *second, *third];
                        set.sort_unstable();
                        threes.insert(set);
                    }
                }
            }
        }
        threes
    }
}

#[derive(Debug)]
struct Output(Vec<[u8; 2]>);

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter().map(BStr::new);
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
        }
        for item in iter {
            write!(f, ",{}", item)?;
        }
        Ok(())
    }
}
