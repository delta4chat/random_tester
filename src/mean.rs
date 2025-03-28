//! the Mean entropy test.

use super::*;

/// Computes the Mean Entropy test
#[derive(Debug, Clone, Copy)]
pub struct MeanCalculation {
    buckets: [usize; 256],
    total_buckets: usize,
}

impl Default for MeanCalculation {
    fn default() -> Self {
        Self::INIT
    }
}

impl MeanCalculation {
    /// the blanket state (initial value) of [MeanCalculation].
    pub const INIT: Self =
        Self {
            buckets: [0; 256],
            total_buckets: 0,
        };

    /// create new blanket state for mean calculation.
    ///
    /// this just copy from [MeanCalculation::INIT].
    pub const fn new() -> Self {
        Self::INIT
    }

    /// apply byte stream to mean state.
    pub const fn update(&mut self, bytes: &[u8]) -> &mut Self {
        let mut i = 0;
        let bytes_len = bytes.len();
        while i < bytes_len {
            self.buckets[bytes[i] as usize] += 1;
            self.total_buckets += 1;

            i += 1;
        }

        self
    }

    /// get finalize mean result of current byte stream.
    pub const fn finalize(&self) -> Dec {
        if self.total_buckets == 0 {
            return Dec::NAN;
        }

        let mut sum = dec!(0.0);

        let mut i = 0;
        let mut index;
        let mut bucket;
        while i < 256 {
            index = Dec::from_usize(i);
            bucket = Dec::from_usize(self.buckets[i]);
            sum = sum.add(index.mul(bucket));

            i += 1;
        }

        sum.div(Dec::from_usize(self.total_buckets))
    }

    /// get the samples of current state.
    pub const fn samples(&self) -> usize {
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

impl EntropyTest for MeanCalculation {
    fn update(&mut self, bytes: &[u8]) {
        Self::update(self, bytes);
    }

    fn finalize(&self) -> Dec {
        Self::finalize(self)
    }
}

