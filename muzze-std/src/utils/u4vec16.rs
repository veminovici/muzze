//! U4Vec16 - A 16-element vector of 4-bit unsigned integers
//!
//! This module provides a U4Vec16 type that represents a vector of 16 elements,
//! where each element is a 4-bit unsigned integer (0-15). This is useful for
//! compact storage and efficient access to small integer values.

use bitflags::bitflags;
use std::ops::Index;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// U4Vec16 represents a vector of 16 elements, each being a 4-bit unsigned integer
    ///
    /// This type stores 16 values in the range 0-15 (4 bits each) within a single u64.
    /// It provides efficient access to individual elements and iteration capabilities.
    /// The underlying storage uses bitflags for efficient bit manipulation.
    pub struct U4Vec16: u64 {}
}

impl U4Vec16 {
    /// Bit mask for extracting a single 4-bit item (0b1111 = 15)
    const ITEM_MASK: u64 = 0b1111;
    /// Size of each item in bits
    const ITEM_SIZE: usize = 4;

    /// Creates a new U4Vec16 from a u64 value
    ///
    /// This method preserves all bits from the input value, allowing
    /// any 64-bit pattern to be stored. Each 4-bit segment represents
    /// one element in the vector.
    ///
    /// # Arguments
    /// * `value` - The u64 value to convert to U4Vec16
    ///
    /// # Returns
    /// A new U4Vec16 instance with the specified bit pattern
    ///
    /// # Example
    /// ```
    /// use muzze_std::U4Vec16;
    /// let vec = U4Vec16::from_u64(0x1234567890ABCDEF);
    /// ```
    #[inline]
    pub const fn from_u64(value: u64) -> Self {
        Self::from_bits_retain(value)
    }

    /// Returns the underlying u64 value of this U4Vec16
    ///
    /// This method extracts the raw u64 representation, which can be
    /// useful for serialization or when interfacing with other APIs.
    ///
    /// # Returns
    /// The u64 value representing the current bit pattern
    ///
    /// # Example
    /// ```
    /// use muzze_std::U4Vec16;
    /// let vec = U4Vec16::from_u64(42);
    /// assert_eq!(vec.inner(), 42);
    /// ```
    #[inline]
    pub const fn inner(&self) -> u64 {
        self.bits()
    }

    /// Returns the 4-bit item at the specified index
    ///
    /// This method extracts a 4-bit value from the specified position.
    /// Index 0 represents the least significant 4 bits, index 15 the most significant.
    /// Each item is in the range 0-15.
    ///
    /// # Arguments
    /// * `index` - The item position to access (0-15)
    ///
    /// # Returns
    /// The 4-bit value (0-15) at the specified position
    ///
    /// # Panics
    /// This method will panic if the index is out of bounds (> 15)
    ///
    /// # Example
    /// ```
    /// use muzze_std::U4Vec16;
    /// let vec = U4Vec16::from_u64(0x1234567890ABCDEF);
    /// assert_eq!(vec.item(0), 0x0F); // Least significant 4 bits
    /// assert_eq!(vec.item(15), 0x1); // Most significant 4 bits
    /// ```
    #[inline]
    pub const fn item(&self, index: usize) -> u8 {
        let val = self.bits() >> (Self::ITEM_SIZE * index);
        (val & Self::ITEM_MASK) as u8
    }

    /// Returns the 4-bit item at position 0 (least significant)
    ///
    /// This is a convenience method equivalent to `self.item(0)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 0
    #[inline]
    pub const fn item0(&self) -> u8 {
        self.item(0)
    }

    /// Returns the 4-bit item at position 1
    ///
    /// This is a convenience method equivalent to `self.item(1)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 1
    #[inline]
    pub const fn item1(&self) -> u8 {
        self.item(1)
    }

    /// Returns the 4-bit item at position 2
    ///
    /// This is a convenience method equivalent to `self.item(2)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 2
    #[inline]
    pub const fn item2(&self) -> u8 {
        self.item(2)
    }

    /// Returns the 4-bit item at position 3
    ///
    /// This is a convenience method equivalent to `self.item(3)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 3
    #[inline]
    pub const fn item3(&self) -> u8 {
        self.item(3)
    }

    /// Returns the 4-bit item at position 4
    ///
    /// This is a convenience method equivalent to `self.item(4)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 4
    #[inline]
    pub const fn item4(&self) -> u8 {
        self.item(4)
    }

    /// Returns the 4-bit item at position 5
    ///
    /// This is a convenience method equivalent to `self.item(5)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 5
    #[inline]
    pub const fn item5(&self) -> u8 {
        self.item(5)
    }

    /// Returns the 4-bit item at position 6
    ///
    /// This is a convenience method equivalent to `self.item(6)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 6
    #[inline]
    pub const fn item6(&self) -> u8 {
        self.item(6)
    }

    /// Returns the 4-bit item at position 7
    ///
    /// This is a convenience method equivalent to `self.item(7)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 7
    #[inline]
    pub const fn item7(&self) -> u8 {
        self.item(7)
    }

    /// Returns the 4-bit item at position 8
    ///
    /// This is a convenience method equivalent to `self.item(8)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 8
    #[inline]
    pub const fn item8(&self) -> u8 {
        self.item(8)
    }

    /// Returns the 4-bit item at position 9
    ///
    /// This is a convenience method equivalent to `self.item(9)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 9
    #[inline]
    pub const fn item9(&self) -> u8 {
        self.item(9)
    }

    /// Returns the 4-bit item at position 10
    ///
    /// This is a convenience method equivalent to `self.item(10)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 10
    #[inline]
    pub const fn item10(&self) -> u8 {
        self.item(10)
    }

    /// Returns the 4-bit item at position 11
    ///
    /// This is a convenience method equivalent to `self.item(11)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 11
    #[inline]
    pub const fn item11(&self) -> u8 {
        self.item(11)
    }

    /// Returns the 4-bit item at position 12
    ///
    /// This is a convenience method equivalent to `self.item(12)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 12
    #[inline]
    pub const fn item12(&self) -> u8 {
        self.item(12)
    }

    /// Returns the 4-bit item at position 13
    ///
    /// This is a convenience method equivalent to `self.item(13)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 13
    #[inline]
    pub const fn item13(&self) -> u8 {
        self.item(13)
    }

    /// Returns the 4-bit item at position 14
    ///
    /// This is a convenience method equivalent to `self.item(14)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 14
    #[inline]
    pub const fn item14(&self) -> u8 {
        self.item(14)
    }

    /// Returns the 4-bit item at position 15 (most significant)
    ///
    /// This is a convenience method equivalent to `self.item(15)`.
    ///
    /// # Returns
    /// The 4-bit value (0-15) at position 15
    #[inline]
    pub const fn item15(&self) -> u8 {
        self.item(15)
    }

    /// Returns an iterator over all 4-bit items in this U4Vec16
    ///
    /// The iterator yields each 4-bit item as a u8 value, starting from
    /// position 0 (least significant) and ending with position 15 (most significant).
    /// Each yielded value is in the range 0-15. This is useful for processing
    /// all items sequentially or collecting them into a vector.
    ///
    /// # Returns
    /// A U4Vec16Iter that implements Iterator<Item = u8>
    ///
    /// # Example
    /// ```
    /// use muzze_std::U4Vec16;
    /// let vec = U4Vec16::from_u64(0x1234567890ABCDEF);
    /// let items: Vec<u8> = vec.iter_items().collect();
    /// assert_eq!(items.len(), 16);
    /// ```
    #[inline]
    pub fn iter_items(&self) -> U4Vec16Iter {
        U4Vec16Iter::new(*self)
    }
}

/// Iterator over the 4-bit items of a U4Vec16
///
/// This iterator yields each 4-bit item of the U4Vec16 as a u8 value,
/// starting from position 0 (least significant) to position 15 (most significant).
/// Each yielded value is in the range 0-15. It implements ExactSizeIterator
/// for efficient collection operations.
pub struct U4Vec16Iter {
    /// The U4Vec16 being iterated over
    vec: U4Vec16,
    /// Current item index (0-15)
    index: usize,
}

impl U4Vec16Iter {
    /// The total number of items in a U4Vec16
    const LENGTH: usize = 16;

    /// Creates a new U4Vec16Iter starting from the beginning
    ///
    /// # Arguments
    /// * `vec` - The U4Vec16 to iterate over
    ///
    /// # Returns
    /// A new iterator positioned at item 0
    fn new(vec: U4Vec16) -> Self {
        Self { vec, index: 0 }
    }
}

impl Iterator for U4Vec16Iter {
    /// The type of item yielded by the iterator
    type Item = u8;

    /// Returns the next 4-bit item in the sequence
    ///
    /// This method advances the iterator and returns the next item
    /// as a u8 value in the range 0-15, or None if all items
    /// have been consumed.
    ///
    /// # Returns
    /// Some(u8) containing the next 4-bit item, or None if exhausted
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < Self::LENGTH {
            let item = self.vec.item(self.index);
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    /// Provides a hint about the number of remaining items
    ///
    /// This method returns the exact number of remaining items,
    /// which is useful for optimizing collection operations.
    ///
    /// # Returns
    /// A tuple (usize, Option<usize>) where both values are the same
    /// and represent the exact number of remaining items
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = Self::LENGTH - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for U4Vec16Iter {}

impl Index<usize> for U4Vec16 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        const VALS: [u8; 16] = [
            0b0000, 0b0001, 0b0010, 0b0011, 0b0100, 0b0101, 0b0110, 0b0111, 0b1000, 0b1001, 0b1010,
            0b1011, 0b1100, 0b1101, 0b1110, 0b1111,
        ];

        let item = self.item(index);
        &VALS[item as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test value with a specific pattern for testing all 16 positions
    ///
    /// This constant represents a u64 where:
    /// - Positions 0-3: 0b1010, 0b1011, 0b1110, 0b1111 (10, 11, 14, 15)
    /// - Positions 4-11: 0b0000 (0) - all zeros
    /// - Positions 12-15: 0b1010, 0b1011, 0b1110, 0b1111 (10, 11, 14, 15)
    const VAL: u64 =
        0b1111_1110_1011_1010_0000_0000_0000_0000_0000_0000_0000_0000_1111_1110_1011_1010;

    /// Tests that the inner() method correctly returns the underlying u64 value
    #[test]
    fn test_inner() {
        let vec = U4Vec16::from_u64(VAL);
        assert_eq!(vec.inner(), VAL);
    }

    /// Tests that the item() method correctly extracts 4-bit values from all positions
    ///
    /// This test verifies that each position returns the expected 4-bit value
    /// based on the test pattern defined in VAL constant.
    #[test]
    fn test_item() {
        let vec = U4Vec16::from_u64(VAL);
        assert_eq!(vec.item(0), 0b1010);
        assert_eq!(vec.item(1), 0b1011);
        assert_eq!(vec.item(2), 0b1110);
        assert_eq!(vec.item(3), 0b1111);
        assert_eq!(vec.item(4), 0b0000);
        assert_eq!(vec.item(5), 0b0000);
        assert_eq!(vec.item(6), 0b0000);
        assert_eq!(vec.item(7), 0b0000);
        assert_eq!(vec.item(8), 0b0000);
        assert_eq!(vec.item(9), 0b0000);
        assert_eq!(vec.item(10), 0b0000);
        assert_eq!(vec.item(11), 0b0000);
        assert_eq!(vec.item(12), 0b1010);
        assert_eq!(vec.item(13), 0b1011);
        assert_eq!(vec.item(14), 0b1110);
        assert_eq!(vec.item(15), 0b1111);
    }

    /// Tests that all individual item accessor methods work correctly
    ///
    /// This test verifies that each convenience method (item0() through item15())
    /// returns the same value as the corresponding item() method call.
    #[test]
    fn test_items() {
        let vec = U4Vec16::from_u64(VAL);
        assert_eq!(vec.item0(), 0b1010);
        assert_eq!(vec.item1(), 0b1011);
        assert_eq!(vec.item2(), 0b1110);
        assert_eq!(vec.item3(), 0b1111);
        assert_eq!(vec.item4(), 0b0000);
        assert_eq!(vec.item5(), 0b0000);
        assert_eq!(vec.item6(), 0b0000);
        assert_eq!(vec.item7(), 0b0000);
        assert_eq!(vec.item8(), 0b0000);
        assert_eq!(vec.item9(), 0b0000);
        assert_eq!(vec.item10(), 0b0000);
        assert_eq!(vec.item11(), 0b0000);
        assert_eq!(vec.item12(), 0b1010);
        assert_eq!(vec.item13(), 0b1011);
        assert_eq!(vec.item14(), 0b1110);
        assert_eq!(vec.item15(), 0b1111);
    }

    /// Tests that the iterator correctly yields all 16 items in sequence
    ///
    /// This test verifies that iter_items() produces the expected sequence
    /// of 4-bit values when collected into a vector.
    #[test]
    fn test_iter_item() {
        let vec = U4Vec16::from_u64(VAL);
        let items: Vec<u8> = vec.iter_items().collect();
        let expect = vec![
            0b1010, 0b1011, 0b1110, 0b1111, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
            0b0000, 0b1010, 0b1011, 0b1110, 0b1111,
        ];
        assert_eq!(items, expect);
    }

    /// Tests that the iterator provides correct size information
    ///
    /// This test verifies that the iterator's len() method returns 16
    /// and that collecting all items produces a vector of the expected size.
    #[test]
    fn test_iter_items_size() {
        let vec = U4Vec16::from_u64(VAL);
        let iter = vec.iter_items();
        assert_eq!(iter.len(), 16);

        let items: Vec<u8> = vec.iter_items().collect();
        assert_eq!(items.len(), 16);
    }

    #[test]
    fn test_index() {
        let vec = U4Vec16::from_u64(VAL);
        assert_eq!(vec[0], 0b1010);
        assert_eq!(vec[1], 0b1011);
        assert_eq!(vec[2], 0b1110);
        assert_eq!(vec[3], 0b1111);
        assert_eq!(vec[4], 0b0000);
        assert_eq!(vec[5], 0b0000);
        assert_eq!(vec[6], 0b0000);
        assert_eq!(vec[7], 0b0000);
        assert_eq!(vec[8], 0b0000);
        assert_eq!(vec[9], 0b0000);
        assert_eq!(vec[10], 0b0000);
        assert_eq!(vec[11], 0b0000);
        assert_eq!(vec[12], 0b1010);
        assert_eq!(vec[13], 0b1011);
        assert_eq!(vec[14], 0b1110);
        assert_eq!(vec[15], 0b1111);
    }
}
