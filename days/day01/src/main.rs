#![feature(type_alias_impl_trait)]
#![feature(array_windows)]
#![feature(array_chunks)]
#![feature(slice_take)]
#![feature(iter_array_chunks)]
#![feature(split_as_slice)]
#![feature(coroutines)]
#![feature(slice_split_once)]
#![feature(impl_trait_in_assoc_type)]
#![feature(array_try_map)]
#![allow(unused)]

use bstr::B;
use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let (mut l1, mut l2): (Vec<_>, Vec<_>) = input
            .split(|&b| b == b'\n')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let v = s
                    .split(|&b| b == b' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| parse_ascii::<i32>(s).unwrap())
                    .collect_vec();
                (v[0], v[1])
            })
            .unzip();
        l1.sort();
        l2.sort();
        l1.iter()
            .zip(&l2)
            .map(|(&a, &b)| a.abs_diff(b))
            .sum::<u32>()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let (mut l1, mut l2): (Vec<_>, Vec<_>) = input
            .split(|&b| b == b'\n')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let v = s
                    .split(|&b| b == b' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| parse_ascii::<i32>(s).unwrap())
                    .collect_vec();
                (v[0], v[1])
            })
            .unzip();

        let l2 = l2.into_iter().counts();
        l1.into_iter()
            .map(|n| l2.get(&n).copied().unwrap_or_default() * n as usize)
            .sum::<usize>()
    }
}
