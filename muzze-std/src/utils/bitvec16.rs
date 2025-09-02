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
    /// use muzze_std::BitVec16;
    /// let bitvec = BitVec16::from_u16(0b1010_1010_1010_1010);
    /// ```
    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        Self::from_bits_retain(value)
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
    /// use muzze_std::BitVec16;
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
    /// use muzze_std::BitVec16;
    /// let bitvec = BitVec16::from_u16(0b0000_0000_0000_0001);
    /// assert_eq!(bitvec.bit(0), true);
    /// assert_eq!(bitvec.bit(1), false);
    /// ```
    #[inline]
    pub const fn bit(&self, index: usize) -> bool {
        self.bits() & (1 << index) != 0
    }

    /// Returns the value of bit 0 (least significant bit)
    ///
    /// This is a convenience method equivalent to `self.bit(0)`.
    ///
    /// # Returns
    /// `true` if bit 0 is set, `false` otherwise
    #[inline]
    pub const fn bit0(&self) -> bool {
        self.bit(0)
    }

    /// Returns the value of bit 1
    ///
    /// This is a convenience method equivalent to `self.bit(1)`.
    ///
    /// # Returns
    /// `true` if bit 1 is set, `false` otherwise
    #[inline]
    pub const fn bit1(&self) -> bool {
        self.bit(1)
    }

    /// Returns the value of bit 2
    ///
    /// This is a convenience method equivalent to `self.bit(2)`.
    ///
    /// # Returns
    /// `true` if bit 2 is set, `false` otherwise
    #[inline]
    pub const fn bit2(&self) -> bool {
        self.bit(2)
    }

    /// Returns the value of bit 3
    ///
    /// This is a convenience method equivalent to `self.bit(3)`.
    ///
    /// # Returns
    /// `true` if bit 3 is set, `false` otherwise
    #[inline]
    pub const fn bit3(&self) -> bool {
        self.bit(3)
    }

    /// Returns the value of bit 4
    ///
    /// This is a convenience method equivalent to `self.bit(4)`.
    ///
    /// # Returns
    /// `true` if bit 4 is set, `false` otherwise
    #[inline]
    pub const fn bit4(&self) -> bool {
        self.bit(4)
    }

    /// Returns the value of bit 5
    ///
    /// This is a convenience method equivalent to `self.bit(5)`.
    ///
    /// # Returns
    /// `true` if bit 5 is set, `false` otherwise
    #[inline]
    pub const fn bit5(&self) -> bool {
        self.bit(5)
    }

    /// Returns the value of bit 6
    ///
    /// This is a convenience method equivalent to `self.bit(6)`.
    ///
    /// # Returns
    /// `true` if bit 6 is set, `false` otherwise
    #[inline]
    pub const fn bit6(&self) -> bool {
        self.bit(6)
    }

    /// Returns the value of bit 7
    ///
    /// This is a convenience method equivalent to `self.bit(7)`.
    ///
    /// # Returns
    /// `true` if bit 7 is set, `false` otherwise
    #[inline]
    pub const fn bit7(&self) -> bool {
        self.bit(7)
    }

    /// Returns the value of bit 8
    ///
    /// This is a convenience method equivalent to `self.bit(8)`.
    ///
    /// # Returns
    /// `true` if bit 8 is set, `false` otherwise
    #[inline]
    pub const fn bit8(&self) -> bool {
        self.bit(8)
    }

    /// Returns the value of bit 9
    ///
    /// This is a convenience method equivalent to `self.bit(9)`.
    ///
    /// # Returns
    /// `true` if bit 9 is set, `false` otherwise
    #[inline]
    pub const fn bit9(&self) -> bool {
        self.bit(9)
    }

    /// Returns the value of bit 10
    ///
    /// This is a convenience method equivalent to `self.bit(10)`.
    ///
    /// # Returns
    /// `true` if bit 10 is set, `false` otherwise
    #[inline]
    pub const fn bit10(&self) -> bool {
        self.bit(10)
    }

    /// Returns the value of bit 11
    ///
    /// This is a convenience method equivalent to `self.bit(11)`.
    ///
    /// # Returns
    /// `true` if bit 11 is set, `false` otherwise
    #[inline]
    pub const fn bit11(&self) -> bool {
        self.bit(11)
    }

    /// Returns the value of bit 12
    ///
    /// This is a convenience method equivalent to `self.bit(12)`.
    ///
    /// # Returns
    /// `true` if bit 12 is set, `false` otherwise
    #[inline]
    pub const fn bit12(&self) -> bool {
        self.bit(12)
    }

    /// Returns the value of bit 13
    ///
    /// This is a convenience method equivalent to `self.bit(13)`.
    ///
    /// # Returns
    /// `true` if bit 13 is set, `false` otherwise
    #[inline]
    pub const fn bit13(&self) -> bool {
        self.bit(13)
    }

    /// Returns the value of bit 14
    ///
    /// This is a convenience method equivalent to `self.bit(14)`.
    ///
    /// # Returns
    /// `true` if bit 14 is set, `false` otherwise
    #[inline]
    pub const fn bit14(&self) -> bool {
        self.bit(14)
    }

    /// Returns the value of bit 15 (most significant bit)
    ///
    /// This is a convenience method equivalent to `self.bit(15)`.
    ///
    /// # Returns
    /// `true` if bit 15 is set, `false` otherwise
    #[inline]
    pub const fn bit15(&self) -> bool {
        self.bit(15)
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
    /// use muzze_std::BitVec16;
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
    /// use muzze_std::BitVec16;
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
    /// use muzze_std::BitVec16;
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

pub struct BitVec16Builder {
    vec: BitVec16,
}

impl BitVec16Builder {
    #[inline]
    pub const fn new() -> Self {
        let vec = BitVec16::from_u16(0);
        Self { vec }
    }

    #[inline]
    pub const fn set_index(self, index: u8) -> Self {
        let value = self.vec.inner() | (1 << index);
        let vec = BitVec16::from_u16(value);
        Self { vec }
    }

    #[inline]
    pub const fn build(self) -> BitVec16 {
        self.vec
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
        assert_eq!(bitvec[0], true);
        assert_eq!(bitvec[1], false);
        assert_eq!(bitvec[2], true);
        assert_eq!(bitvec[3], true);
        assert_eq!(bitvec[4], false);
        assert_eq!(bitvec[5], false);
        assert_eq!(bitvec[6], false);
        assert_eq!(bitvec[7], false);
        assert_eq!(bitvec[8], false);
        assert_eq!(bitvec[9], false);
        assert_eq!(bitvec[10], false);
        assert_eq!(bitvec[11], false);
        assert_eq!(bitvec[12], false);
        assert_eq!(bitvec[13], false);
        assert_eq!(bitvec[14], false);
        assert_eq!(bitvec[15], true);
    }

    #[test]
    fn test_bit_at_index() {
        let bitvec = BitVec16::from_u16(VAL);

        assert_eq!(bitvec.bit(0), true);
        assert_eq!(bitvec.bit(1), false);
        assert_eq!(bitvec.bit(2), true);
        assert_eq!(bitvec.bit(3), true);
        assert_eq!(bitvec.bit(4), false);
        assert_eq!(bitvec.bit(5), false);
        assert_eq!(bitvec.bit(6), false);
        assert_eq!(bitvec.bit(7), false);
        assert_eq!(bitvec.bit(8), false);
        assert_eq!(bitvec.bit(9), false);
        assert_eq!(bitvec.bit(10), false);
        assert_eq!(bitvec.bit(11), false);
        assert_eq!(bitvec.bit(12), false);
        assert_eq!(bitvec.bit(13), false);
        assert_eq!(bitvec.bit(14), false);
        assert_eq!(bitvec.bit(15), true);
    }

    #[test]
    fn test_individual_bits() {
        let bitvec = BitVec16::from_u16(VAL);

        // Test all individual bit methods
        assert_eq!(bitvec.bit0(), true);
        assert_eq!(bitvec.bit1(), false);
        assert_eq!(bitvec.bit2(), true);
        assert_eq!(bitvec.bit3(), true);
        assert_eq!(bitvec.bit4(), false);
        assert_eq!(bitvec.bit5(), false);
        assert_eq!(bitvec.bit6(), false);
        assert_eq!(bitvec.bit7(), false);
        assert_eq!(bitvec.bit8(), false);
        assert_eq!(bitvec.bit9(), false);
        assert_eq!(bitvec.bit10(), false);
        assert_eq!(bitvec.bit11(), false);
        assert_eq!(bitvec.bit12(), false);
        assert_eq!(bitvec.bit13(), false);
        assert_eq!(bitvec.bit14(), false);
        assert_eq!(bitvec.bit15(), true);
    }

    #[test]
    fn test_individual_bits_all_false() {
        let bitvec = BitVec16::from_u16(0);

        // Test all individual bit methods return false for 0
        assert_eq!(bitvec.bit0(), false);
        assert_eq!(bitvec.bit1(), false);
        assert_eq!(bitvec.bit2(), false);
        assert_eq!(bitvec.bit3(), false);
        assert_eq!(bitvec.bit4(), false);
        assert_eq!(bitvec.bit5(), false);
        assert_eq!(bitvec.bit6(), false);
        assert_eq!(bitvec.bit7(), false);
        assert_eq!(bitvec.bit8(), false);
        assert_eq!(bitvec.bit9(), false);
        assert_eq!(bitvec.bit10(), false);
        assert_eq!(bitvec.bit11(), false);
        assert_eq!(bitvec.bit12(), false);
        assert_eq!(bitvec.bit13(), false);
        assert_eq!(bitvec.bit14(), false);
        assert_eq!(bitvec.bit15(), false);
    }

    #[test]
    fn test_individual_bits_all_true() {
        let bitvec = BitVec16::from_u16(0xFFFF);

        // Test all individual bit methods return true for 0xFFFF
        assert_eq!(bitvec.bit0(), true);
        assert_eq!(bitvec.bit1(), true);
        assert_eq!(bitvec.bit2(), true);
        assert_eq!(bitvec.bit3(), true);
        assert_eq!(bitvec.bit4(), true);
        assert_eq!(bitvec.bit5(), true);
        assert_eq!(bitvec.bit6(), true);
        assert_eq!(bitvec.bit7(), true);
        assert_eq!(bitvec.bit8(), true);
        assert_eq!(bitvec.bit9(), true);
        assert_eq!(bitvec.bit10(), true);
        assert_eq!(bitvec.bit11(), true);
        assert_eq!(bitvec.bit12(), true);
        assert_eq!(bitvec.bit13(), true);
        assert_eq!(bitvec.bit14(), true);
        assert_eq!(bitvec.bit15(), true);
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

    #[test]
    fn test_builder() {
        let bitvec = BitVec16Builder::new().set_index(0).set_index(1).set_index(3).build();
        assert_eq!(bitvec.inner(), 0b0000_0000_0000_1011);
    }
    
}
