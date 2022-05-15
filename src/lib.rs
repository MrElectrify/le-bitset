//! A very simple bitset that is guaranteed to be stored
//! in little-endian. If you can't think of a reason you
//! need this, this crate probably isn't for you. Usage
//! is pretty self-explanatory.

#[cfg(test)]
mod test;

const U32_BITS: usize = std::mem::size_of::<u32>() * 8;

/// A little-endian bitset
#[derive(Debug, Clone)]
#[cfg_attr(serde, derive(Deserialize, Serialize))]
pub struct BitSet {
    chunks: Vec<u32>,
    num_bits: usize,
}

impl BitSet {
    /// Calculates the number of chunks to store bits with
    ///
    /// # Arguments
    ///
    /// `num_bits`: The number of bits
    #[inline]
    fn calc_chunks(num_bits: usize) -> usize {
        (num_bits + U32_BITS - 1) / U32_BITS
    }

    /// Calculates the mask of the last chunk's bits
    ///
    /// # Arguments
    ///
    /// `num_bits`: The number of bits
    #[inline]
    fn last_mask(num_bits: usize) -> u32 {
        !(!1 << ((num_bits - 1) % U32_BITS))
    }

    /// Returns the chunks from the bitset
    pub fn chunks(&self) -> &Vec<u32> {
        &self.chunks
    }

    /// Creates a bitset from the chunks
    ///
    /// # Arguments
    ///
    /// `chunks`: The raw little-endian bit chunks
    ///
    /// `num_bits`: The number of bits
    pub fn from_bits(chunks: Vec<u32>, num_bits: usize) -> Self {
        #[cfg(debug_assertions)]
        assert!(num_bits > 0);
        #[cfg(debug_assertions)]
        assert_eq!(Self::calc_chunks(num_bits), chunks.len());
        #[cfg(debug_assertions)]
        assert_eq!(chunks[chunks.len() - 1] & !Self::last_mask(num_bits), 0);
        Self { chunks, num_bits }
    }

    /// Unsets a bit at the index
    ///
    /// # Arguments
    ///
    /// `idx`: The index
    pub fn reset(&mut self, idx: usize) {
        #[cfg(debug_assertions)]
        assert!(idx / U32_BITS < self.chunks.len());
        #[cfg(debug_assertions)]
        assert!(idx < self.num_bits);
        let chunk = &mut self.chunks[idx / U32_BITS];
        let chunk_pos = idx % U32_BITS;
        *chunk = (*chunk & !(1 << chunk_pos)).to_le();
    }

    /// Resets all bits to 0
    pub fn reset_all(&mut self) {
        for chunk in &mut self.chunks {
            *chunk = 0;
        }
    }

    /// Sets a bit at the index
    ///
    /// # Arguments
    ///
    /// `idx`: The index
    pub fn set(&mut self, idx: usize) {
        #[cfg(debug_assertions)]
        assert!(idx / U32_BITS < self.chunks.len());
        #[cfg(debug_assertions)]
        assert!(idx < self.num_bits);
        let chunk = &mut self.chunks[idx / U32_BITS];
        let chunk_pos = idx % U32_BITS;
        *chunk = (*chunk | 1 << chunk_pos).to_le();
    }

    /// Sets all bits to 1
    pub fn set_all(&mut self) {
        for chunk in &mut self.chunks {
            *chunk = u32::MAX;
        }
        let num_chunks = self.chunks.len();
        self.chunks[num_chunks - 1] = Self::last_mask(self.num_bits);
    }

    /// Tests the bit at the index
    ///
    /// # Arguments
    ///
    /// `idx`: The index
    pub fn test(&self, idx: usize) -> bool {
        #[cfg(debug_assertions)]
        assert!(idx / U32_BITS < self.chunks.len());
        #[cfg(debug_assertions)]
        assert!(idx < self.num_bits);
        let chunk = self.chunks[idx / U32_BITS].to_le();
        let chunk_pos = idx % U32_BITS;
        ((chunk >> chunk_pos) & 1) == 1
    }

    /// Creates an empty bitset with a capacity
    ///
    /// # Arguments
    ///
    /// `num_bits`: The number of bits to store
    pub fn with_capacity(num_bits: usize) -> Self {
        #[cfg(debug_assertions)]
        assert!(num_bits > 0);
        let num_chunks = Self::calc_chunks(num_bits);
        let chunks = vec![0; num_chunks];
        Self { chunks, num_bits }
    }
}

impl From<BitSet> for Vec<u32> {
    fn from(bitset: BitSet) -> Self {
        bitset.chunks
    }
}
