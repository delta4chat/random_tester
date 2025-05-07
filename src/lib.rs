//! entest (entropy test) is a program that applies tests to byte sequences stored in files or streams

#![forbid(unsafe_code)]

#![cfg_attr(all(not(test), not(feature="std")), no_std)]

#![deny(
    warnings,
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts
)]

extern crate alloc;
use alloc::{
    string::{String, ToString},
};

/// copy values from `src` to `dst`.
/// returns Ok with copied bytes if success.
/// or return Err if `dst.len() < src.len()`
#[inline(always)]
pub const fn copy_from_slice<T: Copy>(dst: &mut [T], src: &[T]) -> Result<usize, ()> {
    let src_len = src.len();
    let dst_len = dst.len();
    if dst_len < src_len {
        return Err(());
    }

    let mut i = 0;
    while i < src_len {
        dst[i] = src[i];
        i += 1;
    }
    Ok(i)
}

macro_rules! unwrap {
    ($val:expr) => {
        match $val {
            Ok(val) => val,
            _ => {
                panic!("try to unwrap a Result::Err");
            }
        }
    }
}

const DEC_CTX: fastnum2::decimal::Context = {
    let mut ctx = fastnum2::decimal::Context::default();
    ctx.set_notation(fastnum2::decimal::Notation::FullScale);
    ctx
};

#[cfg(not(feature="lite"))]
pub use fastnum2::D256 as Dec;

#[cfg(feature="lite")]
/// 64-bit Decimal for lite
pub type Dec = fastnum2::decimal::D64;

#[macro_export]
/// macro for create Dec type used by entest
macro_rules! dec {
    ($val:expr) => {
        {
            const N: $crate::Dec = $crate::Dec::parse_str(stringify!($val), $crate::DEC_CTX);

            N
        }
    }
}

// only for `cargo doc --document-private-items -F cli`
#[cfg(all(doc, feature="cli"))]
pub mod cli;

pub mod entest;
pub use entest::{Entest, EntestResult};

pub mod chisqr;
pub use chisqr::ChiSquareCalculation;

pub mod mc;
pub use mc::MonteCarloCalculation;

pub mod mean;
pub use mean::MeanCalculation;

pub mod sc;
pub use sc::SerialCorrelationCoefficientCalculation;

pub mod shannon;
pub use shannon::ShannonCalculation;

#[cfg(test)]
mod tests;

/* helper functions */
/// define expected value is `correct`, then calculate error ratio of `actual`.
#[inline(always)]
pub const fn error_ratio(correct: Dec, actual: Dec) -> Dec {
    actual.sub(correct).abs().div(correct.abs())
}

/// Tests entropy bits of provided byte stream.
pub trait EntropyTest {
    /// provides byte stream for testing it's entropy.
    fn update(&mut self, bytes: &[u8]);

    /// get result of entropy test.
    fn finalize(&self) -> Dec;
}

/// extension of [EntropyTest]. but it is **not** dyn-compatible (object-safety).
pub trait EntropyTestExt: Sized + Default {
    /// provides any types that is implements `AsRef<[u8]>` for testing it's entropy.
    fn update<B: AsRef<[u8]>>(&mut self, bytes: B) -> &mut Self;

    /// get result of entropy test.
    fn finalize(&self) -> Dec;

    /// oneshot test function for small data.
    ///
    /// this is equivalent to `Self::default().update(bytes).finalize()`.
    #[inline(always)]
    fn test<B: AsRef<[u8]>>(bytes: B) -> Dec {
        let mut this = Self::default();
        EntropyTestExt::update(&mut this, bytes);
        EntropyTestExt::finalize(&this)
    }
}

impl<T: EntropyTest + Default> EntropyTestExt for T {
    #[inline(always)]
    fn update<B: AsRef<[u8]>>(&mut self, bytes: B) -> &mut Self {
        let bytes = bytes.as_ref();
        EntropyTest::update(self, bytes);
        self
    }

    #[inline(always)]
    fn finalize(&self) -> Dec {
        EntropyTest::finalize(self)
    }
}

#[allow(deprecated)]
impl<T: EntropyTester + Clone> EntropyTest for T {
    #[inline(always)]
    fn update(&mut self, bytes: &[u8]) {
        EntropyTester::update(self, bytes)
    }

    #[inline(always)]
    fn finalize(&self) -> Dec {
        let mut this = self.clone();
        EntropyTester::finalize(&mut this).into()
    }
}

/// old-style, deprecated, kept only for compatibility.
/// please use [EntropyTest] instead.
#[deprecated(note="please use `EntropyTest` instead.")]
pub trait EntropyTester {
    /// Process a sequence of bytes from a stream
    fn update<B: AsRef<[u8]>>(&mut self, stream: B);
    /// Compute the final result
    fn finalize(&mut self) -> f64;
}

/// old-style, deprecated, kept only for compatibility.
/// please use [EntropyTest] instead.
#[deprecated(note="please use `EntropyTest` instead.")]
pub trait DynEntropyTester {
    /// Process a sequence of bytes from a stream
    fn update(&mut self, stream: &[u8]);
    /// Compute the final result
    fn finalize(&mut self) -> f64;
}

#[allow(deprecated)]
impl<T: EntropyTester> DynEntropyTester for T {
    #[inline(always)]
    fn update(&mut self, stream: &[u8]) {
        EntropyTester::update(self, stream)
    }

    #[inline(always)]
    fn finalize(&mut self) -> f64 {
        EntropyTester::finalize(self)
    }
}

