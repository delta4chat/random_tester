//! the Chi Square test.

use super::*;

/// Computes the Chi Square probability of a random dataset this extreme.
#[derive(Debug, Copy, Clone)]
pub struct ChiSquareCalculation {
    buckets: [usize; 256],
    total_buckets: usize,
}

impl Default for ChiSquareCalculation {
    fn default() -> Self {
        Self::INIT
    }
}

impl ChiSquareCalculation {
    /// the blanket state (initial value) of [ChiSquareCalculation].
    pub const INIT: Self =
        Self {
            buckets: [0; 256],
            total_buckets: 0,
        };

    /// creates new blanket state for chi-square calculation.
    ///
    /// this just copy from [ChiSquareCalculation::INIT].
    pub const fn new() -> Self {
        Self::INIT
    }

    /// apply byte stream to chi-square state.
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

    /// get finalize chi-square result of current byte stream.
    pub const fn finalize(&self) -> Dec {
        probability_chi_sq(chi_statistic(&self.buckets, self.total_buckets))
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

impl EntropyTest for ChiSquareCalculation {
    fn update(&mut self, bytes: &[u8]) {
        Self::update(self, bytes);
    }

    fn finalize(&self) -> Dec {
        Self::finalize(self)
    }
}

const MAX_X: Dec = dec!(20.0); // max value of e^x
const NEG_MAX_X: Dec = MAX_X.neg();
const LOG_SQRT_PI: Dec = dec!(0.5723649429247000870717135); // log (sqrt (pi) )
const I_SQRT_PI: Dec = dec!(0.5641895835477562869480795); // 1 / sqrt(pi)
const ZERO: Dec = dec!(0.0);
const HALF_ONE: Dec = dec!(0.5);
const ONE: Dec = dec!(1.0);
const TWO: Dec = dec!(2.0);
const THREE: Dec = dec!(3.0);

/// Compute X^2 statistic
pub const fn chi_statistic(buckets: &[usize; 256], total_buckets: usize) -> Dec {
    let total = Dec::from_usize(total_buckets);
    let mut chi_sq = ZERO;
    let exp = total.div(dec!(256.0));

    let mut i = 0;
    let mut b;
    let mut a;
    while i < 256 {
        b = buckets[i];

        a = Dec::from_usize(b).sub(exp);
        chi_sq = chi_sq.add(a.mul(a).div(exp));

        i += 1;
    }

    chi_sq
}

/// Adapted from <https://www.fourmilab.ch/random/>
/// which is an adaption from
///
/// ALGORITHM Compute probability of chi square value.
///     Adapted from:
///         Hill, I.D. and Pike, M.C. Algorithm 299
///         Collected Algorithms for the CACM 1967 p. 243
///     Updated for rounding errors based on remark in
///         ACM TOMS June 1985, p. 185
///
pub const fn probability_chi_sq(chi_sq: Dec) -> Dec {
    let mut x = chi_sq;
    if x.le(&ZERO) {
        return ONE;
    }

    let a = HALF_ONE.mul(x);
    let mut s = TWO.mul(poz(x.sqrt().neg()));

    x = dec!(127.0);
    let mut z = HALF_ONE;

    if a.gt(&MAX_X) {
        let mut e = LOG_SQRT_PI;
        let c = a.ln();
        while z.le(&x) {
            e = e.add(z.ln());
            s = s.add(ex(c.mul(z).sub(a).sub(e)));
            z = z.add(ONE);
        }
        s
    } else {
        let mut e = I_SQRT_PI.div(a.sqrt());
        let mut c = ZERO;
        while z.le(&x) {
            e = e.mul(a.div(z));
            c = c.add(e);
            z = z.add(ONE);
        }
        let y = ex(a.neg());
        c.mul(y).add(s)
    }
}

/// exp
pub const fn ex(x: Dec) -> Dec {
    if x.lt(&NEG_MAX_X) {
        ZERO
    } else {
        x.exp()
    }
}

/// VAR normal z value
pub const fn poz(z: Dec) -> Dec {
    let w;
    let x;
    let mut y;

    if z.eq(&ZERO) {
        x = ZERO;
    } else {
        y = HALF_ONE.mul(z.abs());
        if y.ge(&THREE) {
            x = ONE;
        } else if y.lt(&ONE) {
            w = y.mul(y);
            x = dec!(0.000124818987)
                .mul(w).sub(dec!(0.001075204047))
                .mul(w).add(dec!(0.005198775019))
                .mul(w).sub(dec!(0.019198292004))
                .mul(w).add(dec!(0.059054035642))
                .mul(w).sub(dec!(0.151968751364))
                .mul(w).add(dec!(0.319152932694))
                .mul(w).sub(dec!(0.531923007300))
                .mul(w).add(dec!(0.797884560593))
                .mul(y)
                .mul(TWO);
        } else {
            y = y.sub(TWO);

            x = dec!(-0.000045255659)
                .mul(y).add(dec!(0.000152529290))
                .mul(y).sub(dec!(0.000019538132))
                .mul(y).sub(dec!(0.000676904986))
                .mul(y).add(dec!(0.001390604284))
                .mul(y).sub(dec!(0.000794620820))
                .mul(y).sub(dec!(0.002034254874))
                .mul(y).add(dec!(0.006549791214))
                .mul(y).sub(dec!(0.010557625006))
                .mul(y).add(dec!(0.011630447319))
                .mul(y).sub(dec!(0.009279453341))
                .mul(y).add(dec!(0.005353579108))
                .mul(y).sub(dec!(0.002141268741))
                .mul(y).add(dec!(0.000535310849))
                .mul(y).add(dec!(0.999936657524));
        }
    }
    if z.gt(&ZERO) {
        x.add(ONE).mul(HALF_ONE)
    } else {
        ONE.sub(x).mul(HALF_ONE)
    }
}
