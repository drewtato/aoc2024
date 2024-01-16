#![feature(array_try_from_fn)]
#![feature(iter_from_coroutine)]

use std::io::stdin;
use std::ops::{Add, Div, Mul};
use std::str::FromStr;

pub use std::array::{from_fn as from_fn_array, try_from_fn};
pub use std::cmp::Reverse;
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
pub use std::convert::identity;
pub use std::fmt::{Debug, Display};
pub use std::iter::{
	empty as empty_iter, from_coroutine as gen_iter, from_fn as from_fn_iter, once as once_iter,
	once_with as once_with_iter, repeat as repeat_iter, repeat_with as repeat_with_iter,
	successors,
};
pub use std::mem::{replace, swap, take};

pub use itertools::Itertools;
pub use regex::bytes::Regex;

pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub use num_integer::*;

pub use primal::*;

/// Computes the triangular number.
///
/// # Example
/// ```
/// # use helpers::triangular_number;
/// for (n, ans) in [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55].into_iter().enumerate() {
///     assert_eq!(triangular_number(n), ans);
/// }
/// ```
pub fn triangular_number<N>(n: N) -> N
where
	N: Add<Output = N> + Mul<Output = N> + Div<Output = N> + TryFrom<u8> + Copy,
	N::Error: Debug,
{
	n * (n + 1u8.try_into().unwrap()) / 2u8.try_into().unwrap()
}

/// Reads a value from standard input.
///
/// Panics if reading from stdin fails. Returns an error if parsing the
/// resulting string fails.
pub fn read_value<T>() -> Result<T, T::Err>
where
	T: FromStr,
{
	stdin().lines().next().unwrap().unwrap().trim().parse()
}

/// Waits for a newline from stdin.
pub fn pause() {
	let line = stdin().lines().next().unwrap().unwrap();
	if line.trim() == "q" {
		std::process::exit(0)
	}
}
