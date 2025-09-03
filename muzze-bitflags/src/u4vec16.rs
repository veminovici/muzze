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
    /// The total number of items in a U4Vec16
    const CAPACITY: usize = 16;

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
    /// use muzze_bitflags::U4Vec16;
    /// let vec = U4Vec16::from_u64(0x1234567890ABCDEF);
    /// ```
    #[inline]
    pub const fn from_u64(value: u64) -> Self {
        Self::from_bits_retain(value)
    }

    /// Creates a new U4Vec16 from a vector of 4-bit values
    ///
    /// This method creates a U4Vec16 from a vector of 4-bit values, where each value represents an element in the U4Vec16.
    ///
    /// # Arguments
    /// * `items` - A vector of 4-bit values to convert to U4Vec16
    ///
    /// # Returns
    /// A new U4Vec16 instance with the specified bit pattern
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::U4Vec16;
    ///let vec = U4Vec16::from_vec([0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0D]);
    ///assert_eq!(vec.inner(), 0xD000_0000_0000_000A);
    /// ```
    #[inline]
    pub fn from_vec(items: [u8; 16]) -> Self {
        let value = items.into_iter().enumerate().fold(0, |acc, (index, item)| {
            let item = (item as u64) << (Self::ITEM_SIZE * index);
            acc | item
        });
        Self::from_u64(value)
    }

    /// Returns the total number of items in a U4Vec16
    ///
    /// # Returns
    /// The total number of items in a U4Vec16
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::U4Vec16;
    /// let vec = U4Vec16::from_u64(0x1234567890ABCDEF);
    /// assert_eq!(vec.capacity(), 16);
    /// ```
    #[inline]
    pub const fn capacity(&self) -> usize {
        Self::CAPACITY
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
    /// use muzze_bitflags::U4Vec16;
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
    /// use muzze_bitflags::U4Vec16;
    /// let vec = U4Vec16::from_u64(0x1234567890ABCDEF);
    /// assert_eq!(vec.item(0), 0x0F); // Least significant 4 bits
    /// assert_eq!(vec.item(15), 0x1); // Most significant 4 bits
    /// ```
    #[inline]
    pub const fn item(&self, index: usize) -> u8 {
        let val = self.bits() >> (Self::ITEM_SIZE * index);
        (val & Self::ITEM_MASK) as u8
    }

    /// Resets the 4-bit item at the specified index
    ///
    /// This method resets the 4-bit item at the specified index to 0.
    ///
    /// # Arguments
    /// * `index` - The item position to reset (0-15)
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::U4Vec16;
    /// let mut vec = U4Vec16::from_u64(0x1234567890ABCDEF);
    /// let vec = vec.reset_item(0);
    /// assert_eq!(vec.item(0), 0x00);
    /// ```
    #[inline]
    pub const fn reset_item(self, index: usize) -> Self {
        let item = self.item(index) as u64;
        let mask = item << (Self::ITEM_SIZE * index);
        Self::from_u64(self.inner() ^ mask)
    }

    /// Sets the 4-bit item at the specified index to the given value
    ///
    /// This method sets the 4-bit item at the specified index to the given value.
    ///
    /// # Arguments
    /// * `index` - The item position to set (0-15)
    /// * `value` - The 4-bit value to set (0-15)
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::U4Vec16;
    /// let mut vec = U4Vec16::from_u64(0x1234567890ABCDEF);
    /// let vec = vec.set_item(0, 0x0F);
    /// assert_eq!(vec.item(0), 0x0F);
    /// ```
    #[inline]
    pub const fn set_item(self, index: usize, value: u8) -> Self {
        let vec = self.reset_item(index);

        let item = value as u64 & Self::ITEM_MASK;
        let mask = item << (Self::ITEM_SIZE * index);
        Self::from_u64(vec.inner() | mask)
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
    /// use muzze_bitflags::U4Vec16;
    /// let vec = U4Vec16::from_u64(0x1234567890ABCDEF);
    /// let items: Vec<u8> = vec.iter_items().collect();
    /// assert_eq!(items.len(), 16);
    /// ```
    #[inline]
    pub fn iter_items(&self) -> U4Vec16Iter {
        U4Vec16Iter::new(*self)
    }
}

/// Builder for constructing U4Vec16 instances
///
/// U4Vec16Builder provides a fluent interface for constructing U4Vec16 instances
/// by setting individual 4-bit items. This is useful when you need to build
/// a U4Vec16 incrementally or when the final pattern is not known at compile time.
///
/// # Examples
///
/// ```rust
/// use muzze_bitflags::u4vec16::U4Vec16Builder;
///
/// // Build a U4Vec16 with specific items
/// let vec = U4Vec16Builder::new()
///     .set_item(0, 5)   // Set item 0 to 5
///     .set_item(1, 10)  // Set item 1 to 10
///     .set_item(15, 15) // Set item 15 to 15
///     .build();
///
/// assert_eq!(vec.item(0), 5);
/// assert_eq!(vec.item(1), 10);
/// assert_eq!(vec.item(15), 15);
/// ```
///
/// # Performance
///
/// The builder pattern allows for efficient construction of U4Vec16 instances
/// without intermediate allocations. Each `set_item` call returns a new builder
/// instance, allowing for method chaining.
pub struct U4Vec16Builder {
    /// The U4Vec16 being constructed
    vec: U4Vec16,
}

impl U4Vec16Builder {
    /// Creates a new U4Vec16Builder with all items initialized to 0
    ///
    /// This method creates a fresh builder instance with a U4Vec16 containing
    /// all zeros. You can then use `set_item` to configure specific positions
    /// and `build` to finalize the construction.
    ///
    /// # Returns
    /// A new U4Vec16Builder instance with all items set to 0
    ///
    /// # Example
    /// ```rust
    /// use muzze_bitflags::u4vec16::U4Vec16Builder;
    ///
    /// let builder = U4Vec16Builder::new();
    /// let vec = builder.build();
    /// assert_eq!(vec.inner(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self {
            vec: U4Vec16::from_u64(0),
        }
    }

    /// Sets a 4-bit item at the specified index
    ///
    /// This method sets the 4-bit item at the given index to the specified value.
    /// The value must be in the range 0-15 (4 bits). This method returns a new
    /// builder instance, allowing for method chaining.
    ///
    /// # Arguments
    /// * `index` - The item position to set (0-15)
    /// * `value` - The 4-bit value to set (0-15)
    ///
    /// # Returns
    /// A new U4Vec16Builder instance with the specified item set
    ///
    /// # Panics
    /// This method will panic if the index is out of bounds (> 15)
    ///
    /// # Example
    /// ```rust
    /// use muzze_bitflags::u4vec16::U4Vec16Builder;
    ///
    /// let vec = U4Vec16Builder::new()
    ///     .set_item(0, 5)
    ///     .set_item(1, 10)
    ///     .build();
    ///
    /// assert_eq!(vec.item(0), 5);
    /// assert_eq!(vec.item(1), 10);
    /// ```
    #[inline]
    pub const fn set_item(self, index: usize, value: u8) -> Self {
        let vec = self.vec.set_item(index, value);
        Self { vec }
    }

    /// Finalizes the construction and returns the built U4Vec16
    ///
    /// This method consumes the builder and returns the constructed U4Vec16.
    /// After calling this method, the builder can no longer be used.
    ///
    /// # Returns
    /// The constructed U4Vec16 instance
    ///
    /// # Example
    /// ```rust
    /// use muzze_bitflags::u4vec16::U4Vec16Builder;
    ///
    /// let builder = U4Vec16Builder::new()
    ///     .set_item(0, 5)
    ///     .set_item(15, 10);
    ///
    /// let vec = builder.build();
    /// assert_eq!(vec.item(0), 5);
    /// assert_eq!(vec.item(15), 10);
    /// ```
    #[inline]
    pub const fn build(self) -> U4Vec16 {
        self.vec
    }
}

impl Default for U4Vec16Builder {
    /// Creates a default U4Vec16Builder with all items initialized to 0
    ///
    /// This implementation allows using `U4Vec16Builder::default()` as an
    /// alternative to `U4Vec16Builder::new()`.
    ///
    /// # Returns
    /// A new U4Vec16Builder instance with all items set to 0
    ///
    /// # Example
    /// ```rust
    /// use muzze_bitflags::u4vec16::U4Vec16Builder;
    ///
    /// let vec = U4Vec16Builder::default()
    ///     .set_item(0, 5)
    ///     .build();
    ///
    /// assert_eq!(vec.item(0), 5);
    /// ```
    fn default() -> Self {
        Self::new()
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
    /// Creates a new U4Vec16Iter starting from the beginning
    ///
    /// # Arguments
    /// * `vec` - The U4Vec16 to iterate over
    ///
    /// # Returns
    /// A new iterator positioned at item 0
    #[inline]
    const fn new(vec: U4Vec16) -> Self {
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
        if self.index < U4Vec16::CAPACITY {
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
    /// A tuple where both values are the same
    /// and represent the exact number of remaining items
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = U4Vec16::CAPACITY - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for U4Vec16Iter {}

impl Index<usize> for U4Vec16 {
    /// The output type when indexing into U4Vec16
    type Output = u8;

    /// Returns a reference to the 4-bit item at the specified index
    ///
    /// This method allows using bracket notation to access items in the U4Vec16.
    /// It returns a reference to a static u8 value representing the 4-bit item.
    /// The returned value is always in the range 0-15.
    ///
    /// # Arguments
    /// * `index` - The item position to access (0-15)
    ///
    /// # Returns
    /// A reference to a u8 value representing the 4-bit item at the specified position
    ///
    /// # Panics
    /// This method will panic if the index is out of bounds (> 15)
    ///
    /// # Example
    /// ```
    /// use muzze_bitflags::U4Vec16;
    /// let vec = U4Vec16::from_u64(0x1234567890ABCDEF);
    /// assert_eq!(vec[0], 0x0F); // Access item at position 0
    /// assert_eq!(vec[15], 0x1); // Access item at position 15
    /// ```
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        // Static array of all possible 4-bit values (0-15) for efficient lookup
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

    /// Tests that the Index trait implementation works correctly
    ///
    /// This test verifies that bracket notation (vec[index]) correctly
    /// accesses the 4-bit items at each position, returning the expected
    /// values based on the test pattern defined in VAL constant.
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

    #[test]
    fn test_reset_item() {
        let vec = U4Vec16::from_u64(VAL);
        let vec = vec.reset_item(1);

        let items: Vec<u8> = vec.iter_items().collect();
        let expect = vec![
            0b1010, 0b0000, 0b1110, 0b1111, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
            0b0000, 0b1010, 0b1011, 0b1110, 0b1111,
        ];
        assert_eq!(items, expect);
    }

    #[test]
    fn test_set_item() {
        let vec = U4Vec16::from_u64(VAL);
        let vec = vec.set_item(1, 0b1010);

        let items: Vec<u8> = vec.iter_items().collect();
        let expect = vec![
            0b1010, 0b1010, 0b1110, 0b1111, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
            0b0000, 0b1010, 0b1011, 0b1110, 0b1111,
        ];
        assert_eq!(items, expect);
    }

    #[test]
    fn test_from_vec() {
        let vec = U4Vec16::from_vec([
            0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x0D,
        ]);
        assert_eq!(vec.inner(), 0xD000_0000_0000_000A);
    }

    // U4Vec16Builder tests

    /// Tests that U4Vec16Builder::new() creates a builder with all items set to 0
    #[test]
    fn test_builder_new() {
        let builder = U4Vec16Builder::new();
        let vec = builder.build();
        assert_eq!(vec.inner(), 0);

        // Verify all items are 0
        for i in 0..16 {
            assert_eq!(vec.item(i), 0);
        }
    }

    /// Tests that U4Vec16Builder::default() creates a builder with all items set to 0
    #[test]
    fn test_builder_default() {
        let builder = U4Vec16Builder::default();
        let vec = builder.build();
        assert_eq!(vec.inner(), 0);

        // Verify all items are 0
        for i in 0..16 {
            assert_eq!(vec.item(i), 0);
        }
    }

    /// Tests that set_item correctly sets individual items
    #[test]
    fn test_builder_set_item() {
        let vec = U4Vec16Builder::new()
            .set_item(0, 5)
            .set_item(1, 10)
            .set_item(15, 15)
            .build();

        assert_eq!(vec.item(0), 5);
        assert_eq!(vec.item(1), 10);
        assert_eq!(vec.item(15), 15);

        // Verify other items remain 0
        for i in 2..15 {
            assert_eq!(vec.item(i), 0);
        }
    }

    /// Tests that set_item can be chained multiple times
    #[test]
    fn test_builder_set_item_chaining() {
        let vec = U4Vec16Builder::new()
            .set_item(0, 1)
            .set_item(1, 2)
            .set_item(2, 3)
            .set_item(3, 4)
            .set_item(4, 5)
            .set_item(5, 6)
            .set_item(6, 7)
            .set_item(7, 8)
            .set_item(8, 9)
            .set_item(9, 10)
            .set_item(10, 11)
            .set_item(11, 12)
            .set_item(12, 13)
            .set_item(13, 14)
            .set_item(14, 15)
            .set_item(15, 0)
            .build();

        // Verify all items are set correctly
        for i in 0..15 {
            assert_eq!(vec.item(i), (i + 1) as u8);
        }
        assert_eq!(vec.item(15), 0);
    }

    /// Tests that set_item can overwrite previously set items
    #[test]
    fn test_builder_set_item_overwrite() {
        let vec = U4Vec16Builder::new()
            .set_item(0, 5)
            .set_item(0, 10) // Overwrite item 0
            .set_item(1, 15)
            .set_item(1, 3) // Overwrite item 1
            .build();

        assert_eq!(vec.item(0), 10);
        assert_eq!(vec.item(1), 3);

        // Verify other items remain 0
        for i in 2..16 {
            assert_eq!(vec.item(i), 0);
        }
    }

    /// Tests that set_item works with edge values (0 and 15)
    #[test]
    fn test_builder_set_item_edge_values() {
        let vec = U4Vec16Builder::new()
            .set_item(0, 0) // Minimum value
            .set_item(1, 15) // Maximum value
            .set_item(15, 0) // Minimum value at max index
            .build();

        assert_eq!(vec.item(0), 0);
        assert_eq!(vec.item(1), 15);
        assert_eq!(vec.item(15), 0);
    }

    /// Tests that set_item works with all possible 4-bit values
    #[test]
    fn test_builder_set_item_all_values() {
        let mut builder = U4Vec16Builder::new();

        // Set each item to its index value (0-15)
        for i in 0..16 {
            builder = builder.set_item(i, i as u8);
        }

        let vec = builder.build();

        // Verify all items are set correctly
        for i in 0..16 {
            assert_eq!(vec.item(i), i as u8);
        }
    }

    /// Tests that set_item works with all possible indices
    #[test]
    fn test_builder_set_item_all_indices() {
        let mut builder = U4Vec16Builder::new();

        // Set each index to value 5
        for i in 0..16 {
            builder = builder.set_item(i, 5);
        }

        let vec = builder.build();

        // Verify all items are set to 5
        for i in 0..16 {
            assert_eq!(vec.item(i), 5);
        }
    }

    /// Tests that build consumes the builder
    #[test]
    fn test_builder_build_consumes() {
        let builder = U4Vec16Builder::new().set_item(0, 5).set_item(1, 10);

        let vec = builder.build();
        assert_eq!(vec.item(0), 5);
        assert_eq!(vec.item(1), 10);

        // Builder is consumed, cannot be used again
        // This would not compile:
        // let _ = builder.build(); // Error: value used after move
    }

    /// Tests that the builder pattern produces the same result as direct construction
    #[test]
    fn test_builder_equivalent_to_direct() {
        // Build using builder pattern
        let builder_vec = U4Vec16Builder::new()
            .set_item(0, 5)
            .set_item(1, 10)
            .set_item(15, 15)
            .build();

        // Build using direct construction
        let direct_vec = U4Vec16::from_u64(0)
            .set_item(0, 5)
            .set_item(1, 10)
            .set_item(15, 15);

        // Both should produce the same result
        assert_eq!(builder_vec.inner(), direct_vec.inner());
        assert_eq!(builder_vec.item(0), direct_vec.item(0));
        assert_eq!(builder_vec.item(1), direct_vec.item(1));
        assert_eq!(builder_vec.item(15), direct_vec.item(15));
    }

    /// Tests that the builder pattern works with complex patterns
    #[test]
    fn test_builder_complex_pattern() {
        let vec = U4Vec16Builder::new()
            .set_item(0, 0xF) // 15
            .set_item(1, 0xE) // 14
            .set_item(2, 0xD) // 13
            .set_item(3, 0xC) // 12
            .set_item(4, 0xB) // 11
            .set_item(5, 0xA) // 10
            .set_item(6, 0x9) // 9
            .set_item(7, 0x8) // 8
            .set_item(8, 0x7) // 7
            .set_item(9, 0x6) // 6
            .set_item(10, 0x5) // 5
            .set_item(11, 0x4) // 4
            .set_item(12, 0x3) // 3
            .set_item(13, 0x2) // 2
            .set_item(14, 0x1) // 1
            .set_item(15, 0x0) // 0
            .build();

        // Verify the pattern
        for i in 0..16 {
            assert_eq!(vec.item(i), (15 - i) as u8);
        }
    }

    /// Tests that the builder pattern works with sparse patterns
    #[test]
    fn test_builder_sparse_pattern() {
        let vec = U4Vec16Builder::new()
            .set_item(0, 1)
            .set_item(7, 7)
            .set_item(15, 15)
            .build();

        assert_eq!(vec.item(0), 1);
        assert_eq!(vec.item(7), 7);
        assert_eq!(vec.item(15), 15);

        // Verify other items remain 0
        for i in 1..7 {
            assert_eq!(vec.item(i), 0);
        }
        for i in 8..15 {
            assert_eq!(vec.item(i), 0);
        }
    }
}
