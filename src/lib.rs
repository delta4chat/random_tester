#![deny(
    warnings,
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts
)]
//! A program that applies tests to byte sequences stored in files or streams

mod chisqr;
mod mc;
mod mean;
mod sc;
mod shannon;

pub use chisqr::*;
pub use mc::*;
pub use mean::*;
pub use sc::*;
pub use shannon::*;

/// An entropy test
pub trait EntropyTester: Default {
    /// Process a sequence of bytes from a stream
    fn update<B: AsRef<[u8]>>(&mut self, stream: B);
    /// Compute the final result
    fn finalize(&mut self) -> f64;
}

/// An entropy test
pub trait DynEntropyTester: Default {
    /// Process a sequence of bytes from a stream
    fn update(&mut self, stream: &[u8]);
    /// Compute the final result
    fn finalize(&mut self) -> f64;
}

impl<R: EntropyTester> DynEntropyTester for R {
    fn update(&mut self, stream: &[u8]) {
        R::update(self, stream)
    }

    fn finalize(&mut self) -> f64 {
        R::finalize(self)
    }
}