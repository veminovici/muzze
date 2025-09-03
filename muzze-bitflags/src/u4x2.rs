//! U4x2 - A packed representation of two 4-bit unsigned integers
//!
//! This module provides a compact way to store two 4-bit values (0-15) in a single u8.
//! The first value is stored in the lower 4 bits, and the second value is stored
//! in the upper 4 bits. This is useful for memory-efficient storage of small
//! musical data like note velocities, durations, or other parameters.

use bitflags::bitflags;

bitflags! {
    /// A packed representation of two 4-bit unsigned integers
    ///
    /// U4x2 stores two 4-bit values (0-15) in a single u8, with the first value
    /// in the lower 4 bits and the second value in the upper 4 bits. This provides
    /// a memory-efficient way to store pairs of small musical parameters.
    ///
    /// # Examples
    /// ```
    /// use muzze_bitflags::U4x2;
    /// let packed = U4x2::new(10, 5);  // First value: 10, Second value: 5
    /// assert_eq!(packed.first(), 10);
    /// assert_eq!(packed.second(), 5);
    /// assert_eq!(packed.inner(), 0b0101_1010);  // 5 << 4 | 10
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct U4x2: u8 {}
}

impl U4x2 {
    /// Bit mask for extracting the first 4-bit value (lower bits)
    const FIRST_MASK: u8 = 0b0000_1111;
    /// Bit mask for extracting the second 4-bit value (upper bits)
    const SECOND_MASK: u8 = 0b1111_0000;
    /// Number of bits to shift for the second value
    const SHIFT: u8 = 4;

    /// Creates a new U4x2 with the specified 4-bit values
    ///
    /// This method packs two 4-bit values into a single u8, with the first value
    /// stored in the lower 4 bits and the second value stored in the upper 4 bits.
    ///
    /// # Arguments
    /// * `first` - The first 4-bit value (0-15) to store in lower bits
    /// * `second` - The second 4-bit value (0-15) to store in upper bits
    ///
    /// # Returns
    /// A new U4x2 instance with the packed values
    ///
    /// # Panics
    /// This method will panic if either value exceeds 15 (4-bit limit)
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::U4x2;
    /// let packed = U4x2::new(10, 5);
    /// assert_eq!(packed.inner(), 0b0101_1010);  // 5 << 4 | 10
    /// ```
    #[inline]
    pub const fn new(first: u8, second: u8) -> Self {
        Self::from_bits_retain((second << Self::SHIFT) | first)
    }

    /// Returns the underlying u8 value containing both packed 4-bit values
    ///
    /// This method provides access to the raw u8 representation of the packed
    /// values, which can be useful for serialization or bit manipulation.
    ///
    /// # Returns
    /// The u8 value containing both packed 4-bit values
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::U4x2;
    /// let packed = U4x2::new(10, 5);
    /// assert_eq!(packed.inner(), 0b0101_1010);
    /// ```
    #[inline]
    pub const fn inner(&self) -> u8 {
        self.bits()
    }

    /// Extracts the first 4-bit value from the packed representation
    ///
    /// This method returns the value stored in the lower 4 bits of the u8.
    ///
    /// # Returns
    /// The first 4-bit value (0-15)
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::U4x2;
    /// let packed = U4x2::new(10, 5);
    /// assert_eq!(packed.first(), 10);
    /// ```
    #[inline]
    pub const fn first(&self) -> u8 {
        self.bits() & Self::FIRST_MASK
    }

    /// Extracts the second 4-bit value from the packed representation
    ///
    /// This method returns the value stored in the upper 4 bits of the u8,
    /// shifted down to the lower position.
    ///
    /// # Returns
    /// The second 4-bit value (0-15)
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::U4x2;
    /// let packed = U4x2::new(10, 5);
    /// assert_eq!(packed.second(), 5);
    /// ```
    #[inline]
    pub const fn second(&self) -> u8 {
        (self.bits() & Self::SECOND_MASK) >> Self::SHIFT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let u4x2 = U4x2::new(0b10, 0b1100);
        assert_eq!(u4x2.inner(), 0b1100_0010);
    }

    #[test]
    fn test_accessors() {
        let u4x2 = U4x2::new(0b10, 0b1100);
        assert_eq!(u4x2.first(), 0b10);
        assert_eq!(u4x2.second(), 0b1100);
    }
}
