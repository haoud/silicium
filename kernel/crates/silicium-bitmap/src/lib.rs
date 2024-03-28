#![cfg_attr(not(test), no_std)]
#![feature(const_mut_refs)]

/// A bitmap that contains N * `core::mem::size_of::<usize>()` bits.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bitmap<const N: usize> {
    data: [usize; N],
}

impl<const N: usize> Bitmap<N> {
    /// The number of bits in a word. A word is a `usize`, sp its size
    /// depends on the architecture. This allows the bitmap to be
    /// optimized for the target architecture.
    pub const BITS_PER_WORD: usize = core::mem::size_of::<usize>() * 8;

    /// Create a new bitmap with all bits set to 0
    #[must_use]
    pub const fn zeroes() -> Self {
        Self { data: [0; N] }
    }

    /// Create a new bitmap with all bits set to 1
    #[must_use]
    pub const fn ones() -> Self {
        Self {
            data: [usize::MAX; N],
        }
    }

    /// Return the capacity of the bitmap, in bytes
    #[must_use]
    pub const fn capacity(&self) -> usize {
        N * core::mem::size_of::<usize>()
    }

    /// Return the number of bits in the bitmap
    #[must_use]
    pub const fn count(&self) -> usize {
        N * Self::BITS_PER_WORD
    }

    /// Set the bit at the given index to 1
    ///
    /// # Panics
    /// Panics if the index is out of bounds
    pub const fn set(&mut self, index: usize) {
        assert!(index < N * Self::BITS_PER_WORD);
        let index = index / Self::BITS_PER_WORD;
        let bit = index % Self::BITS_PER_WORD;
        self.data[index] |= 1 << bit;
    }

    /// Set the bit at the given index to 0
    ///
    /// # Panics
    /// Panics if the index is out of bounds
    pub const fn clear(&mut self, index: usize) {
        assert!(index < N * Self::BITS_PER_WORD);
        let index = index / Self::BITS_PER_WORD;
        let bit = index % Self::BITS_PER_WORD;
        self.data[index] &= !(1 << bit);
    }

    /// Toggle the bit at the given index. If it is 1, it will be set
    /// to 0, and vice versa.
    ///
    /// # Panics
    /// Panics if the index is out of bounds
    pub const fn toggle(&mut self, index: usize) {
        assert!(index < N * Self::BITS_PER_WORD);
        let index = index / Self::BITS_PER_WORD;
        let bit = index % Self::BITS_PER_WORD;
        self.data[index] ^= 1 << bit;
    }

    /// Get the value of the bit at the given index
    ///
    /// # Panics
    /// Panics if the index is out of bounds
    #[must_use]
    pub const fn get(&self, index: usize) -> bool {
        assert!(index < N * Self::BITS_PER_WORD);
        let index = index / Self::BITS_PER_WORD;
        let bit = index % Self::BITS_PER_WORD;
        (self.data[index] & (1 << bit)) != 0
    }

    /// Find the index of the first bit that is not set to 1, set it
    /// to 1, and return it. If all bits are set to 1, return None.
    #[must_use]
    pub const fn get_first_zero(&mut self) -> Option<usize> {
        let mut index = 0;
        while index < N {
            let word = self.data[index];
            if word != usize::MAX {
                let bit = word.trailing_zeros() as usize;
                self.data[index] |= 1 << bit;
                return Some(index * Self::BITS_PER_WORD + bit);
            }
            index += 1;
        }
        None
    }

    /// Find the index of the first bit that is set to 1, set it to 0,
    /// and return it. If all bits are set to 0, return None.
    #[must_use]
    pub const fn get_first_one(&mut self) -> Option<usize> {
        let mut index = 0;
        while index < N {
            let word = self.data[index];
            if word != 0 {
                let bit = word.trailing_zeros() as usize;
                self.data[index] &= !(1 << bit);
                return Some(index * Self::BITS_PER_WORD + bit);
            }
            index += 1;
        }
        None
    }

    /// Find the index of the first bit that is not set to 1, set it
    /// to 1, and return it. If all bits are set to 1, return None.
    ///
    /// This method is the same as `get_first_zero`, but it will start
    /// from the given index and wrap around to the beginning of the
    /// bitmap if necessary.
    ///
    /// If the given index is out of bounds, it will be wrapped around
    /// to the beginning of the bitmap using the modulo operator.
    ///
    /// This is useful for implementing ID pools, where we want to
    /// find the next available ID after a given one.
    #[must_use]
    pub const fn get_next_zero(&mut self, start: usize) -> Option<usize> {
        // Search from the start bit to the end of the bitmap
        // If we reach the end, wrap around to the beginning
        let start_index = (start / Self::BITS_PER_WORD) % N;
        let start_bit = start % Self::BITS_PER_WORD;
       
        // Start searching from the start index
        let mut index = start_index;
        while index < N {
            let word = self.data[index];
            if word != usize::MAX {
                let mut bit = start_bit;
                while bit < Self::BITS_PER_WORD {
                    if (word & (1 << bit)) == 0 {
                        self.data[index] |= 1 << bit;
                        return Some(index * Self::BITS_PER_WORD + bit);
                    }
                    bit += 1;
                }
            }
            index += 1;
        }

        // If we reach the end of the bitmap, wrap around to the beginning
        index = 0;
        while index < start_index {
            let word = self.data[index];
            if word != usize::MAX {
                let mut bit = 0;
                while bit < Self::BITS_PER_WORD {
                    if (word & (1 << bit)) == 0 {
                        self.data[index] |= 1 << bit;
                        return Some(index * Self::BITS_PER_WORD + bit);
                    }
                    bit += 1;
                }
            }
            index += 1;
        }

        None
    }
}
