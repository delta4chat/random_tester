use super::*;

/// Computes the Mean Entropy test
#[derive(Debug, Clone, Copy)]
pub struct MeanCalculation {
    buckets: [usize; 256],
    total_buckets: usize,
}

impl Default for MeanCalculation {
    fn default() -> Self {
        Self {
            buckets: [0; 256],
            total_buckets: 0,
        }
    }
}

impl EntropyTester for MeanCalculation {
    fn update<B: AsRef<[u8]>>(&mut self, stream: B) {
        for b in stream.as_ref() {
            let i = *b as usize;
            self.buckets[i] += 1;
            self.total_buckets += 1;
        }
    }

    fn finalize(&mut self) -> f64 {
        let mut sum = 0.0;
        for (i, b) in self.buckets.iter().enumerate() {
            sum += (i as f64) * (*b as f64);
        }
        sum / (self.total_buckets as f64)
    }
}
