use std::convert::TryFrom;
use std::iter;

/// Simulates an infinite array of memory.
/// Uninitialized memory addresses contain the value 0.
pub struct Memory(Vec<i64>);

impl Memory {
    pub fn new(initial_memory: Vec<i64>) -> Self {
        Self(initial_memory)
    }

    /// Set the memory at `address` to `value`.
    pub fn set(&mut self, address: i64, value: i64) {
        let addr = Self::addr(address);

        // Ensure address is in-bounds, extending with 0s as needed.
        let len = self.0.len();
        if addr >= len {
            let new_len = addr as usize + 1;
            self.0.extend(iter::repeat(0).take(new_len - len));
        }

        self.0[addr] = value;
    }

    /// Get the value at `address`.
    pub fn get(&self, address: i64) -> i64 {
        let addr = Self::addr(address);

        if addr < self.0.len() {
            self.0[addr]
        } else {
            0
        }
    }

    /// Perform checked conversion from i64 to usize. Panics if negative or overflow.
    fn addr(address: i64) -> usize {
        usize::try_from(address).unwrap()
    }

    /// Helper function for unit tests.
    pub fn _into_vec(self) -> Vec<i64> {
        self.0
    }
}
