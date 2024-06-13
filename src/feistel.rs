// Copyright 2019, Asim Ihsan
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

// Modified by RuneBlaze, 2024.

use rustc_data_structures::fx::FxHasher;
use core::hash::Hasher;

#[derive(Debug, Clone)]
pub struct Permutor {
    feistel: FeistelNetwork,
    pub max: u128,
    pub values_returned: u128,
}

impl Permutor {
    pub fn new_with_u64_key(max: u128, key: u64) -> Permutor {
        let key = key ^ 0xDEADBEEF_FEE1DEAD; // FIXME: No magic numbers should be put here.
        let key = u64_to_32slice(key);
        Permutor {
            feistel: FeistelNetwork::new_with_slice_key(max, key),
            max,
            values_returned: 0,
        }
    }

    pub fn new_with_slice_key(max: u128, key: [u8; 32]) -> Permutor {
        Permutor {
            feistel: FeistelNetwork::new_with_slice_key(max, key),
            max,
            values_returned: 0,
        }
    }

    pub fn forward(&self, plaintext: u128) -> u128 {
        let mut result = self.feistel.permute(plaintext);
        while result >= self.max {
            result = self.feistel.permute(result);
        }
        result
    }

    pub fn backward(&self, ciphertext: u128) -> u128 {
        let mut result = self.feistel.invert(ciphertext);
        while result >= self.max {
            result = self.feistel.invert(result);
        }
        result
    }
}

impl Iterator for Permutor {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values_returned < self.max {
            let next = self.forward(self.values_returned);
            self.values_returned += 1;
            return Some(next);
        }
        None
    }
}

/// Implements a Feistel network, which can take a non-invertible pseudo-random function (PRF)
/// and turn it into an invertible pseudo-random permutation (PRP).
///
/// If you use this struct directly note that its intended purpose is to be a PRP and map from
/// an n-bit input to an n-bit output, where n is an even positive integer. For example, if
/// constructed with a `max` of `10`, internally it creates a 4-bit Feistel network, and for all
/// integers in the 4-bit domain `[0, 16)` (`0` inclusive to `16` exclusive) it will map an input
/// to one and only one output, and vice-versa (a given output maps to one and only one input).
/// Even though you specified a max value of `10`, the output range may be larger than expected.
/// Clients like `RandomPermutor` handle this by excluding output values outside of the desired
/// range.
///
/// This is useful in fields like cryptography, where a block cipher is a PRP.
///
/// Another great use of a Feistel network is when you want some input to always map to one and only
/// one output (and vice versa). For example, given a 32-bit IP address, we could use some secret
/// key and map each IP address to some other 32-bit IP address. We could log this new 32-bit
/// IP address and people who do not know what the secret key is would find it difficult
/// to determine what the input IP address was. This is Format Preserving Encryption (FPE).
#[derive(Debug, Clone)]
pub struct FeistelNetwork {
    /// TODO visible just for testing, fix
    pub half_width: u128,

    /// Mask used to keep within the width for the right.
    /// TODO visible just for testing, fix
    pub right_mask: u128,

    /// Mask used to keep within the width for the left.
    /// TODO visible just for testing, fix
    pub left_mask: u128,

    /// Private key, some random seed. 256 bits as 32 bytes.
    key: [u8; 32],

    rounds: u8,
}

impl FeistelNetwork {
    pub fn new_with_slice_key(max_value: u128, key: [u8; 32]) -> FeistelNetwork {
        let mut width = integer_log2(max_value).unwrap();
        if width % 2 != 0 {
            width += 1;
        }
        let half_width = width / 2;
        let mut right_mask = 0;
        for i in 0..half_width {
            right_mask |= 1 << i;
        }
        let left_mask = right_mask << half_width;
        let num_rounds = 8 + (60 / integer_log2(max_value).unwrap().max(4));
        let num_rounds = num_rounds.min(32);
        FeistelNetwork {
            half_width: half_width as u128,
            right_mask,
            left_mask,
            key,
            rounds: num_rounds as u8,
        }
    }

    pub fn permute(&self, input: u128) -> u128 {
        let mut left = (input & self.left_mask) >> self.half_width;
        let mut right = input & self.right_mask;

        for i in 0..self.rounds {
            let new_left = right;
            let f = self.round_function(right, i, self.key, self.right_mask);
            right = left ^ f;
            left = new_left;
        }

        let result = (left << self.half_width) | right;
        result & (self.left_mask | self.right_mask)
    }

    pub fn invert(&self, input: u128) -> u128 {
        let mut left = (input & self.left_mask) >> self.half_width;
        let mut right = input & self.right_mask;

        for i in (0..self.rounds).rev() {
            let new_right = left;
            let f = self.round_function(left, i, self.key, self.right_mask);
            left = right ^ f;
            right = new_right;
        }

        let result = (left << self.half_width) | right;
        result & (self.left_mask | self.right_mask)
    }

    fn round_function(&self, right: u128, round: u8, key: [u8; 32], mask: u128) -> u128 {
        let right_bytes = u128::to_le_bytes(right);
        let round_bytes = u8_to_1slice(round);
        let mut hasher = FxHasher::default();
        hasher.write(&key[..]);
        hasher.write(&right_bytes[..]);
        hasher.write(&round_bytes[..]);
        hasher.write(&key[..]);
        (hasher.finish() as u128) & mask
    }
}

fn u8_to_1slice(input: u8) -> [u8; 1] {
    let mut result: [u8; 1] = [0; 1];
    result[0] = input;
    result
}

pub fn u64_to_32slice(input: u64) -> [u8; 32] {
    let result8 = u64::to_be_bytes(input);
    let mut result: [u8; 32] = [0; 32];
    result[..8].clone_from_slice(&result8[..8]);
    result
}

pub fn integer_log2(input: u128) -> Option<u32> {
    if input == 0 {
        return None;
    }
    Some(128 - input.leading_zeros())
}

#[cfg(test)]
mod tests {
    use ahash::AHashSet;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_invert_roundtrip(u: u128, v: u128, key: u64) -> TestResult {
        let (ub, input) = if u < v { (u, v) } else { (v, u) };
        if ub <= input || ub == 0 {
            return TestResult::discard();
        }
        let feistel = FeistelNetwork::new_with_slice_key(ub, u64_to_32slice(key));

        // Test inversion for all possible inputs
        let output = feistel.permute(input);
        let inverted = feistel.invert(output);
        assert_eq!(
            inverted, input,
            "Inversion does not produce the original input"
        );
        TestResult::passed()
    }

    #[quickcheck]
    fn test_permuter_bounded(u: u128, v: u128, key: u64) -> TestResult {
        let (ub, input) = if u < v { (u, v) } else { (v, u) };
        if ub <= input || ub == 0 {
            return TestResult::discard();
        }
        let permutor = Permutor::new_with_u64_key(ub, key);

        // Test that all values returned are within the bounds
        for value in permutor {
            assert!(value < ub, "Value returned is not within the bounds");
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn test_permuter_roundtrip(u: u128, v: u128, key: u64) -> TestResult {
        let (ub, input) = if u < v { (u, v) } else { (v, u) };
        if ub <= input || ub == 0 {
            return TestResult::discard();
        }
        let permutor = Permutor::new_with_u64_key(ub, key);

        // Test that all values returned are within the bounds
        for value in 0..ub {
            let forward = permutor.forward(value);
            let backward = permutor.backward(forward);
            assert_eq!(backward, value, "Forward and backward do not match");
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn test_permuter_covers_all(ub: u16, key: u64) -> TestResult {
        if ub == 0 {
            return TestResult::discard();
        }
        let permutor = Permutor::new_with_u64_key(ub as u128, key);
        let seen = AHashSet::from_iter(permutor);
        assert_eq!(seen.len(), ub as usize, "Not all values were returned");
        for value in 0..(ub as u128) {
            assert!(seen.contains(&value), "Value was not returned");
        }
        TestResult::passed()
    }
}
