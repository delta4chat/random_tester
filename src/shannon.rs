//! the Shannon entropy test.

use super::*;

/// Computes the Shannon Entropy test
#[derive(Debug, Copy, Clone)]
pub struct ShannonCalculation {
    pub(crate) buckets: [u64; 256],
    pub(crate) total_buckets: u64,
}

impl Default for ShannonCalculation {
    fn default() -> Self {
        Self::INIT
    }
}

impl ShannonCalculation {
    /// the blanket state (initial value) of [ShannonCalculation].
    pub const INIT: Self =
        Self {
            buckets: [0; 256],
            total_buckets: 0,
        };

    /// create new blanket state for shannon calculation.
    ///
    /// this just copy from [ShannonCalculation::INIT].
    pub const fn new() -> Self {
        Self::INIT
    }

    /// apply byte stream to shannon state.
    pub const fn update(&mut self, bytes: &[u8]) -> &mut Self {
        let mut i = 0;
        let bytes_len = bytes.len();
        while i < bytes_len {
            self.buckets[bytes[i] as usize] += 1;
            i += 1;
        }
        self.total_buckets += bytes_len as u64;

        self
    }

    /// get finalize shannon result of current byte stream.
    pub const fn finalize(&self) -> Dec {
        if self.total_buckets == 0 {
            return Dec::NAN;
        }

        let length = Dec::from_u64(self.total_buckets);
        let mut entropy = dec!(0.0);

        let mut i = 0;
        let mut probability;
        let mut p;
        while i < 256 {
            probability = Dec::from_u64(self.buckets[i]).div(length);
            if probability.gt(&Dec::ZERO) {
                p = Dec::ONE.div(probability).log2();
                entropy = entropy.add(probability.mul(p));
            }
            i += 1;
        }
        entropy
    }

    /// get the samples of current state.
    pub const fn samples(&self) -> u64 {
        self.total_buckets
    }

    /// oneshot test function for small data.
    ///
    /// this is equivalent to `Self::new().update(data).finalize()`.
    pub const fn test(data: &[u8]) -> Dec {
        let mut this = Self::INIT;
        this.update(data);
        this.finalize()
    }
}

impl EntropyTest for ShannonCalculation {
    fn update(&mut self, bytes: &[u8]) {
        Self::update(self, bytes);
    }

    fn finalize(&self) -> Dec {
        Self::finalize(self)
    }
}
