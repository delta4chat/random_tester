use super::*;

/// Computes the serial correlation coefficient. In the event of input all 0s, reporst 1.0
#[derive(Debug, Clone, Copy)]
pub struct SerialCorrelationCoefficientCalculation {
    /// first time
    first: bool,
    /// term 1
    t1: f64,
    /// term 2
    t2: f64,
    /// term 3
    t3: f64,
    /// last byte
    last: f64,
    /// first byte
    u0: f64,
    /// total bytes processed
    total: usize,
}

impl Default for SerialCorrelationCoefficientCalculation {
    fn default() -> Self {
        Self {
            first: true,
            t1: 0.0,
            t2: 0.0,
            t3: 0.0,
            last: 0.0,
            u0: 0.0,
            total: 0,
        }
    }
}

impl EntropyTester for SerialCorrelationCoefficientCalculation {
    fn update<B: AsRef<[u8]>>(&mut self, stream: B) {
        let stream = stream.as_ref();
        let mut i = 0;
        if self.first {
            self.first = false;
            self.last = 0.0;
            self.u0 = stream[0] as f64;
            i = 1;
        }

        while i < stream.len() {
            let un = stream[i] as f64;
            self.t1 += self.last * un;
            self.t2 += un;
            self.t3 += un * un;
            self.last = un;

            i += 1;
        }

        self.total += stream.len();
    }

    fn finalize(&mut self) -> f64 {
        let total = self.total as f64;
        let t1 = self.t1 + self.last * self.u0;
        let t2 = self.t2 * self.t2;
        let mut scc = total * self.t3 - t2;

        if scc == 0.0 {
            // Should never see scc = 0.0 for non-zero inputs.
            // Declare this as positively correlated.
            scc = 1.0;
        } else {
            scc = (total * t1 - t2) / scc;
        }
        scc
    }
}
