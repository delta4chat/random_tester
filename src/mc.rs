//! the Monte Carlo test.

use super::*;

const MONTE_LEN: usize = 6;
const MONTE_LEN_HALF: usize = MONTE_LEN / 2;

/// python: ((256 ** 3) - 1) ** 2
const IN_CIRCLE_DISTANCE: u64 = 281_474_943_156_225;

/// See <https://www.geeksforgeeks.org/estimating-value-pi-using-monte-carlo>
#[derive(Debug, Copy, Clone)]
pub struct MonteCarloCalculation {
    /// Bytes used by Monte Carlo coordinates
    monte: [u8; MONTE_LEN],

    /// Accumulator pointer
    accumulator: usize,

    /// Tries
    tries: u64,

    /// Inside Count
    in_count: u64,
}

impl Default for MonteCarloCalculation {
    #[inline(always)]
    fn default() -> Self {
        Self::INIT
    }
}

impl MonteCarloCalculation {
    /// the blanket state (initial value) of [MonteCarloCalculation].
    pub const INIT: Self =
        Self {
            monte: [0; MONTE_LEN],
            accumulator: 0,
            tries: 0,
            in_count: 0,
        };

    /// create new blanket state for monte-carlo calculation.
    ///
    /// this just copy from [MonteCarloCalculation::INIT].
    #[inline(always)]
    pub const fn new() -> Self {
        Self::INIT
    }

    /// apply byte stream to monte-carlo state.
    #[inline(always)]
    pub const fn update(&mut self, bytes: &[u8]) -> &mut Self {
        let bytes_len = bytes.len();

        let mut x: u64;
        let mut y: u64;

        let mut i = 0;
        let mut j;
        while i < bytes_len {
            self.monte[self.accumulator] = bytes[i];
            i += 1;

            self.accumulator += 1;
            if self.accumulator >= MONTE_LEN {
                self.accumulator = 0;
                self.tries += 1;

                x = 0;
                y = 0;
                j = 0;
                while j < MONTE_LEN_HALF {
                    x = (x * 256) + (self.monte[j] as u64);
                    y = (y * 256) + (self.monte[j + 3] as u64);

                    j += 1;
                }

                if (x * x) + (y * y) < IN_CIRCLE_DISTANCE {
                    self.in_count += 1;
                }
            }
        }

        self
    }

    /// get finalize monte-carlo result of current byte stream.
    #[inline(always)]
    pub const fn finalize(&self) -> Dec {
        if self.tries == 0 {
            return Dec::NAN;
        }
        let in_count = Dec::from_u64(self.in_count);
        let tries = Dec::from_u64(self.tries);
        dec!(4.0).mul(in_count.div(tries))
    }

    /// oneshot test function for small data.
    ///
    /// this is equivalent to `Self::new().update(data).finalize()`.
    #[inline(always)]
    pub const fn test(data: &[u8]) -> Dec {
        let mut this = Self::INIT;
        this.update(data);
        this.finalize()
    }
}

impl EntropyTest for MonteCarloCalculation {
    #[inline(always)]
    fn update(&mut self, bytes: &[u8]) {
        Self::update(self, bytes);
    }

    #[inline(always)]
    fn finalize(&self) -> Dec {
        Self::finalize(self)
    }
}

