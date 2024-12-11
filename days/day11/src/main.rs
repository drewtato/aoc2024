use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::calculate_iters(input, 25)
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::calculate_iters(input, 75)
    }
}

impl Solver {
    fn calculate_iters(input: &[u8], iters: usize) -> usize {
        let mut con = Consume::new(input);
        let mut memo = HashMap::default();
        let mut sum = 0;
        while !con.is_empty() {
            let stone = con.int::<u64>().unwrap();
            sum += calculate_memoed(stone, iters, &mut memo);
            con.whitespace();
        }
        sum
    }
}

fn calculate_memoed(stone: u64, iters: usize, memo: &mut HashMap<(u64, usize), usize>) -> usize {
    if let Some(&count) = memo.get(&(stone, iters)) {
        return count;
    }

    let ret = if iters == 1 {
        next_stone(stone).count()
    } else {
        next_stone(stone)
            .map(|stone| calculate_memoed(stone, iters - 1, memo))
            .sum()
    };
    memo.insert((stone, iters), ret);
    ret
}

fn next_stone(stone: u64) -> impl Iterator<Item = u64> {
    if stone == 0 {
        [Some(1), None]
    } else {
        let len = stone.ilog10() + 1;
        if len % 2 == 0 {
            let half = len / 2;
            let first = stone / (10u64.pow(half));
            let second = stone % 10u64.pow(half);
            [Some(first), Some(second)]
        } else {
            [Some(stone * 2024), None]
        }
    }
    .into_iter()
    .flatten()
}
