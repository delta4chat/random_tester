use super::*;

/// Computes the Chi Square probability of a random dataset this extreme.
#[derive(Debug, Clone, Copy)]
pub struct ChiSquareCalculation {
    buckets: [usize; 256],
    total_buckets: usize,
}

impl Default for ChiSquareCalculation {
    fn default() -> Self {
        Self {
            buckets: [0; 256],
            total_buckets: 0,
        }
    }
}

impl EntropyTester for ChiSquareCalculation {
    fn update<B: AsRef<[u8]>>(&mut self, stream: B) {
        for b in stream.as_ref() {
            let i = *b as usize;
            self.buckets[i] += 1;
            self.total_buckets += 1;
        }
    }

    fn finalize(&mut self) -> f64 {
        probability_chi_sq(self.statistic())
    }
}

impl ChiSquareCalculation {
    /// Compute X^2 statistic
    pub(crate) fn statistic(&self) -> f64 {
        let total = self.total_buckets as f64;
        let mut chi_sq = 0.0;
        let exp = total / 256.0;

        for b in &self.buckets {
            let a = (*b as f64) - exp;
            chi_sq += (a * a) / exp;
        }
        chi_sq
    }
}

/// Adapted from https://www.fourmilab.ch/random/
/// which is an adaption from
///
/// ALGORITHM Compute probability of chi square value.
///     Adapted from:
///         Hill, I.D. and Pike, M.C. Algorithm 299
///         Collected Algorithms for the CACM 1967 p. 243
///     Updated for rounding errors based on remark in
///         ACM TOMS June 1985, p. 185
///
pub(crate) fn probability_chi_sq(chi_sq: f64) -> f64 {
    const MAX_X: f64 = 20.0; // max value of e^x
    const LOG_SQRT_PI: f64 = 0.5723649429247000870717135; // log (sqrt (pi) )
    const I_SQRT_PI: f64 = 0.5641895835477562869480795; // 1 / sqrt(pi)

    let mut x = chi_sq;
    if x <= 0.0 {
        return 1.0;
    }

    let a = 0.5 * x;
    let mut s = 2.0 * poz(-x.sqrt());

    x = 127.0;
    let mut z = 0.5;

    if a > MAX_X {
        let mut e = LOG_SQRT_PI;
        let c = a.ln();
        while z <= x {
            e += z.ln();
            s += ex(c * z - a - e);
            z += 1.0;
        }
        s
    } else {
        let mut e = I_SQRT_PI / a.sqrt();
        let mut c = 0.0;
        while z <= x {
            e *= a / z;
            c += e;
            z += 1.0;
        }
        let y = ex(-a);
        c * y + s
    }
}

pub(crate) fn ex(x: f64) -> f64 {
    const MAX_X: f64 = 20.0;
    if x < -MAX_X {
        0.0
    } else {
        x.exp()
    }
}

/// VAR normal z value
pub(crate) fn poz(z: f64) -> f64 {
    let w;
    let x;
    let mut y;

    if z == 0.0 {
        x = 0.0;
    } else {
        y = 0.5 * z.abs();
        if y >= 3.0 {
            x = 1.0;
        } else if y < 1.0 {
            w = y * y;
            x = ((((((((0.000124818987 * w - 0.001075204047) * w + 0.005198775019) * w
                - 0.019198292004)
                * w
                + 0.059054035642)
                * w
                - 0.151968751364)
                * w
                + 0.319152932694)
                * w
                - 0.531923007300)
                * w
                + 0.797884560593)
                * y
                * 2.0;
        } else {
            y -= 2.0;
            x = (((((((((((((-0.000045255659 * y + 0.000152529290) * y - 0.000019538132)
                * y
                - 0.000676904986)
                * y
                + 0.001390604284)
                * y
                - 0.000794620820)
                * y
                - 0.002034254874)
                * y
                + 0.006549791214)
                * y
                - 0.010557625006)
                * y
                + 0.011630447319)
                * y
                - 0.009279453341)
                * y
                + 0.005353579108)
                * y
                - 0.002141268741)
                * y
                + 0.000535310849)
                * y
                + 0.999936657524;
        }
    }
    if z > 0.0 {
        (x + 1.0) * 0.5
    } else {
        (1.0 - x) * 0.5
    }
}
