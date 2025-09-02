//! Musical Scale Implementation
//!
//! This module provides a representation of musical scales using BitVec16.
//! Each bit position represents a semitone interval from the root note.
//! The scales are defined using standard Western music theory patterns.

use crate::{BitVec16, BitVec16Builder};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    /// Applies the scale to a root note
    ///
    /// This method applies the scale to a root note, returning an iterator
    /// over the notes in the scale.
    ///
    /// # Arguments
    /// * `root` - The root note to apply the scale to
    ///
    /// # Returns
    /// An iterator that yields u8 values representing the notes in the scale
    ///
    /// # Example
    /// ```
    /// use muzze_std::Scale;
    /// let scale = Scale::from_u16(0b0000_0000_0000_1111);
    /// let notes: Vec<u8> = scale.apply(4).collect();
    /// // For a major scale, this would yield [4, 6, 7, 9, 11, 12, 14, 16]
    /// ```
    #[inline]
    pub fn apply(&self, root: u8) -> impl Iterator<Item = u8> {
        std::iter::once(root).chain(self.intervals().map(move |interval| interval + root))
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

/// Pentatonic major scale: Whole-Whole-WholeHalf-Whole
pub const PENTATONIC_MAJOR: Scale = Scale::from_u16(0b0000_0001_0100_1010);

/// Pentatonic minor scale: WholeHalft-Whole-Whole-WholeHalf
pub const PENTATONIC_MINOR: Scale = Scale::from_u16(0b0000_0010_0101_0100);

/// Blues major scale: Whole-Half-Half-WholeHalf-Whole
pub const BLUES_MAJOR: Scale = Scale::from_u16(0b0000_0001_0100_1110);

/// Blues minor scale: WholeHalf-Whole-Half-Half-WholeHalf-Whole
pub const BLUES_MINOR: Scale = Scale::from_u16(0b0000_1010_0111_0100);

/// Jazz whole tone scale: Whole-Whole-Whole-Whole-Whole
pub const JAZZ_WHOLE_TONE: Scale = Scale::from_u16(0b0000_0010_1010_1010);

/// Jazz whole half diminished scale: Whole-Half-Whole-Half-Whole-Half-Whole
pub const JAZZ_WHOLEHALF_DIMINISHED: Scale = Scale::from_u16(0b0000_0101_1011_0110);

/// Bibop major scale: Whole-Whole-Half-Whole-Half-Half-Whole-Half
pub const BIBOP_MAJOR: Scale = Scale::from_u16(0b0000_1101_1101_1010);

/// Bibop minor scale: Whole-Half-Half-Half-Whole-Whole-Half-Whole
pub const BIBOP_MINOR: Scale = Scale::from_u16(0b0000_1011_0101_1110);

/// Bibop dominant scale: Whole-Whole-Half-Whole-Whole-Half-Half-Half
pub const BIBOP_DOMINANT: Scale = Scale::from_u16(0b0000_1111_0101_1010);

/// Builder for constructing Scale instances
///
/// The ScaleBuilder provides a fluent interface for constructing Scale
/// instances by setting individual semitone intervals. This is useful when
/// you need to build a scale programmatically or create custom scales
/// that aren't predefined as constants.
///
/// # Examples
/// ```
/// use muzze_std::ScaleBuilder;
/// let scale = ScaleBuilder::default()
///     .set_interval(2)  // Major 2nd
///     .set_interval(4)  // Major 3rd
///     .set_interval(5)  // Perfect 4th
///     .set_interval(7)  // Perfect 5th
///     .set_interval(9)  // Major 6th
///     .set_interval(11) // Major 7th
///     .set_interval(12) // Octave
///     .build();
/// // This creates a major scale
/// ```
pub struct ScaleBuilder {
    /// The underlying BitVec16Builder used for bit manipulation
    vec_builder: BitVec16Builder,
}

impl ScaleBuilder {
    /// Creates a new ScaleBuilder with no intervals initially set
    ///
    /// This method initializes the builder with an empty scale (no intervals set).
    /// The builder can then be used to add specific semitone intervals using
    /// the `set_interval` method.
    ///
    /// # Returns
    /// A new ScaleBuilder instance ready for configuration
    ///
    /// # Example
    /// ```
    /// use muzze_std::ScaleBuilder;
    /// let builder = ScaleBuilder::default();
    /// let scale = builder.build();
    /// assert_eq!(scale.intervals().count(), 0);
    /// ```
    #[inline]
    const fn new() -> Self {
        Self {
            vec_builder: BitVec16Builder::new(),
        }
    }

    /// Adds a semitone interval to the scale being constructed
    ///
    /// This method adds the specified semitone interval to the scale.
    /// The interval parameter represents the number of semitones from the root note.
    /// For example, interval 2 represents a major 2nd, interval 12 represents an octave.
    ///
    /// # Arguments
    /// * `interval` - The semitone interval to add (1-16)
    ///
    /// # Returns
    /// A new ScaleBuilder with the specified interval added
    ///
    /// # Panics
    /// This method will panic if the interval is out of bounds (< 1 or > 16)
    ///
    /// # Example
    /// ```
    /// use muzze_std::ScaleBuilder;
    /// let scale = ScaleBuilder::default()
    ///     .set_interval(2)  // Add major 2nd
    ///     .set_interval(7)  // Add perfect 5th
    ///     .set_interval(12) // Add octave
    ///     .build();
    /// let intervals: Vec<u8> = scale.intervals().collect();
    /// assert_eq!(intervals, vec![2, 7, 12]);
    /// ```
    #[inline]
    pub const fn set_interval(self, interval: u8) -> Self {
        let vec_builder = self.vec_builder.set_index(interval - 1);
        Self { vec_builder }
    }

    /// Finalizes the builder and returns the constructed Scale
    ///
    /// This method consumes the builder and returns the final Scale
    /// instance with all the intervals that were added during construction.
    ///
    /// # Returns
    /// The constructed Scale instance
    ///
    /// # Example
    /// ```
    /// use muzze_std::{MAJOR, ScaleBuilder};
    /// let scale = ScaleBuilder::default()
    ///     .set_interval(2)
    ///     .set_interval(4)
    ///     .set_interval(5)
    ///     .set_interval(7)
    ///     .set_interval(9)
    ///     .set_interval(11)
    ///     .set_interval(12)
    ///     .build();
    /// // This creates a major scale with intervals [2, 4, 5, 7, 9, 11, 12]
    /// assert_eq!(scale, MAJOR);
    /// ```
    #[inline]
    pub const fn build(self) -> Scale {
        Scale::from_u16(self.vec_builder.build().inner())
    }
}

impl Default for ScaleBuilder {
    /// Creates a default ScaleBuilder instance
    ///
    /// This implementation provides a convenient way to create a new builder
    /// using the `Default` trait, which is equivalent to calling `ScaleBuilder::new()`.
    ///
    /// # Returns
    /// A new ScaleBuilder instance with no intervals initially set
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing Scale instances using step intervals
///
/// The ScaleStepBuilder provides a fluent interface for constructing Scale
/// instances by adding consecutive step intervals. This is useful when you
/// want to build a scale by specifying the step pattern (e.g., whole steps,
/// half steps) rather than absolute semitone intervals.
///
/// # Examples
/// ```
/// use muzze_std::{MAJOR, ScaleStepBuilder};
/// let scale = ScaleStepBuilder::default()
///     .add_step(2)  // Whole step
///     .add_step(2)  // Whole step
///     .add_step(1)  // Half step
///     .add_step(2)  // Whole step
///     .add_step(2)  // Whole step
///     .add_step(2)  // Whole step
///     .add_step(1)  // Half step
///     .build();
/// // This creates a major scale with step pattern [2, 2, 1, 2, 2, 2, 1]
/// assert_eq!(scale, MAJOR);
/// ```
pub struct ScaleStepBuilder {
    /// The underlying ScaleBuilder used for bit manipulation
    scale_builder: ScaleBuilder,
    /// The last interval that was added to track cumulative position
    last_interval: u8,
}

impl ScaleStepBuilder {
    /// Creates a new ScaleStepBuilder with no intervals initially set
    ///
    /// This method initializes the builder with an empty scale (no intervals set)
    /// and resets the interval tracking. The builder can then be used to add
    /// step intervals using the `add_step` method.
    ///
    /// # Returns
    /// A new ScaleStepBuilder instance ready for configuration
    #[inline]
    const fn new() -> Self {
        Self {
            scale_builder: ScaleBuilder::new(),
            last_interval: 0,
        }
    }

    /// Adds a step interval to the scale being constructed
    ///
    /// This method adds a step interval to the scale, where the step represents
    /// the number of semitones to move from the last added interval. This allows
    /// building scales by specifying step patterns (e.g., whole steps = 2,
    /// half steps = 1) rather than absolute semitone positions.
    ///
    /// # Arguments
    /// * `step` - The number of semitones to add from the last interval (1-16)
    ///
    /// # Returns
    /// A new ScaleStepBuilder with the specified step interval added
    ///
    /// # Panics
    /// This method will panic if the resulting interval exceeds 16 semitones
    ///
    /// # Example
    /// ```
    /// use muzze_std::ScaleStepBuilder;
    /// let scale = ScaleStepBuilder::default()
    ///     .add_step(2)  // Add whole step (major 2nd)
    ///     .add_step(2)  // Add whole step (major 3rd)
    ///     .add_step(1)  // Add half step (perfect 4th)
    ///     .build();
    /// let intervals: Vec<u8> = scale.intervals().collect();
    /// assert_eq!(intervals, vec![2, 4, 5]);
    /// ```
    pub const fn add_step(self, step: u8) -> Self {
        let interval = self.last_interval + step;
        let last_interval = interval;
        Self {
            scale_builder: self.scale_builder.set_interval(last_interval),
            last_interval,
        }
    }

    /// Finalizes the builder and returns the constructed Scale
    ///
    /// This method consumes the builder and returns the final Scale
    /// instance with all the step intervals that were added during construction.
    ///
    /// # Returns
    /// The constructed Scale instance
    ///
    /// # Example
    /// ```
    /// use muzze_std::{MAJOR, ScaleStepBuilder};
    /// let scale = ScaleStepBuilder::default()
    ///     .add_step(2)
    ///     .add_step(2)
    ///     .add_step(1)
    ///     .add_step(2)
    ///     .add_step(2)
    ///     .add_step(2)
    ///     .add_step(1)
    ///     .build();
    /// // This creates a major scale with intervals [2, 4, 5, 7, 9, 11, 12]
    /// assert_eq!(scale, MAJOR);
    /// ```
    #[inline]
    pub const fn build(self) -> Scale {
        self.scale_builder.build()
    }
}

impl Default for ScaleStepBuilder {
    /// Creates a default ScaleStepBuilder instance
    ///
    /// This implementation provides a convenient way to create a new builder
    /// using the `Default` trait, which is equivalent to calling `ScaleStepBuilder::new()`.
    ///
    /// # Returns
    /// A new ScaleStepBuilder instance with no intervals initially set
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const C: u8 = 60;
    // const CS: u8 = 61;
    const D: u8 = 62;
    // const DS: u8 = 63;
    const EF: u8 = 63;
    const E: u8 = 64;
    const F: u8 = 65;
    const FS: u8 = 66;
    // const FS: u8 = 66;
    const GF: u8 = 66;
    const G: u8 = 67;
    const GS: u8 = 68;
    const AF: u8 = 68;
    const A: u8 = 69;
    const AS: u8 = 70;
    const BF: u8 = 70;
    const B: u8 = 71;
    const C1: u8 = 72;

    /// Tests that the major scale produces the correct intervals and step pattern
    ///
    /// This test verifies that the major scale constant correctly represents
    /// the standard major scale with intervals [2, 4, 5, 7, 9, 11, 12] semitones
    /// and step pattern [2, 2, 1, 2, 2, 2, 1].
    #[test]
    fn test_major_scale() {
        assert_eq!(
            MAJOR.intervals().collect::<Vec<u8>>(),
            vec![2, 4, 5, 7, 9, 11, 12]
        );
        assert_eq!(
            MAJOR.steps().collect::<Vec<u8>>(),
            vec![2, 2, 1, 2, 2, 2, 1]
        );

        assert_eq!(
            MAJOR.apply(C).collect::<Vec<u8>>(),
            vec![C, D, E, F, G, A, B, C1]
        );
    }

    /// Tests that the natural minor scale produces the correct intervals and step pattern
    ///
    /// This test verifies that the natural minor scale constant correctly represents
    /// the standard natural minor scale with intervals [2, 3, 5, 7, 8, 10, 12] semitones
    /// and step pattern [2, 1, 2, 2, 1, 2, 2].
    #[test]
    fn test_natural_minor_scale() {
        assert_eq!(
            NATURAL_MINOR.intervals().collect::<Vec<u8>>(),
            vec![2, 3, 5, 7, 8, 10, 12]
        );
        assert_eq!(
            NATURAL_MINOR.steps().collect::<Vec<u8>>(),
            vec![2, 1, 2, 2, 1, 2, 2]
        );

        assert_eq!(
            NATURAL_MINOR.apply(C).collect::<Vec<u8>>(),
            vec![C, D, EF, F, G, AF, BF, C1]
        );
    }

    /// Tests that the harmonic minor scale produces the correct intervals and step pattern
    ///
    /// This test verifies that the harmonic minor scale constant correctly represents
    /// the standard harmonic minor scale with intervals [2, 3, 5, 7, 8, 11, 12] semitones
    /// and step pattern [2, 1, 2, 2, 1, 3, 1], including the characteristic augmented 2nd.
    #[test]
    fn test_harmonic_minor_scale() {
        assert_eq!(
            HARMONIC_MINOR.intervals().collect::<Vec<u8>>(),
            vec![2, 3, 5, 7, 8, 11, 12]
        );
        assert_eq!(
            HARMONIC_MINOR.steps().collect::<Vec<u8>>(),
            vec![2, 1, 2, 2, 1, 3, 1]
        );

        assert_eq!(
            HARMONIC_MINOR.apply(C).collect::<Vec<u8>>(),
            vec![C, D, EF, F, G, AF, B, C1]
        );
    }

    /// Tests that the melodic minor scale produces the correct intervals and step pattern
    ///
    /// This test verifies that the melodic minor scale constant correctly represents
    /// the ascending melodic minor scale with intervals [2, 3, 5, 7, 9, 11, 12] semitones
    /// and step pattern [2, 1, 2, 2, 2, 2, 1], avoiding the augmented 2nd.
    #[test]
    fn test_melodic_minor_scale() {
        assert_eq!(
            MELODIC_MINOR.intervals().collect::<Vec<u8>>(),
            vec![2, 3, 5, 7, 9, 11, 12]
        );
        assert_eq!(
            MELODIC_MINOR.steps().collect::<Vec<u8>>(),
            vec![2, 1, 2, 2, 2, 2, 1]
        );

        assert_eq!(
            MELODIC_MINOR.apply(C).collect::<Vec<u8>>(),
            vec![C, D, EF, F, G, A, B, C1]
        );
    }

    #[test]
    fn test_pentatonic_major_scale() {
        assert_eq!(
            PENTATONIC_MAJOR.intervals().collect::<Vec<u8>>(),
            vec![2, 4, 7, 9]
        );
        assert_eq!(
            PENTATONIC_MAJOR.steps().collect::<Vec<u8>>(),
            vec![2, 2, 3, 2]
        );

        assert_eq!(
            PENTATONIC_MAJOR.apply(C).collect::<Vec<u8>>(),
            vec![C, D, E, G, A]
        );
    }

    #[test]
    fn test_pentatonic_minor_scale() {
        assert_eq!(
            PENTATONIC_MINOR.intervals().collect::<Vec<u8>>(),
            vec![3, 5, 7, 10]
        );
        assert_eq!(
            PENTATONIC_MINOR.steps().collect::<Vec<u8>>(),
            vec![3, 2, 2, 3]
        );

        assert_eq!(
            PENTATONIC_MINOR.apply(C).collect::<Vec<u8>>(),
            vec![C, EF, F, G, BF]
        );
    }

    #[test]
    fn test_blues_minor_scale() {
        assert_eq!(
            BLUES_MINOR.intervals().collect::<Vec<u8>>(),
            vec![3, 5, 6, 7, 10, 12]
        );
        assert_eq!(
            BLUES_MINOR.steps().collect::<Vec<u8>>(),
            vec![3, 2, 1, 1, 3, 2]
        );

        assert_eq!(
            BLUES_MINOR.apply(C).collect::<Vec<u8>>(),
            vec![C, EF, F, GF, G, BF, C1]
        );
    }

    #[test]
    fn test_blues_major_scale() {
        assert_eq!(
            BLUES_MAJOR.intervals().collect::<Vec<u8>>(),
            vec![2, 3, 4, 7, 9]
        );
        assert_eq!(
            BLUES_MAJOR.steps().collect::<Vec<u8>>(),
            vec![2, 1, 1, 3, 2]
        );
        assert_eq!(
            BLUES_MAJOR.apply(C).collect::<Vec<u8>>(),
            vec![C, D, EF, E, G, A]
        );
    }

    #[test]
    fn test_jazz_whole_tone_scale() {
        assert_eq!(
            JAZZ_WHOLE_TONE.intervals().collect::<Vec<u8>>(),
            vec![2, 4, 6, 8, 10]
        );
        assert_eq!(
            JAZZ_WHOLE_TONE.steps().collect::<Vec<u8>>(),
            vec![2, 2, 2, 2, 2]
        );
        assert_eq!(
            JAZZ_WHOLE_TONE.apply(C).collect::<Vec<u8>>(),
            vec![C, D, E, FS, GS, AS]
        );
    }

    #[test]
    fn test_jazz_wholehalf_diminished_scale() {
        assert_eq!(
            JAZZ_WHOLEHALF_DIMINISHED.intervals().collect::<Vec<u8>>(),
            vec![2, 3, 5, 6, 8, 9, 11]
        );
        assert_eq!(
            JAZZ_WHOLEHALF_DIMINISHED.steps().collect::<Vec<u8>>(),
            vec![2, 1, 2, 1, 2, 1, 2]
        );
        assert_eq!(
            JAZZ_WHOLEHALF_DIMINISHED.apply(C).collect::<Vec<u8>>(),
            vec![C, D, EF, F, GF, AF, A, B]
        );
    }

    #[test]
    fn test_bibop_major_scale() {
        assert_eq!(
            BIBOP_MAJOR.intervals().collect::<Vec<u8>>(),
            vec![2, 4, 5, 7, 8, 9, 11, 12]
        );
        assert_eq!(
            BIBOP_MAJOR.steps().collect::<Vec<u8>>(),
            vec![2, 2, 1, 2, 1, 1, 2, 1]
        );
        assert_eq!(
            BIBOP_MAJOR.apply(C).collect::<Vec<u8>>(),
            vec![C, D, E, F, G, AF, A, B, C1]
        );
    }

    #[test]
    fn test_bibop_minor_scale() {
        assert_eq!(
            BIBOP_MINOR.intervals().collect::<Vec<u8>>(),
            vec![2, 3, 4, 5, 7, 9, 10, 12]
        );
        assert_eq!(
            BIBOP_MINOR.steps().collect::<Vec<u8>>(),
            vec![2, 1, 1, 1, 2, 2, 1, 2]
        );
        assert_eq!(
            BIBOP_MINOR.apply(C).collect::<Vec<u8>>(),
            vec![C, D, EF, E, F, G, A, BF, C1]
        );
    }

    #[test]
    fn test_bibop_dominant_scale() {
        assert_eq!(
            BIBOP_DOMINANT.intervals().collect::<Vec<u8>>(),
            vec![2, 4, 5, 7, 9, 10, 11, 12]
        );
        assert_eq!(
            BIBOP_DOMINANT.steps().collect::<Vec<u8>>(),
            vec![2, 2, 1, 2, 2, 1, 1, 1]
        );
        assert_eq!(
            BIBOP_DOMINANT.apply(C).collect::<Vec<u8>>(),
            vec![C, D, E, F, G, A, BF, B, C1]
        );
    }

    /// Tests that the ScaleBuilder correctly constructs Scale instances
    ///
    /// This test verifies that the builder pattern works correctly by setting
    /// multiple semitone intervals and ensuring the final result matches the
    /// expected major scale intervals.
    #[test]
    fn test_scale_builder() {
        let scale = ScaleBuilder::default()
            .set_interval(2)
            .set_interval(4)
            .set_interval(5)
            .set_interval(7)
            .set_interval(9)
            .set_interval(11)
            .set_interval(12)
            .build();
        assert_eq!(
            scale.intervals().collect::<Vec<u8>>(),
            vec![2, 4, 5, 7, 9, 11, 12]
        );
        assert_eq!(scale, MAJOR);
    }

    #[test]
    fn test_scale_step_builder() {
        let scale = ScaleStepBuilder::default()
            .add_step(2)
            .add_step(2)
            .add_step(1)
            .add_step(2)
            .add_step(2)
            .add_step(2)
            .add_step(1)
            .build();

        assert_eq!(
            scale.intervals().collect::<Vec<u8>>(),
            vec![2, 4, 5, 7, 9, 11, 12]
        );
        assert_eq!(scale, MAJOR);
    }
}
