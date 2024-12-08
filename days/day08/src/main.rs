use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut con = Consume::new(input);
        let mut antennas: HashMap<u8, HashSet<[i32; 2]>> = HashMap::default();
        let mut x = 0;
        let mut width = 0;
        while !con.is_empty() {
            let row = con.next_newline();
            for (y, &c) in row.iter().enumerate() {
                if c == b'.' {
                    continue;
                }
                antennas.entry(c).or_default().insert([y as i32, x]);
            }
            width = (row.len() - 1) as i32;
            x += 1;
        }

        let height = x;
        let mut antinodes = HashSet::default();
        for set in antennas.values() {
            let mut iter = set.iter();
            while let Some(&a1) = iter.next() {
                for &a2 in iter.clone() {
                    let dy = a1[0] - a2[0];
                    let dx = a1[1] - a2[1];
                    let an0 = [a2[0] - dy, a2[1] - dx];
                    let an1 = [a1[0] + dy, a1[1] + dx];
                    for an in [an0, an1] {
                        if (0..height).contains(&an[0]) && (0..width).contains(&an[1]) {
                            antinodes.insert(an);
                        }
                    }
                }
            }
        }

        antinodes.len()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut con = Consume::new(input);
        let mut antennas: HashMap<u8, HashSet<[i32; 2]>> = HashMap::default();
        let mut x = 0;
        let mut width = 0;
        while !con.is_empty() {
            let row = con.next_newline();
            for (y, &c) in row.iter().enumerate() {
                if c == b'.' {
                    continue;
                }
                antennas.entry(c).or_default().insert([y as i32, x]);
            }
            width = (row.len() - 1) as i32;
            x += 1;
        }

        let height = x;
        let mut antinodes = HashSet::default();
        for set in antennas.values() {
            let mut iter = set.iter();
            while let Some(&a1) = iter.next() {
                for &a2 in iter.clone() {
                    let dy = a1[0] - a2[0];
                    let dx = a1[1] - a2[1];
                    let nodes = (0..)
                        .map(|back| [a2[0] - dy * back, a2[1] - dx * back])
                        .take_while(|[any, anx]| {
                            (0..height).contains(any) && (0..width).contains(anx)
                        })
                        .chain(
                            (0..)
                                .map(|fwd| [a1[0] + dy * fwd, a1[1] + dx * fwd])
                                .take_while(|[any, anx]| {
                                    (0..height).contains(any) && (0..width).contains(anx)
                                }),
                        );
                    antinodes.extend(nodes);
                }
            }
        }

        antinodes.len()
    }
}
