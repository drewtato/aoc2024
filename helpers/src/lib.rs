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
    empty as empty_iter, from_coroutine as coroutine_iter, from_fn as fn_iter, once as once_iter,
    once_with as once_with_iter, repeat as repeat_iter, repeat_with as repeat_with_iter,
    successors,
};
pub use std::mem::{replace, swap, take};

pub use itertools::Itertools;
pub use regex::bytes::Regex;

pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub use num_integer::*;

pub use primal::*;

pub trait UnwrapDisplay {
    type Output;
    /// Like [`Result::unwrap`], but prints with `Display` instead of `Debug`.
    ///
    /// # Examples
    ///
    /// Unwrapping an `Ok` works normally.
    ///
    /// ```
    /// # use helpers::UnwrapDisplay;
    /// let opt: Result<i32, &'static str> = Ok(4);
    /// assert_eq!(4, opt.unwrap_display());
    /// ```
    ///
    /// Unwrapping an `Err` prints using `Display` and panics.
    ///
    /// ```should_panic
    /// # use helpers::UnwrapDisplay;
    /// let opt: Result<i32, &'static str> = Err("oh no");
    /// opt.unwrap_display(); // panics
    /// ```
    fn unwrap_display(self) -> Self::Output;
}

impl<T, E> UnwrapDisplay for Result<T, E>
where
    E: Display,
{
    type Output = T;

    fn unwrap_display(self) -> Self::Output {
        self.unwrap_or_else(|e| panic!("{e}"))
    }
}

/// Short version of [`Default::default`].
pub fn def<D: Default>() -> D {
    D::default()
}

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

/// Creates a [`HashSet`] from a list of values.
///
/// # Examples
///
/// ```
/// # use helpers::{hashset, HashSet};
/// let set = hashset! { 0, 1, 2 };
/// assert_eq!(set, HashSet::from_iter([0, 1, 2]));
/// ```
#[macro_export]
macro_rules! hashset {
	($($i:expr),* $(,)?) => {
		HashSet::from_iter([$($i),*])
	};
}

/// Creates a [`HashMap`] from a list of values.
///
/// # Examples
///
/// ```
/// # use helpers::{hashmap, HashMap};
/// let map = hashmap! {
///     0 => "a",
///     1 => "b",
///     2 => "c",
/// };
/// assert_eq!(map, HashMap::from_iter([
///     (0, "a"),
///     (1, "b"),
///     (2, "c"),
/// ]));
/// ```
#[macro_export]
macro_rules! hashmap {
	($($k:expr => $v:expr),* $(,)?) => {
		HashMap::from_iter([$(($k, $v)),*])
	};
}

/// Creates a [`BTreeSet`] from a list of values.
///
/// # Examples
///
/// ```
/// # use helpers::{btreeset, BTreeSet};
/// let set = btreeset! { 0, 1, 2 };
/// assert_eq!(set, BTreeSet::from_iter([0, 1, 2]));
/// ```
#[macro_export]
macro_rules! btreeset {
	($($i:expr),* $(,)?) => {
		BTreeSet::from_iter([$($i),*])
	};
}

/// Creates a [`BTreeMap`] from a list of values.
///
/// # Examples
///
/// ```
/// # use helpers::{btreemap, BTreeMap};
/// let map = btreemap! {
///     0 => "a",
///     1 => "b",
///     2 => "c",
/// };
/// assert_eq!(map, BTreeMap::from_iter([
///     (0, "a"),
///     (1, "b"),
///     (2, "c"),
/// ]));
/// ```
#[macro_export]
macro_rules! btreemap {
	($($k:expr => $v:expr),* $(,)?) => {
		BTreeMap::from_iter([$(($k, $v)),*])
	};
}
