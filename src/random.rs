use std::hash::Hasher;

use rand::{SeedableRng, Rng, distributions::uniform::{SampleUniform, SampleRange}};
use rand_xoshiro::Xoroshiro128PlusPlus;
use wyhash::WyHash;

/// Pseudo random number generator. Wrapper struct around Xoroshiro128++.
/// (Newtype pattern: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html)
pub struct PRng(Xoroshiro128PlusPlus);

impl PRng {

    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.0.gen_range::<T, R>(range)
    }

    pub fn gen_ratio(&mut self, numerator: u32, denominator: u32) -> bool {
        self.0.gen_ratio(numerator, denominator)
    }
}

pub struct PRngBuilder {
    hasher: WyHash, 
}

impl PRngBuilder {
    /// Initializes a new builder with the specified seed.
    pub fn new_seeded(seed: u64) -> Self {
        Self { hasher: WyHash::with_seed(seed) }
    }

    /// Initialize a new builder with a randomly generated seed (thread-local).
    pub fn new() -> Self {
        Self { hasher: WyHash::with_seed(rand::random::<u64>()) }
    }

    pub fn write_i32(mut self, val: i32) -> Self {
        self.hasher.write_i32(val);
        self
    }

    pub fn write_u32(mut self, val: u32) -> Self {
        self.hasher.write_u32(val);
        self
    }

    pub fn write_u64(mut self, val: u64) -> Self {
        self.hasher.write_u64(val);
        self
    }

    pub fn build(self) -> PRng {
        PRng(Xoroshiro128PlusPlus::seed_from_u64(self.hasher.finish()))
    }
}
