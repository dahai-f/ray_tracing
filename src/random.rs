use rand::prelude::*;
use std::cell::*;

thread_local!(static RNG: RefCell<ThreadRng> = RefCell::new(rand::thread_rng()));

pub struct Random;

impl Random {
    pub fn gen<T>() -> T
    where
        rand::distributions::Standard: Distribution<T>,
    {
        RNG.with(|rng| rng.borrow_mut().gen::<T>())
    }

    pub fn with_rng<T, F>(f: F) -> T
    where
        F: FnOnce(&mut ThreadRng) -> T,
    {
        RNG.with(|rng| f(&mut rng.borrow_mut()))
    }
}
