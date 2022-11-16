use super::*;

/// See <https://www.geeksforgeeks.org/estimating-value-pi-using-monte-carlo>
#[derive(Default, Debug, Clone, Copy)]
pub struct MonteCarloCalculation {
    /// Bytes used by Monte Carlo coordinates
    monte: [usize; 6],
    /// Accumulator pointer
    accumulator: usize,
    /// Tries
    tries: usize,
    /// Inside Count
    in_count: usize,
}

impl EntropyTester for MonteCarloCalculation {
    fn update<B: AsRef<[u8]>>(&mut self, stream: B) {
        for b in stream.as_ref() {
            self.monte[self.accumulator] = *b as usize;
            self.accumulator += 1;

            if self.accumulator == self.monte.len() {
                self.accumulator = 0;
                self.tries += 1;

                let (mut x, mut y) = (0.0, 0.0);
                for j in 0..self.monte.len() / 2 {
                    x = x * 256.0 + (self.monte[j] as f64);
                    y = y * 256.0 + (self.monte[j + 3] as f64);
                }

                if (x * x + y * y) < MonteCarloCalculation::IN_CIRCLE_DISTANCE {
                    self.in_count += 1;
                }
            }
        }
    }

    fn finalize(&mut self) -> f64 {
        4.0 * ((self.in_count as f64) / (self.tries as f64))
    }
}

impl MonteCarloCalculation {
    /// (256**3-1)**2
    const IN_CIRCLE_DISTANCE: f64 = 281_474_943_156_225f64;
}
