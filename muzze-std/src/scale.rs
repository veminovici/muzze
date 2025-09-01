//! Musical Scale Implementation
//!
//! This module provides a representation of musical scales using BitVec16.
//! Each bit position represents a semitone interval from the root note.
//! The scales are defined using standard Western music theory patterns.

use crate::BitVec16;

/// Represents a musical scale using a 16-bit vector
///
/// Each bit position (0-15) represents a semitone interval from the root note.
/// Bit 0 represents the root (unison), bit 1 represents a minor 2nd, etc.
/// The scale pattern is stored as a BitVec16 where set bits indicate
/// which semitone intervals are part of the scale.
///
/// # Examples
/// ```
/// use muzze_std::Scale;
/// let major_scale = Scale::from_u16(0b0000_0000_0000_1111);
/// let intervals: Vec<u8> = major_scale.intervals().collect();
/// ```
pub struct Scale(BitVec16);

impl Scale {
    /// Creates a new Scale from a u16 value
    ///
    /// The u16 value represents the bit pattern where each bit position
    /// corresponds to a semitone interval from the root note.
    ///
    /// # Arguments
    /// * `value` - The u16 value representing the scale pattern
    ///
    /// # Returns
    /// A new Scale instance with the specified bit pattern
    ///
    /// # Example
    /// ```
    /// use muzze_std::Scale;
    /// let scale = Scale::from_u16(0b0000_0000_0000_1111);
    /// ```
    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        Self(BitVec16::from_u16(value))
    }

    /// Returns an iterator over the semitone intervals in this scale
    ///
    /// This method yields the semitone intervals (1-16) that are part of the scale.
    /// The intervals are returned in ascending order, starting from the root.
    ///
    /// # Returns
    /// An iterator that yields u8 values representing semitone intervals
    ///
    /// # Example
    /// ```
    /// use muzze_std::Scale;
    /// let scale = Scale::from_u16(0b0000_0000_0000_1111);
    /// let intervals: Vec<u8> = scale.intervals().collect();
    /// // For a major scale, this would yield [1, 2, 3, 4, 5, 6, 7, 8]
    /// ```
    #[inline]
    pub fn intervals(&self) -> impl Iterator<Item = u8> {
        self.0.indeces_on().map(|i| (i + 1) as u8)
    }

    /// Returns an iterator over the step intervals between consecutive scale degrees
    ///
    /// This method calculates the semitone differences between consecutive
    /// intervals in the scale, representing the step pattern of the scale.
    /// For example, a major scale has steps: [2, 2, 1, 2, 2, 2, 1].
    ///
    /// # Returns
    /// An iterator that yields u8 values representing step intervals
    ///
    /// # Example
    /// ```
    /// use muzze_std::Scale;
    /// let scale = Scale::from_u16(0b0000_0000_0000_1111);
    /// let steps: Vec<u8> = scale.steps().collect();
    /// // For a major scale, this would yield [2, 2, 1, 2, 2, 2, 1]
    /// ```
    #[inline]
    pub fn steps(&self) -> impl Iterator<Item = u8> {
        let mut last = 0;
        self.intervals().map(move |interval| {
            let step = interval - last;
            last = interval;
            step
        })
    }
}

/// Major scale: Whole-Whole-Half-Whole-Whole-Whole-Half
///
/// The major scale is one of the most fundamental scales in Western music.
/// It follows the pattern: W-W-H-W-W-W-H (where W = whole step, H = half step).
/// This creates a bright, happy sound characteristic of major keys.
///
/// # Musical Theory
/// The major scale contains the intervals: 2nd, 3rd, 4th, 5th, 6th, 7th, octave
/// from the root note, following the step pattern [2, 2, 1, 2, 2, 2, 1].
///
/// # Bit Pattern
/// The bit pattern `0b0000_1101_0101_1010` represents the semitone intervals
/// that are part of the major scale structure.
pub const MAJOR: Scale = Scale::from_u16(0b0000_1101_0101_1010);

/// Natural minor scale: Whole-Half-Whole-Whole-Half-Whole-Whole
///
/// The natural minor scale is the relative minor of the major scale.
/// It follows the pattern: W-H-W-W-H-W-W, creating a darker, more somber sound.
///
/// # Musical Theory
/// The natural minor scale contains the intervals: 2nd, 3rd, 4th, 5th, 6th, 7th, octave
/// from the root note, following the step pattern [2, 1, 2, 2, 1, 2, 2].
///
/// # Bit Pattern
/// The bit pattern `0b0000_1010_1101_0110` represents the semitone intervals
/// that are part of the natural minor scale structure.
pub const NATURAL_MINOR: Scale = Scale::from_u16(0b0000_1010_1101_0110);

/// Harmonic minor scale: Whole-Half-Whole-Whole-Half, WholeHalf, Half
///
/// The harmonic minor scale is a variation of the natural minor scale
/// with a raised 7th degree. This creates a stronger leading tone resolution.
///
/// # Musical Theory
/// The harmonic minor scale follows the step pattern [2, 1, 2, 2, 1, 3, 1],
/// where the 6th to 7th degree is an augmented 2nd (3 semitones).
///
/// # Bit Pattern
/// The bit pattern `0b0000_1100_1101_0110` represents the semitone intervals
/// that are part of the harmonic minor scale structure.
pub const HARMONIC_MINOR: Scale = Scale::from_u16(0b0000_1100_1101_0110);

/// Melodic minor scale: Whole-Half-Whole-Whole-Whole-Whole-Half
///
/// The melodic minor scale has different ascending and descending forms.
/// The ascending form raises both the 6th and 7th degrees, while the
/// descending form is identical to the natural minor scale.
///
/// # Musical Theory
/// The ascending melodic minor follows the step pattern [2, 1, 2, 2, 2, 2, 1],
/// creating a smooth melodic line without the augmented 2nd of the harmonic minor.
///
/// # Bit Pattern
/// The bit pattern `0b0000_1101_0101_0110` represents the semitone intervals
/// that are part of the melodic minor scale structure.
pub const MELODIC_MINOR: Scale = Scale::from_u16(0b0000_1101_0101_0110);

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the major scale produces the correct intervals and step pattern
    ///
    /// This test verifies that the major scale constant correctly represents
    /// the standard major scale with intervals [2, 4, 5, 7, 9, 11, 12] semitones
    /// and step pattern [2, 2, 1, 2, 2, 2, 1].
    #[test]
    fn test_major_scale() {
        assert_eq!(MAJOR.intervals().collect::<Vec<u8>>(), vec![2, 4, 5, 7, 9, 11, 12]);
        assert_eq!(MAJOR.steps().collect::<Vec<u8>>(), vec![2, 2, 1, 2, 2, 2, 1]);
    }

    /// Tests that the natural minor scale produces the correct intervals and step pattern
    ///
    /// This test verifies that the natural minor scale constant correctly represents
    /// the standard natural minor scale with intervals [2, 3, 5, 7, 8, 10, 12] semitones
    /// and step pattern [2, 1, 2, 2, 1, 2, 2].
    #[test]
    fn test_natural_minor_scale() {
        assert_eq!(NATURAL_MINOR.intervals().collect::<Vec<u8>>(), vec![2, 3, 5, 7, 8, 10, 12]);
        assert_eq!(NATURAL_MINOR.steps().collect::<Vec<u8>>(), vec![2, 1, 2, 2, 1, 2, 2]);
    }

    /// Tests that the harmonic minor scale produces the correct intervals and step pattern
    ///
    /// This test verifies that the harmonic minor scale constant correctly represents
    /// the standard harmonic minor scale with intervals [2, 3, 5, 7, 8, 11, 12] semitones
    /// and step pattern [2, 1, 2, 2, 1, 3, 1], including the characteristic augmented 2nd.
    #[test]
    fn test_harmonic_minor_scale() {
        assert_eq!(HARMONIC_MINOR.intervals().collect::<Vec<u8>>(), vec![2, 3, 5, 7, 8, 11, 12]);
        assert_eq!(HARMONIC_MINOR.steps().collect::<Vec<u8>>(), vec![2, 1, 2, 2, 1, 3, 1]);
    }

    /// Tests that the melodic minor scale produces the correct intervals and step pattern
    ///
    /// This test verifies that the melodic minor scale constant correctly represents
    /// the ascending melodic minor scale with intervals [2, 3, 5, 7, 9, 11, 12] semitones
    /// and step pattern [2, 1, 2, 2, 2, 2, 1], avoiding the augmented 2nd.
    #[test]
    fn test_melodic_minor_scale() {
        assert_eq!(MELODIC_MINOR.intervals().collect::<Vec<u8>>(), vec![2, 3, 5, 7, 9, 11, 12]);
        assert_eq!(MELODIC_MINOR.steps().collect::<Vec<u8>>(), vec![2, 1, 2, 2, 2, 2, 1]);
    }
}