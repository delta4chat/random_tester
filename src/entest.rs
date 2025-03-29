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
        //self.mean.update(bytes);
        //self.shannon.update(bytes);

        self.mc.update(bytes);
        self.sc.update(bytes);

        self
    }

    /// get results from all test methods.
    pub const fn finalize(&mut self) -> EntestResult {
        unwrap!(copy_from_slice(&mut self.mean.buckets, &self.chi.buckets));
        self.mean.total_buckets = self.chi.total_buckets;

        unwrap!(copy_from_slice(&mut self.shannon.buckets, &self.chi.buckets));
        self.shannon.total_buckets = self.chi.total_buckets;

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

    /// this is equivalent to `Self::new().update(data).finalize()`.
    pub const fn test(bytes: &[u8]) -> EntestResult {
        let mut this = Self::INIT;
        this.update(bytes);
        this.finalize()
    }
}

#[cfg(feature="test-rng")]
impl Entest {
    /// test the provided RNG with provided buffer.
    pub fn test_rng<R: rand_core::RngCore>(rng: &mut R, size: usize, buf: &mut [u8]) -> EntestResult {
        let mut this = Self::INIT;
        if size == 0 {
            return this.finalize();
        }

        let mut chunk_size = buf.len();
        if chunk_size == 0 {
            panic!("programming error in external caller: provided zero length buffer to test_rng() method!");
        }

        let mut remaining = size;
        while remaining > 0 {
            if remaining < chunk_size {
                chunk_size = remaining;
            }

            rng.fill_bytes(&mut buf[..chunk_size]);
            this.update(&buf[..chunk_size]);
            remaining -= chunk_size;
        }

        this.finalize()
    }

    /// test the provided RNG with dynamic heap memory alloc by Vec.
    pub fn test_rng_heap<R: rand_core::RngCore>(rng: &mut R, size: usize) -> EntestResult {
        if size == 0 {
            let mut this = Self::INIT;
            return this.finalize();
        }

        let mut chunk_size = size / 10;
        if chunk_size < 1024 || chunk_size < size {
            chunk_size = size;
        }

        use alloc::vec;
        let mut buf = vec![0u8; chunk_size];

        Self::test_rng(rng, size, &mut buf)
    }

    /// test the provided RNG with static stack memory alloc by fixed-size array.
    pub fn test_rng_stack<const BUF_SIZE: usize, R: rand_core::RngCore>(rng: &mut R, size: usize) -> EntestResult {
        if size == 0 {
            let mut this = Self::INIT;
            return this.finalize();
        }

        // avoid overflow the stack.
        // limit max buffer size to 256 KiB.
        const BUF_SIZE_MAX: usize = 1024 * 256;

        if BUF_SIZE > BUF_SIZE_MAX {
            Self::test_rng(rng, size, &mut [0u8; BUF_SIZE_MAX])
        } else {
            Self::test_rng(rng, size, &mut [0u8; BUF_SIZE])
        }
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature="test-rng")]
    #[test]
    fn test_rng_stack_must_not_overflow_if_buf_size_too_large() {
        struct DummyRng;
        impl rand_core::RngCore for DummyRng {
            fn next_u32(&mut self) -> u32 {
                12345678
            }
            fn next_u64(&mut self) -> u64 {
                12345678
            }
            fn fill_bytes(&mut self, b: &mut [u8]) {
                if ! b.is_empty() {
                    b[0] = 1;
                }
            }
        }

        let mut rng = DummyRng;
        println!("{:?}", Entest::test_rng_stack::<104857600, DummyRng>(&mut rng, 118435101));
    }
}
