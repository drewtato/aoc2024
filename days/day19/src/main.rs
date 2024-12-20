use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

#[derive(Debug, Clone)]
struct Solver {
    patterns: PatternTree,
    designs: Vec<Box<[Stripe]>>,
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).one()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).two()
    }
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut con = Consume::new(input);

        let mut patterns = PatternTree::new();
        let mut current = Vec::new();
        while !con.newline() {
            let b = con.consume_byte().unwrap();
            if let Some(stripe) = Stripe::from_u8(b) {
                current.push(stripe);
                continue;
            }
            con.consume_byte();
            patterns.add_pattern(&current);
            current.clear();
        }

        con.consume_byte();
        patterns.add_pattern(&current);
        current.clear();

        let mut requests = Vec::new();
        while !con.is_empty() {
            let line = con.next_newline();
            current.extend(
                line[..line.len() - 1]
                    .iter()
                    .map(|&b| Stripe::from_u8(b).unwrap()),
            );
            requests.push(current.into_boxed_slice());
            current = Vec::new();
        }

        Self {
            patterns,
            designs: requests,
        }
    }

    fn one(&self) -> usize {
        self.designs
            .iter()
            .filter(|design| self.patterns.is_possible(design))
            .count()
    }

    fn two(&self) -> usize {
        let mut memo = PatternTree::create_memo();
        self.designs
            .iter()
            .map(|design| self.patterns.count_possible_memo(design, &mut memo))
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Stripe {
    White,
    Blue,
    Black,
    Red,
    Green,
}
use Stripe::*;

impl Stripe {
    fn from_u8(b: u8) -> Option<Self> {
        let color = match b {
            b'w' => White,
            b'u' => Blue,
            b'b' => Black,
            b'r' => Red,
            b'g' => Green,
            _ => return None,
        };
        Some(color)
    }

    fn to_index(self) -> usize {
        match self {
            White => 0,
            Blue => 1,
            Black => 2,
            Red => 3,
            Green => 4,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct PatternTree {
    terminates: bool,
    patterns: Option<Box<[PatternTree; 5]>>,
}

impl PatternTree {
    fn new() -> Self {
        Self::default()
    }

    fn add_pattern(&mut self, pattern: &[Stripe]) {
        let mut current = &mut *self;
        for &stripe in pattern {
            let patterns = current.patterns.get_or_insert_default();
            current = &mut patterns[stripe.to_index()];
        }
        current.terminates = true;
    }

    fn is_possible(&self, design: &[Stripe]) -> bool {
        if design.is_empty() {
            return true;
        }
        let mut i = 1;
        let pairs = fn_iter(|| design.split_at_checked(i).inspect(|_| i += 1));
        for (head, tail) in pairs {
            match self.contains(head) {
                PatternContains::Yes => {
                    if self.is_possible(tail) {
                        return true;
                    }
                }
                PatternContains::NoTerminal => (),
                PatternContains::No => break,
            }
        }
        false
    }

    fn contains(&self, towel: &[Stripe]) -> PatternContains {
        let mut tree = self;
        for &stripe in towel {
            if let Some(patterns) = &tree.patterns {
                tree = &patterns[stripe.to_index()];
            } else {
                return PatternContains::No;
            }
        }
        if tree.terminates {
            PatternContains::Yes
        } else {
            PatternContains::NoTerminal
        }
    }

    fn create_memo<'a>() -> HashMap<&'a [Stripe], usize> {
        let mut memo = HashMap::default();
        memo.insert(&[] as &[Stripe], 1);
        memo
    }

    fn count_possible_memo<'a>(
        &self,
        design: &'a [Stripe],
        memo: &mut HashMap<&'a [Stripe], usize>,
    ) -> usize {
        if let Some(&count) = memo.get(design) {
            return count;
        }
        let mut i = 1;
        let pairs = fn_iter(|| design.split_at_checked(i).inspect(|_| i += 1));
        let mut count = 0;
        for (head, tail) in pairs {
            match self.contains(head) {
                PatternContains::Yes => {
                    count += self.count_possible_memo(tail, memo);
                }
                PatternContains::NoTerminal => (),
                PatternContains::No => break,
            }
        }
        memo.insert(design, count);
        count
    }
}

impl Drop for PatternTree {
    fn drop(&mut self) {
        if self.patterns.is_none() {
            return;
        }
        let mut branches = vec![self.patterns.take().unwrap()];
        while let Some(tree) = branches.pop() {
            branches.extend(tree.into_iter().flat_map(|mut pt| pt.patterns.take()));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatternContains {
    Yes,
    NoTerminal,
    No,
}
