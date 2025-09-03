//! BitVec16 - A 16-bit vector implementation using bitflags
//!
//! This module provides a BitVec16 type that represents a 16-bit vector
//! with efficient bit-level operations and iteration capabilities.

use bitflags::bitflags;
use std::ops::Index;

bitflags! {
    /// BitVec16 represents a 16-bit vector using the bitflags crate
    ///
    /// This type provides efficient bit-level operations and can be used
    /// for flags, masks, or any 16-bit bit pattern representation.
    /// Each bit position (0-15) is accessible as a named constant.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BitVec16: u16 {}
}

impl BitVec16 {
    /// The total number of bits in a BitVec16
    const CAPACITY: usize = 16;

    /// Creates a new BitVec16 from a u16 value
    ///
    /// This method preserves all bits from the input value, including
    /// any bits that might not correspond to defined constants.
    ///
    /// # Arguments
    /// * `value` - The u16 value to convert to BitVec16
    ///
    /// # Returns
    /// A new BitVec16 instance with the specified bit pattern
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::BitVec16;
    /// let bitvec = BitVec16::from_u16(0b1010_1010_1010_1010);
    /// ```
    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        Self::from_bits_retain(value)
    }

    /// Creates a new BitVec16 from a vector of booleans
    ///
    /// This method creates a BitVec16 from a vector of booleans, where each boolean represents a bit in the BitVec16.
    ///
    /// # Arguments
    /// * `bits` - A vector of booleans representing the bits of the BitVec16
    ///
    /// # Returns
    /// A new BitVec16 instance with the specified bit pattern
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::BitVec16;
    /// let bitvec = BitVec16::from_vec([true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, true]);
    /// ```
    #[inline]
    pub fn from_vec(bits: [bool; 16]) -> Self {
        let value = bits.into_iter().enumerate().fold(0, |acc, (index, b)| {
            let b = (b as u16) << index;
            acc | b
        });

        Self::from_u16(value)
    }

    /// Returns the total number of bits in a BitVec16
    ///
    /// # Returns
    /// The total number of bits in a BitVec16
    #[inline]
    pub const fn capacity(&self) -> usize {
        Self::CAPACITY
    }

    /// Returns the underlying u16 value of this BitVec16
    ///
    /// This method extracts the raw u16 representation, which can be
    /// useful for serialization or when interfacing with other APIs.
    ///
    /// # Returns
    /// The u16 value representing the current bit pattern
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::BitVec16;
    /// let bitvec = BitVec16::from_u16(42);
    /// assert_eq!(bitvec.inner(), 42);
    /// ```
    #[inline]
    pub const fn inner(&self) -> u16 {
        self.bits()
    }

    /// Returns the value of the bit at the specified index
    ///
    /// This method checks if the bit at the given position is set.
    /// Index 0 represents the least significant bit, index 15 the most significant.
    ///
    /// # Arguments
    /// * `index` - The bit position to check (0-15)
    ///
    /// # Returns
    /// `true` if the bit is set, `false` otherwise
    ///
    /// # Panics
    /// This method will panic if the index is out of bounds (> 15)
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::BitVec16;
    /// let bitvec = BitVec16::from_u16(0b0000_0000_0000_0001);
    /// assert_eq!(bitvec.bit(0), true);
    /// assert_eq!(bitvec.bit(1), false);
    /// ```
    #[inline]
    pub const fn bit(&self, index: usize) -> bool {
        self.bits() & (1 << index) != 0
    }

    /// Returns an iterator over all bits in this BitVec16
    ///
    /// The iterator yields each bit as a boolean value, starting from
    /// bit 0 (least significant) and ending with bit 15 (most significant).
    /// This is useful for processing all bits sequentially or collecting
    /// them into a vector.
    ///
    /// # Returns
    /// A BitVec16Iter that implements Iterator<Item = bool>
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::BitVec16;
    /// let bitvec = BitVec16::from_u16(0b1010_1010_1010_1010);
    /// let bits: Vec<bool> = bitvec.iter_bits().collect();
    /// assert_eq!(bits.len(), 16);
    /// ```
    #[inline]
    pub fn iter_bits(&self) -> BitVec16Iter {
        BitVec16Iter::new(*self)
    }

    /// Returns an iterator over the indices of bits that are set (true)
    ///
    /// This method yields the positions (0-15) where bits are set to true.
    /// It's useful for finding which specific bits are active in the bit vector.
    ///
    /// # Returns
    /// An iterator that yields usize values representing the positions of set bits
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::BitVec16;
    /// let bitvec = BitVec16::from_u16(0b1000_0000_0000_1101);
    /// let on_indices: Vec<usize> = bitvec.indeces_on().collect();
    /// assert_eq!(on_indices, vec![0, 2, 3, 15]);
    /// ```
    #[inline]
    pub fn indeces_on(&self) -> impl Iterator<Item = usize> {
        self.iter_bits()
            .enumerate()
            .filter_map(|(i, b)| if b { Some(i) } else { None })
    }

    /// Returns an iterator over the indices of bits that are not set (false)
    ///
    /// This method yields the positions (0-15) where bits are set to false.
    /// It's useful for finding which specific bits are inactive in the bit vector.
    ///
    /// # Returns
    /// An iterator that yields usize values representing the positions of unset bits
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::BitVec16;
    /// let bitvec = BitVec16::from_u16(0b1000_0000_0000_1101);
    /// let off_indices: Vec<usize> = bitvec.indeces_off().collect();
    /// assert_eq!(off_indices, vec![1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    /// ```
    #[inline]
    pub fn indeces_off(&self) -> impl Iterator<Item = usize> {
        self.iter_bits()
            .enumerate()
            .filter_map(|(i, b)| if !b { Some(i) } else { None })
    }
}

/// Builder for constructing BitVec16 instances
///
/// The BitVec16Builder provides a fluent interface for constructing BitVec16
/// instances by setting individual bits. This is useful when you need to
/// build a bit vector programmatically or when the bit pattern is not known
/// at compile time.
///
/// # Examples
/// ```
/// use muzze_bitflags::BitVec16Builder;
/// let bitvec = BitVec16Builder::default()
///     .set_index(0)
///     .set_index(2)
///     .set_index(15)
///     .build();
/// assert_eq!(bitvec.inner(), 0b1000_0000_0000_0101);
/// ```
pub struct BitVec16Builder {
    /// The BitVec16 being constructed
    vec: BitVec16,
}

impl BitVec16Builder {
    /// Creates a new BitVec16Builder with all bits initially set to false
    ///
    /// This method initializes the builder with an empty BitVec16 (all bits unset).
    /// The builder can then be used to set specific bits using the `set_index` method.
    ///
    /// # Returns
    /// A new BitVec16Builder instance ready for configuration
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::BitVec16Builder;
    /// let builder = BitVec16Builder::default();
    /// let bitvec = builder.build();
    /// assert_eq!(bitvec.inner(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        let vec = BitVec16::from_u16(0);
        Self { vec }
    }

    /// Sets the bit at the specified index to true
    ///
    /// This method sets the bit at the given position (0-15) to true,
    /// leaving all other bits unchanged. The method returns a new builder
    /// instance, allowing for method chaining.
    ///
    /// # Arguments
    /// * `index` - The bit position to set (0-15)
    ///
    /// # Returns
    /// A new BitVec16Builder with the specified bit set
    ///
    /// # Panics
    /// This method will panic if the index is out of bounds (> 15)
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::BitVec16Builder;
    /// let bitvec = BitVec16Builder::default()
    ///     .set_index(0)
    ///     .set_index(7)
    ///     .build();
    /// assert_eq!(bitvec.bit(0), true);
    /// assert_eq!(bitvec.bit(7), true);
    /// assert_eq!(bitvec.bit(1), false);
    /// ```
    #[inline]
    pub const fn set_index(self, index: u8) -> Self {
        let value = self.vec.inner() | (1 << index);
        let vec = BitVec16::from_u16(value);
        Self { vec }
    }

    /// Finalizes the builder and returns the constructed BitVec16
    ///
    /// This method consumes the builder and returns the final BitVec16
    /// instance with all the bits that were set during construction.
    ///
    /// # Returns
    /// The constructed BitVec16 instance
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::BitVec16Builder;
    /// let bitvec = BitVec16Builder::default()
    ///     .set_index(0)
    ///     .set_index(2)
    ///     .set_index(3)
    ///     .build();
    /// assert_eq!(bitvec.inner(), 0b0000_0000_0000_1101);
    /// ```
    #[inline]
    pub const fn build(self) -> BitVec16 {
        self.vec
    }
}

impl Default for BitVec16Builder {
    /// Creates a default BitVec16Builder instance
    ///
    /// This implementation provides a convenient way to create a new builder
    /// using the `Default` trait, which is equivalent to calling `BitVec16Builder::new()`.
    ///
    /// # Returns
    /// A new BitVec16Builder instance with all bits initially unset
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over the bits of a BitVec16
///
/// This iterator yields each bit of the BitVec16 as a boolean value,
/// starting from bit 0 (least significant) to bit 15 (most significant).
/// It implements ExactSizeIterator for efficient collection operations.
pub struct BitVec16Iter {
    /// The BitVec16 being iterated over
    vec: BitVec16,
    /// Current bit index (0-15)
    index: usize,
}

impl BitVec16Iter {
    /// Creates a new BitVec16Iter starting from the beginning
    ///
    /// # Arguments
    /// * `bitvec` - The BitVec16 to iterate over
    ///
    /// # Returns
    /// A new iterator positioned at bit 0
    fn new(vec: BitVec16) -> Self {
        Self { vec, index: 0 }
    }
}

impl Iterator for BitVec16Iter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < BitVec16::CAPACITY {
            let bit = self.vec.bit(self.index);
            self.index += 1;
            Some(bit)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = BitVec16::CAPACITY - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for BitVec16Iter {}

impl Index<usize> for BitVec16 {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if self.bit(index) {
            &true
        } else {
            &false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VAL: u16 = 0b1000_0000_0000_1101;

    #[test]
    fn test_inner() {
        let bitvec = BitVec16::from_u16(VAL);

        // Check the inner value
        assert_eq!(bitvec.inner(), VAL);
    }

    #[test]
    fn test_index() {
        let bitvec = BitVec16::from_u16(VAL);

        // Check the bits are set correctly
        assert!(bitvec[0]);
        assert!(!bitvec[1]);
        assert!(bitvec[2]);
        assert!(bitvec[3]);
        assert!(!bitvec[4]);
        assert!(!bitvec[5]);
        assert!(!bitvec[6]);
        assert!(!bitvec[7]);
        assert!(!bitvec[8]);
        assert!(!bitvec[9]);
        assert!(!bitvec[10]);
        assert!(!bitvec[11]);
        assert!(!bitvec[12]);
        assert!(!bitvec[13]);
        assert!(!bitvec[14]);
        assert!(bitvec[15]);
    }

    #[test]
    fn test_bit_at_index() {
        let bitvec = BitVec16::from_u16(VAL);

        assert!(bitvec.bit(0));
        assert!(!bitvec.bit(1));
        assert!(bitvec.bit(2));
        assert!(bitvec.bit(3));
        assert!(!bitvec.bit(4));
        assert!(!bitvec.bit(5));
        assert!(!bitvec.bit(6));
        assert!(!bitvec.bit(7));
        assert!(!bitvec.bit(8));
        assert!(!bitvec.bit(9));
        assert!(!bitvec.bit(10));
        assert!(!bitvec.bit(11));
        assert!(!bitvec.bit(12));
        assert!(!bitvec.bit(13));
        assert!(!bitvec.bit(14));
        assert!(bitvec.bit(15));
    }

    #[test]
    fn test_iter_bits() {
        let bitvec = BitVec16::from_u16(VAL);

        let bits: Vec<bool> = bitvec.iter_bits().collect();
        let expected = vec![
            true, false, true, true, false, false, false, false, false, false, false, false, false,
            false, false, true,
        ];
        assert_eq!(bits, expected);
    }

    #[test]
    fn test_iter_bits_size() {
        let bitvec = BitVec16::from_u16(0);
        let iter = bitvec.iter_bits();
        assert_eq!(iter.len(), 16);

        let bits: Vec<bool> = iter.collect();
        assert_eq!(bits.len(), 16);
    }

    #[test]
    fn test_iter_bits_all_false() {
        let bitvec = BitVec16::from_u16(0);
        let bits: Vec<bool> = bitvec.iter_bits().collect();
        assert_eq!(bits, vec![false; 16]);
    }

    #[test]
    fn test_iter_bits_all_true() {
        let bitvec = BitVec16::from_u16(0xFFFF);
        let bits: Vec<bool> = bitvec.iter_bits().collect();
        assert_eq!(bits, vec![true; 16]);
    }

    #[test]
    fn test_indeces_on() {
        let bitvec = BitVec16::from_u16(0b1000_0000_0000_1101);
        let indeces: Vec<usize> = bitvec.indeces_on().collect();
        assert_eq!(indeces, vec![0, 2, 3, 15]);
    }

    #[test]
    fn test_indeces_off() {
        let bitvec = BitVec16::from_u16(0b1000_0000_0000_1101);
        let indeces: Vec<usize> = bitvec.indeces_off().collect();
        assert_eq!(indeces, vec![1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    }

    /// Tests that the BitVec16Builder correctly constructs BitVec16 instances
    ///
    /// This test verifies that the builder pattern works correctly by setting
    /// multiple bits and ensuring the final result matches the expected bit pattern.
    #[test]
    fn test_builder() {
        let bitvec = BitVec16Builder::default()
            .set_index(0)
            .set_index(1)
            .set_index(3)
            .build();
        assert_eq!(bitvec.inner(), 0b0000_0000_0000_1011);
    }

    #[test]
    fn test_from_vec() {
        let bitvec = BitVec16::from_vec([
            true, false, true, true, false, false, false, false, false, false, false, false, false,
            false, false, true,
        ]);
        assert_eq!(bitvec.inner(), 0b1000_0000_0000_1101);
    }
}
