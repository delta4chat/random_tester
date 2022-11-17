use super::*;

/// Computes the Shannon Entropy test
#[derive(Debug, Clone, Copy)]
pub struct ShannonCalculation {
    buckets: [usize; 256],
    total_buckets: usize
}

impl Default for ShannonCalculation {
    fn default() -> Self {
        Self {
            buckets: [0; 256],
            total_buckets: 0,
        }
    }
}

impl EntropyTester for ShannonCalculation {
    fn update<B: AsRef<[u8]>>(&mut self, stream: B) {
        for b in stream.as_ref() {
            let i = *b as usize;
            self.buckets[i] += 1;
            self.total_buckets += 1;
        }
    }

    fn finalize(&mut self) -> f64 {
        let length = self.total_buckets as f64;
        let mut entropy = 0.0;
        for b in &self.buckets {
            let probability = (*b as f64) / length;
            if probability > 0.0 {
                entropy += probability * (1.0/probability).log2();
            }
        }
        entropy
    }
}