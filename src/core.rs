use num_prime::nt_funcs::next_prime;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use rand::rngs::OsRng;
use rand_unique::{RandomSequence, RandomSequenceBuilder};
use std::collections::HashSet;

trait Bijections {
    fn forward(&self, val: u64) -> u64;
}

#[derive(Debug)]
pub struct RestrictedPrimePerm {
    a: u64,
    b: u64,
    limit: u64,
    effective_limit: u64,
    first_next: Option<u64>,
}

impl Iterator for RestrictedPrimePerm {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let initial_next = (self.a + self.b) % self.limit;
        // If initial_next is already within the effective limit, return it
        if initial_next < self.effective_limit {
            self.a = initial_next;
            if self.first_next.is_none() {
                self.first_next = Some(initial_next);
            }
            return Some(initial_next);
        }

        // Calculate the number of steps needed to make the value within the effective limit
        let k = (initial_next + self.b - self.effective_limit) / self.limit + 1;
        // Calculate the next valid value
        let next = (initial_next + k * self.b) % self.limit;
        if Some(next) == self.first_next {
            return None;
        }
        self.a = next;
        if self.first_next.is_none() {
            self.first_next = Some(next);
        }
        Some(next)
    }
}

impl RestrictedPrimePerm {
    pub fn new(a: u64, b: u64, effective_limit: u64) -> RestrictedPrimePerm {
        let limit = next_prime(&effective_limit, None);
        if let Some(limit) = limit {
            RestrictedPrimePerm {
                a,
                b,
                limit,
                effective_limit,
                first_next: None,
            }
        } else {
            panic!("Error: overflow in next_prime");
        }
    }

    pub fn from_rng(rng: &mut Xoshiro256PlusPlus, effective_limit: u64) -> RestrictedPrimePerm {
        let a = rng.gen_range(0..effective_limit);
        let b = rng.gen_range(1..effective_limit);
        RestrictedPrimePerm::new(a, b, effective_limit)
    }

    pub fn from_seed(seed: u64, effective_limit: u64) -> RestrictedPrimePerm {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        RestrictedPrimePerm::from_rng(&mut rng, effective_limit)
    }
}

#[derive(Debug)]
struct SmallTableBijection {
    size: u64,
    period: u32,
    lookup: Vec<u32>,
    last_lookup: Vec<u32>,
}

impl SmallTableBijection {
    pub fn from_seed(seed: u64, size: u64, period: u32) -> SmallTableBijection {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        let mut lookup = (0..period).collect::<Vec<u32>>();
        lookup.shuffle(&mut rng);
        let rest_size = (size % period as u64) as u32;
        let mut last_lookup = (0..rest_size).collect::<Vec<u32>>();
        last_lookup.shuffle(&mut rng);
        SmallTableBijection {
            size,
            period,
            lookup,
            last_lookup,
        }
    }
}

impl Bijections for SmallTableBijection {
    fn forward(&self, val: u64) -> u64 {
        if val < (self.size / self.period as u64) * self.period as u64 {
            let period_index = (val % self.period as u64) as usize;
            let period_group = val / self.period as u64;
            (period_group * self.period as u64) + self.lookup[period_index] as u64
        } else {
            let last_index = (val % self.period as u64) as usize;
            let last_group = (self.size / self.period as u64) * self.period as u64;
            last_group + self.last_lookup[last_index] as u64
        }
    }
}

struct PingPongBijection {
    size: u64,
    period: u32,
}

impl PingPongBijection {
    pub fn new(size: u64, period: u32) -> PingPongBijection {
        PingPongBijection { size, period }
    }
}

impl Bijections for PingPongBijection {
    fn forward(&self, val: u64) -> u64 {
        let period_index = (val % self.period as u64) as usize;
        let period_group = val / self.period as u64;
        let is_last_block = period_group == self.size / self.period as u64;

        if is_last_block {
            let rest_size = self.size % self.period as u64;
            if period_index < rest_size as usize {
                val
            } else {
                self.size - 1 - (val - rest_size)
            }
        } else {
            let period_offset = period_group % 2;
            let period_size = self.period as u64;
            let period_start = period_group * period_size;
            if period_offset == 0 {
                period_start + period_index as u64
            } else {
                period_start + period_size - period_index as u64 - 1
            }
        }
    }
}

pub struct PseudoPermutation {
    perm: RestrictedPrimePerm,
    rand_seq: GroupOfRandomSeq,
    modulo: u64, // next largest power-of-two w.r.t. n
    n: u64,
}

pub struct GroupOfRandomSeq {
    // from u8 to u64
    rseq1: RandomSequence<u8>,
    rseq2: RandomSequence<u16>,
    rseq3: RandomSequence<u32>,
    rseq4: RandomSequence<u64>,
}

impl GroupOfRandomSeq {
    pub fn new(seed: u64) -> GroupOfRandomSeq {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        let rseq1 = RandomSequenceBuilder::<u8>::rand(&mut rng);
        let rseq2 = RandomSequenceBuilder::<u16>::rand(&mut rng);
        let rseq3 = RandomSequenceBuilder::<u32>::rand(&mut rng);
        let rseq4 = RandomSequenceBuilder::<u64>::rand(&mut rng);
        GroupOfRandomSeq {
            rseq1: rseq1.into_iter(),
            rseq2: rseq2.into_iter(),
            rseq3: rseq3.into_iter(),
            rseq4: rseq4.into_iter(),
        }
    }

    pub fn n_smaller_than(&self, n: u64, limit: u64) -> u64 {
        if limit < 256 {
            self.rseq1.n(n as u8) as u64
        } else if limit < 65536 {
            self.rseq2.n(n as u16) as u64
        } else if limit < 4294967296 {
            self.rseq3.n(n as u32) as u64
        } else {
            self.rseq4.n(n)
        }
    }

    pub fn modulo(&self, limit: u64) -> u64 {
        if limit < 256 {
            256
        } else if limit < 65536 {
            65536
        } else if limit < 4294967296 {
            4294967296
        } else {
            0
        }
    }
}

impl PseudoPermutation {
    pub fn from_seed(seed: u64, n: u64) -> PseudoPermutation {
        let perm = RestrictedPrimePerm::from_seed(seed, n);
        let mut xorshift_seed = Xoshiro256PlusPlus::seed_from_u64(seed);
        let rand_seq = GroupOfRandomSeq::new(seed);
        let modulo = 1u64 << (64 - n.leading_zeros());
        PseudoPermutation {
            perm,
            rand_seq: rand_seq,
            modulo,
            n,
        }
    }
}

impl Iterator for PseudoPermutation {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let mut val = self.perm.next()?;
        let modulo = self.rand_seq.modulo(self.n);
        let mut mapped = self.rand_seq.n_smaller_than(val, self.n) % modulo;

        while mapped >= self.n {
            val = self.perm.next()?;
            mapped = self.rand_seq.n_smaller_than(val, self.n) % modulo;
        }
        Some(mapped)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::feistel::{integer_log2, Permutor};

    use super::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn test_next() {
        let mut perm = RestrictedPrimePerm::new(2, 3, 10);
        let results: Vec<u64> = perm.take(10).collect();
        // Check the generated values are within the effective limit
        for &val in &results {
            assert!(val < 10);
        }
    }

    #[test]
    fn test_cycle_detection() {
        let mut perm = RestrictedPrimePerm::from_seed(0, 10);
        let mut seen = std::collections::HashSet::new();
        for _ in 0..10 {
            let val = perm.next().unwrap();
            // Check for cycles
            if seen.contains(&val) {
                break;
            }
            seen.insert(val);
        }
        // The set should contain all unique values
        assert_eq!(seen.len(), 10);
    }

    #[test]
    fn test_new() {
        let perm = RestrictedPrimePerm::new(2, 3, 10);
        assert_eq!(perm.a, 2);
        assert_eq!(perm.b, 3);
        assert_eq!(perm.effective_limit, 10);
        assert!(perm.limit > 10); // Since limit should be the next prime number greater than effective_limit
    }

    #[test]
    fn test_from_rng() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
        let perm = RestrictedPrimePerm::from_rng(&mut rng, 10);
        assert!(perm.a < 10);
        assert!(perm.b < 10);
        assert_eq!(perm.effective_limit, 10);
        assert!(perm.limit > 10);
    }

    #[test]
    fn test_big_limit() {
        let mut perm = RestrictedPrimePerm::new(1000000 - 40, 11, 1000000);
        let results: Vec<u64> = perm.take(10).collect();
        // Check the generated values are within the effective limit
        for &val in &results {
            assert!(val < 1000000);
        }
        assert!(results.len() == 10);
    }

    #[test]
    fn test_bijection() {
        let size = 102;
        let period = 10;
        let seed = 42;
        let bijection = SmallTableBijection::from_seed(seed, size, period);
        let mut seen = HashSet::new();

        for i in 0..size {
            let mapped = bijection.forward(i);
            if seen.contains(&mapped) {
                assert!(false, "Value {} is mapped to {} multiple times.", i, mapped);
            }
            seen.insert(mapped);
        }

        assert_eq!(
            seen.len() as u64,
            size,
            "Not all values are mapped uniquely."
        );
    }

    #[test]
    fn test_pingpong_bijection() {
        let size = 102;
        let period = 10;
        let bijection = PingPongBijection::new(size, period);
        let mut seen = HashSet::new();

        for i in 0..size {
            let mapped = bijection.forward(i);
            if seen.contains(&mapped) {
                assert!(false, "Value {} is mapped to {} multiple times.", i, mapped);
            }
            seen.insert(mapped);
        }

        assert_eq!(
            seen.len() as u64,
            size,
            "Not all values are mapped uniquely."
        );
    }

    #[test]
    fn test_pseudo_permutation() {
        let n = 100;
        let seed = 42;
        let mut perm = Permutor::new_with_u64_key(n, seed);
        let mut seen = HashSet::new();
        for i in 0..n {
            let val = perm.next().unwrap();
            if seen.contains(&val) {
                assert!(false, "Value {} is mapped to {} multiple times.", i, val);
            }
            seen.insert(val);
        }
        assert_eq!(seen.len() as u128, n, "Not all values are mapped uniquely.");
    }

    #[test]
    fn test_integer_log() {
        assert_eq!(None, integer_log2(0), "failed for {}", 0);
        assert_eq!(Some(1), integer_log2(1), "failed for {}", 1);
        assert_eq!(Some(2), integer_log2(2), "failed for {}", 2);
        assert_eq!(Some(2), integer_log2(3), "failed for {}", 3);
        assert_eq!(Some(3), integer_log2(4), "failed for {}", 4);
        assert_eq!(Some(3), integer_log2(5), "failed for {}", 5);
        assert_eq!(Some(3), integer_log2(6), "failed for {}", 6);
        assert_eq!(Some(3), integer_log2(7), "failed for {}", 7);
        assert_eq!(Some(4), integer_log2(8), "failed for {}", 8);
        assert_eq!(Some(4), integer_log2(9), "failed for {}", 9);
        assert_eq!(Some(4), integer_log2(10), "failed for {}", 10);
    }
}
