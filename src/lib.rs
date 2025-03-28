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

pub use fastnum2::D256 as Dec;
pub use fastnum2::dec256 as dec;

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

/* helper functions */
/// define expected value is `correct`, then calculate error ratio of `actual`.
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
    fn test<B: AsRef<[u8]>>(bytes: B) -> Dec {
        let mut this = Self::default();
        EntropyTestExt::update(&mut this, bytes);
        EntropyTestExt::finalize(&this)
    }
}

impl<T: EntropyTest + Default> EntropyTestExt for T {
    fn update<B: AsRef<[u8]>>(&mut self, bytes: B) -> &mut Self {
        let bytes = bytes.as_ref();
        EntropyTest::update(self, bytes);
        self
    }

    fn finalize(&self) -> Dec {
        EntropyTest::finalize(self)
    }
}

#[allow(deprecated)]
impl<T: EntropyTester + Clone> EntropyTest for T {
    fn update(&mut self, bytes: &[u8]) {
        EntropyTester::update(self, bytes)
    }

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
    fn update(&mut self, stream: &[u8]) {
        EntropyTester::update(self, stream)
    }

    fn finalize(&mut self) -> f64 {
        EntropyTester::finalize(self)
    }
}

