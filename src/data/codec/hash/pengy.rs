// devela::data::codec::hash::pengy
//
//! Based on pengyhash v0.2 LICENSED as BSD-2.
//

use crate::{ConstDefault, Hasher};

/// A fast 64-bit non-cryptographic hash algorithm.
///
#[doc = crate::doc_!(vendor: "pengyhash")]
#[derive(Debug)]
pub struct HasherPengy {
    state: [u64; 4],
    seed: u64,
}

impl ConstDefault for HasherPengy {
    const DEFAULT: Self = Self { state: [0; 4], seed: 0 };
}
impl Default for HasherPengy {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl HasherPengy {
    /// Returns a default pengy hasher.
    pub const fn new() -> Self {
        Self::DEFAULT
    }

    /// Returns a new pengy hasher with the given `seed`.
    pub const fn with_seed(seed: u64) -> Self {
        Self { state: [0; 4], seed }
    }

    /// Checks whether the current state is uninitialized (or unchanged from default).
    pub const fn is_empty(&self) -> bool {
        let [a, b, c, d] = self.state;
        a + b + c + d == 0
    }

    /// Resets the internal state to its initial default values without changing the seed.
    pub const fn reset(&mut self) {
        self.state = [0; 4];
    }

    /// Resets both the state and seed for full reinitialization.
    pub const fn clear(&mut self) {
        self.state = [0; 4];
        self.seed = 0;
    }

    /// Processes the given data slice, updating the internal state.
    pub const fn process(&mut self, data: &[u8]) {
        let mut b: [u64; 4] = [0; 4];
        let mut s: [u64; 4] = [0, 0, 0, data.len() as u64];
        let mut remaining = data;

        // Process 32-byte chunks into 64-bit words
        while remaining.len() >= 32 {
            let mut i = 0;
            while i < 4 {
                let (_, rest) = remaining.split_at(i * 8);
                let (chunk, _) = rest.split_at(8);
                b[i] = u64::from_le_bytes(Self::make_byte_array(chunk));

                i += 1;
            }
            let (_left, right) = remaining.split_at(32);
            remaining = right;

            s[0] = s[0].wrapping_add(s[1]).wrapping_add(b[3]);
            s[1] = s[0].wrapping_add(s[1].wrapping_shl(14) | s[1].wrapping_shr(50));
            s[2] = s[2].wrapping_add(s[3]).wrapping_add(b[2]);
            s[3] = s[2].wrapping_add(s[3].wrapping_shl(23) | s[3].wrapping_shr(41));
            s[0] = s[0].wrapping_add(s[3]).wrapping_add(b[1]);
            s[3] = s[0] ^ (s[3].wrapping_shl(16) | s[3].wrapping_shr(48));
            s[2] = s[2].wrapping_add(s[1]).wrapping_add(b[0]);
            s[1] = s[2] ^ (s[1].wrapping_shl(40) | s[1].wrapping_shr(24));
        }

        let mut tmp = [0u8; 32];
        let mut i = 0;
        while i < remaining.len() {
            if i < tmp.len() {
                tmp[i] = remaining[i];
            }
            i += 1;
        }

        let mut i = 0;
        while i < 4 {
            let (_, rest) = tmp.split_at(i * 8);
            let (chunk, _) = rest.split_at(8);
            b[i] = u64::from_le_bytes(Self::make_byte_array(chunk));
            i += 1;
        }

        // Perform final mixing of state with seed and remaining data
        let mut _i = 0;
        while _i < 6 {
            s[0] = s[0].wrapping_add(s[1]).wrapping_add(b[3]);
            s[1] = s[0]
                .wrapping_add(s[1].wrapping_shl(14) | s[1].wrapping_shr(50))
                .wrapping_add(self.seed);
            s[2] = s[2].wrapping_add(s[3]).wrapping_add(b[2]);
            s[3] = s[2].wrapping_add(s[3].wrapping_shl(23) | s[3].wrapping_shr(41));
            s[0] = s[0].wrapping_add(s[3]).wrapping_add(b[1]);
            s[3] = s[0] ^ (s[3].wrapping_shl(16) | s[3].wrapping_shr(48));
            s[2] = s[2].wrapping_add(s[1]).wrapping_add(b[0]);
            s[1] = s[2] ^ (s[1].wrapping_shl(40) | s[1].wrapping_shr(24));
            _i += 1;
        }

        self.state[0] = s[0];
        self.state[1] = s[1];
        self.state[2] = s[2];
        self.state[3] = s[3];
    }

    const fn make_byte_array(chunk: &[u8]) -> [u8; 8] {
        let mut byte_array = [0; 8];
        let mut j = 0;
        while j < chunk.len() {
            byte_array[j] = chunk[j];
            j += 1;
        }
        byte_array
    }

    /// Produces the final hash value based on the internal state.
    pub const fn digest(&self) -> u64 {
        self.state[0]
            .wrapping_add(self.state[1])
            .wrapping_add(self.state[2])
            .wrapping_add(self.state[3])
    }
}

impl Hasher for HasherPengy {
    fn write(&mut self, data: &[u8]) {
        self.process(data);
    }
    fn finish(&self) -> u64 {
        self.digest()
    }
}
