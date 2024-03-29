use std::ops::DerefMut;
use std::sync::{Mutex, OnceLock};

use rand::distributions::{DistIter, Distribution, Standard};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

static GLOBAL_RNG: OnceLock<Mutex<DefaultRng>> = OnceLock::new();
pub type DefaultRng = XorShiftRng;

pub fn global_rng() -> impl DerefMut<Target = DefaultRng> {
    GLOBAL_RNG.get_or_init(|| Mutex::new(rng())).lock().unwrap()
}

pub fn random<T>() -> T
where
    Standard: Distribution<T>,
{
    global_rng().gen()
}

pub fn rng() -> DefaultRng {
    XorShiftRng::seed_from_u64(2024)
}

pub trait RngExt: Rng {
    /// Borrows `self`.
    ///
    /// This exists for the same reason [`Iterator::by_ref`] exists.
    fn by_ref(&mut self) -> &mut Self {
        self
    }

    /// Calls [`Rng::sample_iter`] with the [`Standard`] distribution.
    ///
    /// This function is to `sample_iter` as [`Rng::gen`] is to [`Rng::sample`].
    fn gen_iter<T>(self) -> DistIter<Standard, Self, T>
    where
        Standard: Distribution<T>,
        Self: Sized,
    {
        self.sample_iter(Standard)
    }
}
