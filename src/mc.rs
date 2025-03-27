//! the Monte Carlo test.

use super::*;

const MONTE_LEN: usize = 6;
const MONTE_LEN_HALF: usize = MONTE_LEN / 2;

/// python: ((256 ** 3) - 1) ** 2
const IN_CIRCLE_DISTANCE: Dec = Dec::from_u64(281_474_943_156_225);

/// See <https://www.geeksforgeeks.org/estimating-value-pi-using-monte-carlo>
#[derive(Debug, Copy, Clone)]
pub struct MonteCarloCalculation {
    /// Bytes used by Monte Carlo coordinates
    monte: [u8; MONTE_LEN],
    /// Accumulator pointer
    accumulator: usize,
    /// Tries
    tries: usize,
    /// Inside Count
    in_count: usize,
}

impl Default for MonteCarloCalculation {
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
    pub const fn new() -> Self {
        Self::INIT
    }

    /// apply byte stream to monte-carlo state.
    pub const fn update(&mut self, bytes: &[u8]) -> &mut Self {
        let bytes_len = bytes.len();

        let mut x;
        let mut y;

        let mut i = 0;
        let mut j;
        while i < bytes_len {
            self.monte[self.accumulator] = bytes[i];
            i += 1;

            self.accumulator += 1;
            if self.accumulator >= MONTE_LEN {
                self.accumulator = 0;
                self.tries += 1;

                x = dec!(0.0);
                y = dec!(0.0);
                j = 0;
                while j < MONTE_LEN_HALF {
                    x = x.mul(dec!(256.0)).add(Dec::from_u8(self.monte[j]));
                    y = y.mul(dec!(256.0)).add(Dec::from_u8(self.monte[j + 3]));

                    j += 1;
                }
                if x.mul(x).add(y.mul(y)).lt(&IN_CIRCLE_DISTANCE) {
                    self.in_count += 1;
                }
            }
        }

        self
    }

    /// get finalize monte-carlo result of current byte stream.
    pub const fn finalize(&self) -> Dec {
        if self.tries == 0 {
            return Dec::NAN;
        }
        let in_count = Dec::from_usize(self.in_count);
        let tries = Dec::from_usize(self.tries);
        dec!(4.0).mul(in_count.div(tries))
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

impl EntropyTest for MonteCarloCalculation {
    fn update(&mut self, bytes: &[u8]) {
        Self::update(self, bytes);
    }

    fn finalize(&self) -> Dec {
        Self::finalize(self)
    }
}

