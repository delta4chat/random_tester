//! the Serial Correlation Coefficient test.

use super::*;

/// Computes the serial correlation coefficient. In the event of input all 0s, reporst 1.0
#[derive(Debug, Clone, Copy)]
pub struct SerialCorrelationCoefficientCalculation {
    /// Whether all values is equal
    all_equals: bool,
    /// first time
    first: bool,
    /// term 1
    t1: u64,
    /// term 2
    t2: u64,
    /// term 3
    t3: u64,
    /// last byte
    last: u8,
    /// first byte
    u0: u8,
    /// total bytes processed
    total: u64,
}

impl Default for SerialCorrelationCoefficientCalculation {
    fn default() -> Self {
        Self::INIT
    }
}

impl SerialCorrelationCoefficientCalculation {
    /// the blanket state (initial value) of [SerialCorrelationCoefficientCalculation].
    pub const INIT: Self =
        Self {
            all_equals: true,
            first: true,
            t1: 0,
            t2: 0,
            t3: 0,
            last: 0,
            u0: 0,
            total: 0,
        };

    /// create new blanket state for serial-correlation-coefficient calculation.
    ///
    /// this just copy from [SerialCorrelationCoefficientCalculation::INIT].
    pub const fn new() -> Self {
        Self::INIT
    }

    /// apply byte stream to serial-correlation-coefficient state.
    pub const fn update(&mut self, bytes: &[u8]) -> &mut Self {
        let bytes_len = bytes.len();
        if bytes_len == 0 {
            return self;
        }

        let mut i = 0;
        if self.first {
            self.first = false;
            self.last = 0;
            self.u0 = bytes[0];
            i = 1;
        }

        let mut un;
        let mut b;
        while i < bytes_len {
            b = bytes[i];

            if self.all_equals && self.u0 != b {
                self.all_equals = false;
            }

            un = b as u64;
            // mul is loss-less for native integer type
            self.t1 += (self.last as u64) * un;
            self.t2 += un;
            self.t3 += un * un;
            self.last = b;

            i += 1;
        }
        self.total += bytes_len as u64;

        self
    }

    /// checks whether all bytes equals.
    pub const fn all_equals(&self) -> bool {
        self.all_equals
    }

    /// get finalize serial-correlation-coefficient result of current byte stream.
    pub const fn finalize(&self) -> Dec {
        if self.total == 0 || self.all_equals {
            return Dec::NAN;
        }

        let total = Dec::from_u64(self.total);
        let u0 = Dec::from_u8(self.u0);
        let t1 = Dec::from_u8(self.last).mul(u0).add(Dec::from_u64(self.t1));
        let t2 = Dec::from_u64(self.t2).powi(2);
        let mut scc = total.mul(Dec::from_u64(self.t3)).sub(t2);

        // Should never see scc = 0.0 for non-zero inputs and not "all-values equal".
        // Declare this as positively correlated.
        if scc.eq(&dec!(0.0)) {
            //scc = dec!(1.0);
        } else {
            scc = total.mul(t1).sub(t2).div(scc);
        }

        scc
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

impl EntropyTest for SerialCorrelationCoefficientCalculation {
    fn update(&mut self, bytes: &[u8]) {
        Self::update(self, bytes);
    }

    fn finalize(&self) -> Dec {
        Self::finalize(self)
    }
}
