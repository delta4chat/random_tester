//! entest

use crate::*;

/// EntestResult is contains results from all test methods.
#[derive(Copy, Clone)]
#[non_exhaustive]
pub struct EntestResult {
    samples: usize, // samples of input (length of bytes)
    chi: Dec,
    chi_prob: Dec,
    mc: Dec,
    mean: Dec,
    sc: Dec,
    shannon: Dec,
}

impl core::fmt::Debug for EntestResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EntestResult")
         .field("samples", &(self.samples))
         .field("chi", &(self.chi.to_string()))
         .field("chi_prob", &(self.chi_prob.to_string()))

         .field("mc", &(self.mc.to_string()))
         .field("mean", &(self.mean.to_string()))
         .field("sc", &(self.sc.to_string()))
         .field("shannon", &(self.shannon.to_string()))
         .finish()
    }
}

impl core::fmt::Display for EntestResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Write;
        let mut out = String::new();

        macro_rules! r {
            ($n:ident) => {
                {
                    write!(out, "{}", $n)?;
                    let prefix = out.split_once('.').map(|(int, _)| { int.len() + 1 }).unwrap_or(0);
                    while
                        out.contains('.')
                        &&
                        (
                            (out.len() - prefix) > 20
                            ||
                            out.ends_with('0')
                            ||
                            out.ends_with('.')
                        )
                    {
                        out.pop();
                    }
                    f.write_str(&out)?;
                    out.clear();
                }
            }
        }

        let samples = self.samples();

        let chi = self.chi();
        let chi_prob = self.chi_prob.mul(dec!(100.0));

        let mc = self.mc;
        let mc_error = error_ratio(Dec::PI, mc).mul(dec!(100.0));

        let mean = self.mean();
        let sc = self.sc();
        let shannon = self.shannon();

        f.write_str("
Entropy = ")?;
        r!(shannon);
        write!(f, " bits per byte.

Optimum compression would reduce the size of this {samples} byte file by [TODO] percent.

Chi square distribution for {samples} samples is ")?;
        r!(chi);
        write!(f, ",
and randomly would exceed this value {chi_prob:.2} percent of the times.

Arithmetic mean value of data bytes is ")?;
        r!(mean);
        f.write_str(" (127.5 = random).

Monte Carlo value for Pi is ")?;
        r!(mc);
        write!(f, " (error {mc_error:.2} percent).

(TODO scc is not accurate)
Serial correlation coefficient is ")?;
        r!(sc);
        f.write_str(" (totally uncorrelated = 0.0).
")
    }
}

impl EntestResult {
    /// total samples of input. that is the length of bytes.
    pub const fn samples(&self) -> usize {
        self.samples
    }

    /// result of chi-square calculation.
    pub const fn chi<'a>(&'a self) -> &'a Dec {
        &self.chi
    }

    /// result of `probability_chi_sq(self.chi)`
    pub const fn chi_prob<'a>(&'a self) -> &'a Dec {
        &self.chi_prob
    }

    /// result of monte-carlo calculation.
    pub const fn mc<'a>(&'a self) -> &'a Dec {
        &self.mc
    }

    /// result of mean calculation.
    pub const fn mean<'a>(&'a self) -> &'a Dec {
        &self.mean
    }

    /// result of serial-correlation-coefficient calculation.
    pub const fn sc<'a>(&'a self) -> &'a Dec {
        &self.sc
    }

    /// result of shannon calculation.
    pub const fn shannon<'a>(&'a self) -> &'a Dec {
        &self.shannon
    }
}

/// Entest can be used for testing random data between all test methods.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct Entest {
    /// state of chi-square calculation.
    pub chi: ChiSquareCalculation,

    /// state of monte-carlo calculation.
    pub mc: MonteCarloCalculation,

    /// state of mean calculation.
    pub mean: MeanCalculation,

    /// state of serial-correlation-coefficient calculation.
    pub sc: SerialCorrelationCoefficientCalculation,

    /// state of shannon calculation.
    pub shannon: ShannonCalculation,
}

impl Default for Entest {
    fn default() -> Self {
        Self::INIT
    }
}

impl Entest {
    /// the blanket state (initial value) of [Entest].
    pub const INIT: Self =
        Self {
            chi: ChiSquareCalculation::INIT,
            mc: MonteCarloCalculation::INIT,
            mean: MeanCalculation::INIT,
            sc: SerialCorrelationCoefficientCalculation::INIT,
            shannon: ShannonCalculation::INIT,
        };

    /// create new blanket state for [Entest].
    ///
    /// this just copy from [Entest::INIT].
    pub const fn new() -> Self {
        Self::INIT
    }

    /// update all test state inside the Entest.
    pub const fn update(&mut self, bytes: &[u8]) -> &mut Self {
        self.chi.update(bytes);
        self.mc.update(bytes);
        self.mean.update(bytes);
        self.sc.update(bytes);
        self.shannon.update(bytes);

        self
    }

    /// get results from all test methods.
    pub const fn finalize(&self) -> EntestResult {
        let (chi, chi_prob) = self.chi.finalize_probability();
        EntestResult {
            samples: self.chi.samples(),
            chi, chi_prob,
            mc: self.mc.finalize(),
            mean: self.mean.finalize(),
            sc: self.sc.finalize(),
            shannon: self.shannon.finalize(),
        }
    }
}
