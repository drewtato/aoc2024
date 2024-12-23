use std::sync::atomic::AtomicU16;

use helpers::*;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        rayon::ThreadPoolBuilder::new()
            .stack_size(32_000_000)
            .build_global()
            .unwrap();
        Self::for_each_number(input, def, one, |a, b| a + b, identity)
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut ans = AtomicU16::new(0);
        rayon::scope(|scope| {
            scope.spawn(|_| {
                ans.store(
                    Self::for_each_number(
                        input,
                        def,
                        two,
                        |mut a, b| {
                            for (na, &nb) in a
                                .iter_mut()
                                .flatten()
                                .flatten()
                                .flatten()
                                .zip(b.iter().flatten().flatten().flatten())
                            {
                                *na += nb;
                            }
                            a
                        },
                        two_answer,
                    ),
                    std::sync::atomic::Ordering::Relaxed,
                )
            });
        });
        *ans.get_mut()
    }
}

fn one(total: &mut i64, mut n: i64) {
    for _ in 0..2000 {
        n = iterate(n);
    }
    *total += n;
}

fn two(change_totals: &mut [[[[u16; 19]; 19]; 19]; 19], n: i64) {
    let mut seen: [[[[bool; 19]; 19]; 19]; 19] = def();
    let mut iter = successors(Some(n), |&n| Some(iterate(n)))
        .map(|n| n % 10)
        .take(2000);
    let mut last_price = iter.next().unwrap();
    let mut last_diffs: [i64; 3] = from_fn_array(|_| {
        let next_price = iter.next().unwrap();
        let diff = next_price - last_price;
        last_price = next_price;
        diff
    });
    for price in iter {
        let diff = price - last_price;
        last_price = price;

        let [a, b, c] = last_diffs;
        if !replace(
            &mut seen[(a + 9) as usize][(b + 9) as usize][(c + 9) as usize][(diff + 9) as usize],
            true,
        ) {
            change_totals[(a + 9) as usize][(b + 9) as usize][(c + 9) as usize]
                [(diff + 9) as usize] += price as u16;
        }
        last_diffs = [last_diffs[1], last_diffs[2], diff];
    }
}

fn two_answer(change_totals: [[[[u16; 19]; 19]; 19]; 19]) -> u16 {
    change_totals
        .iter()
        .flatten()
        .flatten()
        .flatten()
        .copied()
        .max()
        .unwrap()
}

impl Solver {
    fn for_each_number<I: Send, R>(
        input: &[u8],
        init: impl Fn() -> I + Sync + Send,
        f: impl Fn(&mut I, i64) + Sync + Send,
        combine: impl Fn(I, I) -> I + Sync + Send,
        finish: impl FnOnce(I) -> R,
    ) -> R {
        let mut con = Consume::new(input);
        let iter = fn_iter(|| {
            if con.is_empty() {
                return None;
            }
            let n = con.int().unwrap();
            con.newline();
            Some(n)
        });
        let res = iter
            .par_bridge()
            .fold(init, |mut i, n| {
                f(&mut i, n);
                i
            })
            .reduce_with(combine)
            .unwrap();
        finish(res)
    }
}

fn iterate(mut secret: i64) -> i64 {
    secret = mix_and_prune(secret, secret * 64);
    secret = mix_and_prune(secret, secret / 32);
    secret = mix_and_prune(secret, secret * 2048);
    secret
}

fn mix_and_prune(mut secret: i64, mix: i64) -> i64 {
    secret ^= mix;
    secret %= 16777216;
    secret
}
