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
        chi_statistic(&self.buckets, self.total_buckets)
    }

    /// returns `self.finalize()` and `probability_chi_sq(self.finalize())`
    pub const fn finalize_probability(&self) -> (Dec, Dec) {
        let f = self.finalize();

        //(f, pochisq(f, 255))
        //(f, pochisq(f, self.samples()))
        (f, probability_chi_sq(f, 127))
    }

    /// get the samples of current state.
    pub const fn samples(&self) -> usize {
        self.total_buckets
    }

    /// oneshot test function for small data.
    ///
    /// this is equivalent to `Self::new().update(data).finalize()`.
    pub const fn test(data: &[u8]) -> (Dec, Dec) {
        let mut this = Self::INIT;
        this.update(data);
        this.finalize_probability()
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

/// max value of e^x (`20.0`)
pub const MAX_X: Dec = dec!(20.0);

/// negative value of [MAX_X].
pub const NEG_MAX_X: Dec = MAX_X.neg();

/// python: `math.log(math.sqrt(math.pi))`
///
/// (approx `0.5723649429247000870717135`).
//#[allow(long_running_const_eval)]
pub const LOG_SQRT_PI: Dec = dec!(0.57236494292470008707171367567652935582364740645765578575681153573606888494239); // Dec::PI.sqrt().ln();

/// python: `1/math.sqrt(math.pi)`
///
/// (approx `0.5641895835477562869480795`)
//#[allow(long_running_const_eval)]
pub const I_SQRT_PI: Dec = dec!(0.56418958354775628694807945156077258584405062932899885684408572171064246844150); // Dec::ONE.div(Dec::PI.sqrt());

/// zero (`0.0`)
pub const ZERO: Dec = dec!(0.0);

/// half of one (`0.5`)
pub const HALF_ONE: Dec = dec!(0.5);

/// one (`1.0`)
pub const ONE: Dec = dec!(1.0);

/// two (`2.0`)
pub const TWO: Dec = dec!(2.0);

/// max value of z (`6.0`)
pub const MAX_Z: Dec = dec!(6.0);

/// half of [MAX_Z].
pub const HALF_MAX_Z: Dec = MAX_Z.div(TWO);

/// Compute X^2 statistic
pub const fn chi_statistic(buckets: &[usize; 256], total_buckets: usize) -> Dec {
    if total_buckets == 0 {
        return Dec::NAN;
    }

    let total = Dec::from_usize(total_buckets);
    let mut chi_sq = ZERO;
    let exp = total.div(dec!(256.0));

    let mut i = 0;
    let mut b;
    let mut a;

    while i < 256 {
        b = Dec::from_usize(buckets[i]);

        a = b.sub(exp);
        chi_sq = chi_sq.add(a.mul(a).div(exp));

        i += 1;
    }

    chi_sq
}
/**
 * FUNCTION pochisq: probability of chi sqaure value

 * ALGORITHM Compute probability of chi square value.
        Adapted from:
                Hill, I. D. and Pike, M. C.  Algorithm 299
                Collected Algorithms for the CACM 1967 p. 243

        [FIXME]
        Updated for rounding errors based on remark in
                ACM TOMS June 1985, page 185
            (??? is it causes bugs? because fastnum has no rounding errors)
*/
// FIXME returns incorrect result currently
pub(crate) const fn _pochisq(chi_sq: Dec, df: usize) -> Dec {
    if chi_sq.le(&ZERO) || df < 1 {
        return ONE;
    }

    let mut x = chi_sq;

    let mut e;
    let mut c;
    let mut z;

    let a = HALF_ONE.mul(x);
    let even = (df & 1) == 0;

    let y = if df > 1 { ex(a.neg()) } else { ZERO };

    let mut s = if even { y } else { TWO.mul(poz(x.sqrt().neg())) };
    if df > 2 {
        x = HALF_ONE.mul(Dec::from_usize(df - 1));
        z = if even { ONE } else { HALF_ONE };
        if a.gt(&MAX_X) {
            e = if even { ZERO } else { LOG_SQRT_PI };
            c = a.ln();
            while z.le(&x) {
                e = e.add(z.ln());
                s = s.add(c.mul(z).sub(a).sub(e));
                z = z.add(ONE);
            }
            s
        } else {
            e = if even { ONE } else { I_SQRT_PI.div(a.sqrt()) };
            c = ZERO;
            while z.le(&x) {
                e = e.mul(a.div(z));
                c = c.add(e);
                z = z.add(ONE);
            }
            c.mul(y).add(s)
        }
    } else {
        s
    }
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
/// default df = 127
pub const fn probability_chi_sq(chi_sq: Dec, df: usize) -> Dec {
    if chi_sq.is_nan() {
        return chi_sq;
    }
    let mut x = chi_sq;
    if x.le(&ZERO) {
        return ONE;
    }

    let a = HALF_ONE.mul(x);
    let mut s = TWO.mul(poz(x.sqrt().neg()));

    x = Dec::from_usize(df);
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
        if y.ge(&HALF_MAX_Z) {
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
