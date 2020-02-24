use std::cmp::max;
use std::ops::Add;

use packed_simd::u8x64;

pub struct SIMDBuffer {
    pub buffer: Vec<u8>,
}

impl SIMDBuffer {
    pub fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }
}

// Use 64 as the step size since avx2 chips can handle 256 bits or 64 u8s.
const STEP_SIZE: usize = 64;

impl Add for SIMDBuffer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let max = max(self.buffer.len(), rhs.buffer.len());
        let remainder = max % STEP_SIZE;

        let mut result = Vec::new();
        result.resize(max, 0);
        for index in (0..(max - STEP_SIZE)).step_by(STEP_SIZE) {
            let a = u8x64::from_slice_unaligned(&self.buffer[index..]);
            let b = u8x64::from_slice_unaligned(&rhs.buffer[index..]);
            let c = a + b;
            c.write_to_slice_unaligned(&mut result[index..(index + STEP_SIZE)]);
        }

        // Handle the remaining bits in a piecewise fashion
        for index in ((max - STEP_SIZE) - remainder)..max {
            result[index] = self.buffer[index] + rhs.buffer[index]
        }

        SIMDBuffer { buffer: result }
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::SIMDBuffer;

    #[test]
    fn test_add() {
        let a = SIMDBuffer {
            buffer: vec![0; 1024],
        };
        let b = SIMDBuffer {
            buffer: vec![1; 1024],
        };

        let c = a + b;
        for val in c.buffer {
            assert_eq!(1, val);
        }
    }

    #[test]
    fn test_reminder_add() {
        let a = SIMDBuffer {
            buffer: vec![0; 6051],
        };
        let b = SIMDBuffer {
            buffer: vec![1; 6051],
        };

        let c = a + b;
        for val in c.buffer {
            assert_eq!(1, val);
        }
    }
}
