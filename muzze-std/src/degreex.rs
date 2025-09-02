//! Musical Degree with Accidentals
//!
//! This module provides a representation of musical degrees (scale degrees) with
//! accidentals (sharps, flats, double sharps, double flats). The degree is stored
//! in the lower 4 bits and the accidental type is stored in the upper 4 bits
//! using the U4x2 packed representation.

use crate::U4x2;

/// A musical degree with accidentals
///
/// Degreex represents a musical scale degree (1-7) combined with an accidental
/// type (0=natural, 1=flat, 2=double flat, 3=sharp, 4=double sharp). This provides
/// a compact way to represent chord tones, scale degrees, and other musical
/// intervals with their specific alterations.
///
/// # Examples
/// ```
/// use muzze_std::{THIRD, THIRD_FLAT, THIRD_SHARP};
///
/// // Natural third
/// assert_eq!(THIRD.first(), 3);  // Third degree
/// assert_eq!(THIRD.second(), 0); // Natural (no accidental)
///
/// // Flattened third
/// assert_eq!(THIRD_FLAT.first(), 3);  // Third degree
/// assert_eq!(THIRD_FLAT.second(), 1); // Flat accidental
///
/// // Sharpened third
/// assert_eq!(THIRD_SHARP.first(), 3);  // Third degree
/// assert_eq!(THIRD_SHARP.second(), 3); // Sharp accidental
/// ```
pub struct Degreex {
    /// The underlying U4x2 containing the degree and accidental information
    inner: U4x2,
}

impl Degreex {
    /// Creates a new Degreex with the specified degree and accidental
    ///
    /// This method creates a new Degreex instance by packing the degree and
    /// accidental information into a U4x2 structure.
    ///
    /// # Arguments
    /// * `first` - The scale degree (1-7, where 1=root, 2=second, 3=third, etc.)
    /// * `second` - The accidental type (0=natural, 1=flat, 2=double flat, 3=sharp, 4=double sharp)
    ///
    /// # Returns
    /// A new Degreex instance with the specified degree and accidental
    ///
    /// # Panics
    /// This method will panic if either value exceeds 15 (4-bit limit)
    ///
    /// # Example
    /// ```
    /// use muzze_std::Degreex;
    /// let degree = Degreex::new(5, 1);  // Fifth degree with flat accidental
    /// assert_eq!(degree.first(), 5);    // Fifth degree
    /// assert_eq!(degree.second(), 1);   // Flat accidental
    /// ```
    pub const fn new(first: u8, second: u8) -> Self {
        Self {
            inner: U4x2::new(first, second),
        }
    }

    /// Returns the underlying u8 value containing both degree and accidental
    ///
    /// This method provides access to the raw u8 representation of the packed
    /// degree and accidental information, which can be useful for serialization
    /// or bit manipulation.
    ///
    /// # Returns
    /// The u8 value containing both the degree and accidental information
    ///
    /// # Example
    /// ```
    /// use muzze_std::THIRD_FLAT;
    /// assert_eq!(THIRD_FLAT.inner(), 0b0001_0011);  // 1 << 4 | 3
    /// ```
    pub const fn inner(&self) -> u8 {
        self.inner.inner()
    }

    /// Returns the scale degree (1-7)
    ///
    /// This method extracts the scale degree from the packed representation.
    /// The degree represents the position in the scale (1=root, 2=second, 3=third, etc.).
    ///
    /// # Returns
    /// The scale degree (1-7)
    ///
    /// # Example
    /// ```
    /// use muzze_std::{THIRD, FIFTH, SEVENTH};
    /// assert_eq!(THIRD.first(), 3);   // Third degree
    /// assert_eq!(FIFTH.first(), 5);   // Fifth degree
    /// assert_eq!(SEVENTH.first(), 7); // Seventh degree
    /// ```
    pub const fn first(&self) -> u8 {
        self.inner.first()
    }

    /// Returns the accidental type
    ///
    /// This method extracts the accidental type from the packed representation.
    /// The accidental indicates how the degree is altered from its natural form.
    ///
    /// # Returns
    /// The accidental type:
    /// - 0 = Natural (no accidental)
    /// - 1 = Flat (♭)
    /// - 2 = Double flat (♭♭)
    /// - 3 = Sharp (♯)
    /// - 4 = Double sharp (♯♯)
    ///
    /// # Example
    /// ```
    /// use muzze_std::{THIRD, THIRD_FLAT, THIRD_SHARP, THIRD_DOUBLESHARP};
    /// assert_eq!(THIRD.second(), 0);           // Natural
    /// assert_eq!(THIRD_FLAT.second(), 1);      // Flat
    /// assert_eq!(THIRD_SHARP.second(), 3);     // Sharp
    /// assert_eq!(THIRD_DOUBLESHARP.second(), 4); // Double sharp
    /// ```
    pub const fn second(&self) -> u8 {
        self.inner.second()
    }
}

// Third degree variations
/// Natural third degree (major third)
pub const THIRD: Degreex = Degreex::new(3, 0);
/// Flattened third degree (minor third)
pub const THIRD_FLAT: Degreex = Degreex::new(3, 1);
/// Double flattened third degree (diminished third)
pub const THIRD_DOUBLEFLAT: Degreex = Degreex::new(3, 2);
/// Sharpened third degree (augmented third)
pub const THIRD_SHARP: Degreex = Degreex::new(3, 3);
/// Double sharpened third degree (doubly augmented third)
pub const THIRD_DOUBLESHARP: Degreex = Degreex::new(3, 4);

// Fifth degree variations
/// Natural fifth degree (perfect fifth)
pub const FIFTH: Degreex = Degreex::new(5, 0);
/// Flattened fifth degree (diminished fifth/tritone)
pub const FIFTH_FLAT: Degreex = Degreex::new(5, 1);
/// Double flattened fifth degree (doubly diminished fifth)
pub const FIFTH_DOUBLEFLAT: Degreex = Degreex::new(5, 2);
/// Sharpened fifth degree (augmented fifth)
pub const FIFTH_SHARP: Degreex = Degreex::new(5, 3);
/// Double sharpened fifth degree (doubly augmented fifth)
pub const FIFTH_DOUBLESHARP: Degreex = Degreex::new(5, 4);

// Seventh degree variations
/// Natural seventh degree (major seventh)
pub const SEVENTH: Degreex = Degreex::new(7, 0);
/// Flattened seventh degree (minor seventh)
pub const SEVENTH_FLAT: Degreex = Degreex::new(7, 1);
/// Double flattened seventh degree (diminished seventh)
pub const SEVENTH_DOUBLEFLAT: Degreex = Degreex::new(7, 2);
/// Sharpened seventh degree (augmented seventh)
pub const SEVENTH_SHARP: Degreex = Degreex::new(7, 3);
/// Double sharpened seventh degree (doubly augmented seventh)
pub const SEVENTH_DOUBLESHARP: Degreex = Degreex::new(7, 4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degreex() {
        assert_eq!(THIRD.inner(), 0b0000_0011);
        assert_eq!(THIRD_FLAT.inner(), 0b0001_0011);
        assert_eq!(THIRD_DOUBLEFLAT.inner(), 0b0010_0011);
        assert_eq!(THIRD_SHARP.inner(), 0b0011_0011);
        assert_eq!(THIRD_DOUBLESHARP.inner(), 0b0100_0011);

        assert_eq!(FIFTH.inner(), 0b0000_0101);
        assert_eq!(FIFTH_FLAT.inner(), 0b0001_0101);
        assert_eq!(FIFTH_DOUBLEFLAT.inner(), 0b0010_0101);
        assert_eq!(FIFTH_SHARP.inner(), 0b0011_0101);
        assert_eq!(FIFTH_DOUBLESHARP.inner(), 0b0100_0101);

        assert_eq!(SEVENTH.inner(), 0b0000_0111);
        assert_eq!(SEVENTH_FLAT.inner(), 0b0001_0111);
        assert_eq!(SEVENTH_DOUBLEFLAT.inner(), 0b0010_0111);
        assert_eq!(SEVENTH_SHARP.inner(), 0b0011_0111);
        assert_eq!(SEVENTH_DOUBLESHARP.inner(), 0b0100_0111);
    }

    #[test]
    fn test_degreex_accessors() {
        assert_eq!(THIRD.first(), 3);
        assert_eq!(THIRD.second(), 0);
        assert_eq!(THIRD_FLAT.first(), 3);
        assert_eq!(THIRD_FLAT.second(), 1);
        assert_eq!(THIRD_DOUBLEFLAT.first(), 3);
        assert_eq!(THIRD_DOUBLEFLAT.second(), 2);
        assert_eq!(THIRD_SHARP.first(), 3);
        assert_eq!(THIRD_SHARP.second(), 3);
        assert_eq!(THIRD_DOUBLESHARP.first(), 3);
        assert_eq!(THIRD_DOUBLESHARP.second(), 4);

        assert_eq!(FIFTH.first(), 5);
        assert_eq!(FIFTH.second(), 0);
        assert_eq!(FIFTH_FLAT.first(), 5);
        assert_eq!(FIFTH_FLAT.second(), 1);
        assert_eq!(FIFTH_DOUBLEFLAT.first(), 5);
        assert_eq!(FIFTH_DOUBLEFLAT.second(), 2);
        assert_eq!(FIFTH_SHARP.first(), 5);
        assert_eq!(FIFTH_SHARP.second(), 3);
        assert_eq!(FIFTH_DOUBLESHARP.first(), 5);
        assert_eq!(FIFTH_DOUBLESHARP.second(), 4);

        assert_eq!(SEVENTH.first(), 7);
        assert_eq!(SEVENTH.second(), 0);
        assert_eq!(SEVENTH_FLAT.first(), 7);
        assert_eq!(SEVENTH_FLAT.second(), 1);
        assert_eq!(SEVENTH_DOUBLEFLAT.first(), 7);
        assert_eq!(SEVENTH_DOUBLEFLAT.second(), 2);
        assert_eq!(SEVENTH_SHARP.first(), 7);
        assert_eq!(SEVENTH_SHARP.second(), 3);
        assert_eq!(SEVENTH_DOUBLESHARP.first(), 7);
        assert_eq!(SEVENTH_DOUBLESHARP.second(), 4);
    }
}
