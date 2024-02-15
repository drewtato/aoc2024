#![feature(type_alias_impl_trait)]
#![feature(array_windows)]
#![feature(array_chunks)]
#![feature(byte_slice_trim_ascii)]
#![feature(slice_take)]
#![feature(iter_array_chunks)]
#![feature(split_as_slice)]
#![feature(coroutines)]
#![feature(slice_split_once)]
#![feature(impl_trait_in_assoc_type)]
#![feature(array_try_map)]
#![allow(unused)]

use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display {
        "unimplemented"
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display {
        "unimplemented"
    }
}
