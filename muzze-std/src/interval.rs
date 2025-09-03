//! Musical Interval Types
//!
//! This module provides the `Interval` struct for representing musical intervals
//! in semitones with their corresponding names and display representations.

use std::fmt::Display;

use crate::Step;

/// Represents a musical interval in semitones
///
/// An `Interval` represents the distance between two notes in semitones. This struct
/// provides a type-safe way to represent common musical intervals with their
/// corresponding semitone values and display representations.
///
/// # Examples
///
/// ```rust
/// use muzze_std::{Interval, MAJOR_THIRD, PERFECT_FIFTH, OCTAVE};
///
/// // Using predefined constants
/// let major_third = MAJOR_THIRD;
/// assert_eq!(major_third.to_string(), "M3");
/// assert_eq!(u8::from(major_third), 4);
///
/// // Creating custom intervals
/// let custom_interval = Interval::from(15);
/// assert_eq!(custom_interval.to_string(), "I15");
/// ```
///
/// # Semitone Values
///
/// Common intervals and their semitone values:
/// - Unison: 0 semitones
/// - Minor 2nd: 1 semitone
/// - Major 2nd: 2 semitones
/// - Minor 3rd: 3 semitones
/// - Major 3rd: 4 semitones
/// - Perfect 4th: 5 semitones
/// - Augmented 4th/Diminished 5th: 6 semitones
/// - Perfect 5th: 7 semitones
/// - Minor 6th: 8 semitones
/// - Major 6th: 9 semitones
/// - Minor 7th: 10 semitones
/// - Major 7th: 11 semitones
/// - Octave: 12 semitones
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval(u8);

/// Unison interval constant - 0 semitones
///
/// This represents the same note, with no pitch difference.
/// It's equivalent to `Interval(0)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::UNISON;
///
/// assert_eq!(UNISON.to_string(), "P1");
/// assert_eq!(u8::from(UNISON), 0);
/// ```
pub const UNISON: Interval = Interval(0);

/// Minor second interval constant - 1 semitone
///
/// This represents a minor second interval, equivalent to a half step.
/// It's equivalent to `Interval(1)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::MINOR_SECOND;
///
/// assert_eq!(MINOR_SECOND.to_string(), "m2");
/// assert_eq!(u8::from(MINOR_SECOND), 1);
/// ```
pub const MINOR_SECOND: Interval = Interval(1);

/// Major second interval constant - 2 semitones
///
/// This represents a major second interval, equivalent to a whole step.
/// It's equivalent to `Interval(2)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::MAJOR_SECOND;
///
/// assert_eq!(MAJOR_SECOND.to_string(), "M2");
/// assert_eq!(u8::from(MAJOR_SECOND), 2);
/// ```
pub const MAJOR_SECOND: Interval = Interval(2);

/// Minor third interval constant - 3 semitones
///
/// This represents a minor third interval.
/// It's equivalent to `Interval(3)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::MINOR_THIRD;
///
/// assert_eq!(MINOR_THIRD.to_string(), "m3");
/// assert_eq!(u8::from(MINOR_THIRD), 3);
/// ```
pub const MINOR_THIRD: Interval = Interval(3);

/// Major third interval constant - 4 semitones
///
/// This represents a major third interval.
/// It's equivalent to `Interval(4)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::MAJOR_THIRD;
///
/// assert_eq!(MAJOR_THIRD.to_string(), "M3");
/// assert_eq!(u8::from(MAJOR_THIRD), 4);
/// ```
pub const MAJOR_THIRD: Interval = Interval(4);

/// Perfect fourth interval constant - 5 semitones
///
/// This represents a perfect fourth interval.
/// It's equivalent to `Interval(5)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::PERFECT_FOURTH;
///
/// assert_eq!(PERFECT_FOURTH.to_string(), "P4");
/// assert_eq!(u8::from(PERFECT_FOURTH), 5);
/// ```
pub const PERFECT_FOURTH: Interval = Interval(5);

/// Augmented fourth interval constant - 6 semitones
///
/// This represents an augmented fourth interval, also known as the tritone.
/// It's equivalent to `Interval(6)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::AUGMENTED_FOURTH;
///
/// assert_eq!(AUGMENTED_FOURTH.to_string(), "d5");
/// assert_eq!(u8::from(AUGMENTED_FOURTH), 6);
/// ```
pub const AUGMENTED_FOURTH: Interval = Interval(6);

/// Diminished fifth interval constant - 6 semitones
///
/// This represents a diminished fifth interval, also known as the tritone.
/// It's equivalent to `Interval(6)` and has the same semitone value as `AUGMENTED_FOURTH`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::DIMINISHED_FIFTH;
///
/// assert_eq!(DIMINISHED_FIFTH.to_string(), "d5");
/// assert_eq!(u8::from(DIMINISHED_FIFTH), 6);
/// ```
pub const DIMINISHED_FIFTH: Interval = Interval(6);

/// Perfect fifth interval constant - 7 semitones
///
/// This represents a perfect fifth interval.
/// It's equivalent to `Interval(7)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::PERFECT_FIFTH;
///
/// assert_eq!(PERFECT_FIFTH.to_string(), "P5");
/// assert_eq!(u8::from(PERFECT_FIFTH), 7);
/// ```
pub const PERFECT_FIFTH: Interval = Interval(7);

/// Minor sixth interval constant - 8 semitones
///
/// This represents a minor sixth interval.
/// It's equivalent to `Interval(8)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::MINOR_SIXTH;
///
/// assert_eq!(MINOR_SIXTH.to_string(), "m6");
/// assert_eq!(u8::from(MINOR_SIXTH), 8);
/// ```
pub const MINOR_SIXTH: Interval = Interval(8);

/// Major sixth interval constant - 9 semitones
///
/// This represents a major sixth interval.
/// It's equivalent to `Interval(9)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::MAJOR_SIXTH;
///
/// assert_eq!(MAJOR_SIXTH.to_string(), "M6");
/// assert_eq!(u8::from(MAJOR_SIXTH), 9);
/// ```
pub const MAJOR_SIXTH: Interval = Interval(9);

/// Minor seventh interval constant - 10 semitones
///
/// This represents a minor seventh interval.
/// It's equivalent to `Interval(10)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::MINOR_SEVENTH;
///
/// assert_eq!(MINOR_SEVENTH.to_string(), "m7");
/// assert_eq!(u8::from(MINOR_SEVENTH), 10);
/// ```
pub const MINOR_SEVENTH: Interval = Interval(10);

/// Major seventh interval constant - 11 semitones
///
/// This represents a major seventh interval.
/// It's equivalent to `Interval(11)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::MAJOR_SEVENTH;
///
/// assert_eq!(MAJOR_SEVENTH.to_string(), "M7");
/// assert_eq!(u8::from(MAJOR_SEVENTH), 11);
/// ```
pub const MAJOR_SEVENTH: Interval = Interval(11);

/// Octave interval constant - 12 semitones
///
/// This represents an octave interval, where the higher note has double the frequency
/// of the lower note. It's equivalent to `Interval(12)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::OCTAVE;
///
/// assert_eq!(OCTAVE.to_string(), "P8");
/// assert_eq!(u8::from(OCTAVE), 12);
/// ```
pub const OCTAVE: Interval = Interval(12);

impl Interval {
    /// Returns the underlying semitone value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::Interval;
    /// assert_eq!(Interval::from(4).inner(), 4);
    /// ```
    #[inline]
    pub const fn inner(&self) -> u8 {
        self.0
    }

    /// Adds a step interval to the interval
    ///
    /// # Arguments
    /// * `step` - The step interval to add
    ///
    /// # Returns
    /// A new Interval with the step interval added
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::{Interval, Step, WHOLE};
    /// assert_eq!(Interval::from(4).add_step(WHOLE), Interval::from(6));
    /// ```
    #[inline]
    pub const fn add_step(self, step: Step) -> Self {
        Self(self.0 + step.inner())
    }
}

impl From<Interval> for u8 {
    /// Converts an `Interval` to its corresponding `u8` value
    ///
    /// This conversion extracts the underlying semitone value from the Interval.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::{Interval, MAJOR_THIRD, PERFECT_FIFTH};
    ///
    /// assert_eq!(u8::from(MAJOR_THIRD), 4);
    /// assert_eq!(u8::from(PERFECT_FIFTH), 7);
    /// assert_eq!(u8::from(Interval::from(15)), 15);
    /// ```
    #[inline]
    fn from(interval: Interval) -> Self {
        interval.inner()
    }
}

impl From<u8> for Interval {
    /// Converts a `u8` value to an `Interval`
    ///
    /// This conversion creates a new Interval with the specified semitone value.
    /// Any positive integer value is valid for creating custom intervals.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of semitones for the interval
    ///
    /// # Returns
    ///
    /// A new `Interval` with the specified semitone value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::{Interval, MAJOR_THIRD, PERFECT_FIFTH};
    ///
    /// let major_third = Interval::from(4);
    /// let perfect_fifth = Interval::from(7);
    /// let custom_interval = Interval::from(15);
    ///
    /// assert_eq!(major_third, MAJOR_THIRD);
    /// assert_eq!(perfect_fifth, PERFECT_FIFTH);
    /// assert_eq!(u8::from(custom_interval), 15);
    /// ```
    #[inline]
    fn from(value: u8) -> Self {
        Interval(value)
    }
}

impl Display for Interval {
    /// Formats the interval as its string representation
    ///
    /// Returns the appropriate string representation for each interval type:
    /// - Unison: "P1"
    /// - Minor 2nd: "m2"
    /// - Major 2nd: "M2"
    /// - Minor 3rd: "m3"
    /// - Major 3rd: "M3"
    /// - Perfect 4th: "P4"
    /// - Augmented 4th: "A4"
    /// - Diminished 5th: "d5"
    /// - Perfect 5th: "P5"
    /// - Minor 6th: "m6"
    /// - Major 6th: "M6"
    /// - Minor 7th: "m7"
    /// - Major 7th: "M7"
    /// - Octave: "P8"
    /// - Custom intervals: "I{n}" where n is the semitone value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::{Interval, MAJOR_THIRD, PERFECT_FIFTH, OCTAVE};
    ///
    /// assert_eq!(MAJOR_THIRD.to_string(), "M3");
    /// assert_eq!(PERFECT_FIFTH.to_string(), "P5");
    /// assert_eq!(OCTAVE.to_string(), "P8");
    /// assert_eq!(Interval::from(15).to_string(), "I15");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "P1"),
            1 => write!(f, "m2"),
            2 => write!(f, "M2"),
            3 => write!(f, "m3"),
            4 => write!(f, "M3"),
            5 => write!(f, "P4"),
            6 => write!(f, "d5"), // Default to augmented fourth for semitone 6
            7 => write!(f, "P5"),
            8 => write!(f, "m6"),
            9 => write!(f, "M6"),
            10 => write!(f, "m7"),
            11 => write!(f, "M7"),
            12 => write!(f, "P8"),
            n => write!(f, "I{n}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        // Test that each interval displays the correct string representation
        assert_eq!(UNISON.to_string(), "P1");
        assert_eq!(MINOR_SECOND.to_string(), "m2");
        assert_eq!(MAJOR_SECOND.to_string(), "M2");
        assert_eq!(MINOR_THIRD.to_string(), "m3");
        assert_eq!(MAJOR_THIRD.to_string(), "M3");
        assert_eq!(PERFECT_FOURTH.to_string(), "P4");
        assert_eq!(AUGMENTED_FOURTH.to_string(), "d5");
        assert_eq!(DIMINISHED_FIFTH.to_string(), "d5");
        assert_eq!(PERFECT_FIFTH.to_string(), "P5");
        assert_eq!(MINOR_SIXTH.to_string(), "m6");
        assert_eq!(MAJOR_SIXTH.to_string(), "M6");
        assert_eq!(MINOR_SEVENTH.to_string(), "m7");
        assert_eq!(MAJOR_SEVENTH.to_string(), "M7");
        assert_eq!(OCTAVE.to_string(), "P8");

        // Test custom intervals
        assert_eq!(Interval::from(13).to_string(), "I13");
        assert_eq!(Interval::from(0).to_string(), "P1");
        assert_eq!(Interval::from(255).to_string(), "I255");
    }

    #[test]
    fn test_from_interval_to_u8() {
        // Test conversion from Interval to u8
        assert_eq!(u8::from(UNISON), 0);
        assert_eq!(u8::from(MINOR_SECOND), 1);
        assert_eq!(u8::from(MAJOR_SECOND), 2);
        assert_eq!(u8::from(MINOR_THIRD), 3);
        assert_eq!(u8::from(MAJOR_THIRD), 4);
        assert_eq!(u8::from(PERFECT_FOURTH), 5);
        assert_eq!(u8::from(AUGMENTED_FOURTH), 6);
        assert_eq!(u8::from(DIMINISHED_FIFTH), 6);
        assert_eq!(u8::from(PERFECT_FIFTH), 7);
        assert_eq!(u8::from(MINOR_SIXTH), 8);
        assert_eq!(u8::from(MAJOR_SIXTH), 9);
        assert_eq!(u8::from(MINOR_SEVENTH), 10);
        assert_eq!(u8::from(MAJOR_SEVENTH), 11);
        assert_eq!(u8::from(OCTAVE), 12);
        assert_eq!(u8::from(Interval::from(15)), 15);
        assert_eq!(u8::from(Interval::from(0)), 0);
        assert_eq!(u8::from(Interval::from(255)), 255);
    }

    #[test]
    fn test_from_u8_to_interval() {
        // Test conversion from u8 to Interval
        assert_eq!(Interval::from(0), UNISON);
        assert_eq!(Interval::from(1), MINOR_SECOND);
        assert_eq!(Interval::from(2), MAJOR_SECOND);
        assert_eq!(Interval::from(3), MINOR_THIRD);
        assert_eq!(Interval::from(4), MAJOR_THIRD);
        assert_eq!(Interval::from(5), PERFECT_FOURTH);
        assert_eq!(Interval::from(6), AUGMENTED_FOURTH);
        assert_eq!(Interval::from(6), DIMINISHED_FIFTH);
        assert_eq!(Interval::from(7), PERFECT_FIFTH);
        assert_eq!(Interval::from(8), MINOR_SIXTH);
        assert_eq!(Interval::from(9), MAJOR_SIXTH);
        assert_eq!(Interval::from(10), MINOR_SEVENTH);
        assert_eq!(Interval::from(11), MAJOR_SEVENTH);
        assert_eq!(Interval::from(12), OCTAVE);
        assert_eq!(Interval::from(15), Interval(15));
        assert_eq!(Interval::from(0), Interval(0));
        assert_eq!(Interval::from(255), Interval(255));
    }

    #[test]
    fn test_roundtrip_conversion() {
        // Test that converting from Interval to u8 and back preserves the value
        let test_values = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 15, 255];

        for value in &test_values {
            let interval = Interval::from(*value);
            let converted_back = u8::from(interval);
            assert_eq!(*value, converted_back);
        }
    }

    #[test]
    fn test_equality() {
        // Test that identical intervals are equal
        assert_eq!(UNISON, UNISON);
        assert_eq!(MAJOR_THIRD, MAJOR_THIRD);
        assert_eq!(PERFECT_FIFTH, PERFECT_FIFTH);
        assert_eq!(OCTAVE, OCTAVE);
        assert_eq!(Interval::from(15), Interval::from(15));

        // Test that different intervals are not equal
        assert_ne!(UNISON, MAJOR_THIRD);
        assert_ne!(MAJOR_THIRD, PERFECT_FIFTH);
        assert_ne!(PERFECT_FIFTH, OCTAVE);
        assert_ne!(Interval::from(4), Interval::from(5));

        // Test that AUGMENTED_FOURTH and DIMINISHED_FIFTH are equal (same semitone value)
        assert_eq!(AUGMENTED_FOURTH, DIMINISHED_FIFTH);
    }

    #[test]
    fn test_clone_and_copy() {
        // Test that intervals can be cloned and copied
        let original = MAJOR_THIRD;
        let cloned = original.clone();
        let copied = original;

        assert_eq!(original, cloned);
        assert_eq!(original, copied);
        assert_eq!(cloned, copied);

        // Test with custom interval
        let custom_original = Interval::from(15);
        let custom_cloned = custom_original.clone();
        let custom_copied = custom_original;

        assert_eq!(custom_original, custom_cloned);
        assert_eq!(custom_original, custom_copied);
        assert_eq!(custom_cloned, custom_copied);
    }

    #[test]
    fn test_debug_formatting() {
        // Test that debug formatting works correctly
        let debug_str = format!("{:?}", MAJOR_THIRD);
        assert!(debug_str.contains("Interval"));
        assert!(debug_str.contains("4"));

        let debug_str = format!("{:?}", PERFECT_FIFTH);
        assert!(debug_str.contains("Interval"));
        assert!(debug_str.contains("7"));

        let debug_str = format!("{:?}", Interval::from(42));
        assert!(debug_str.contains("Interval"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;

        // Test that intervals can be used as HashMap keys
        let mut map = HashMap::new();
        map.insert(UNISON, "unison");
        map.insert(MAJOR_THIRD, "major third");
        map.insert(PERFECT_FIFTH, "perfect fifth");
        map.insert(OCTAVE, "octave");
        map.insert(Interval::from(15), "custom");

        assert_eq!(map.get(&UNISON), Some(&"unison"));
        assert_eq!(map.get(&MAJOR_THIRD), Some(&"major third"));
        assert_eq!(map.get(&PERFECT_FIFTH), Some(&"perfect fifth"));
        assert_eq!(map.get(&OCTAVE), Some(&"octave"));
        assert_eq!(map.get(&Interval::from(15)), Some(&"custom"));
        assert_eq!(map.get(&Interval::from(16)), None);
    }

    #[test]
    fn test_constants() {
        // Test that constants have the correct values
        assert_eq!(u8::from(UNISON), 0);
        assert_eq!(u8::from(MINOR_SECOND), 1);
        assert_eq!(u8::from(MAJOR_SECOND), 2);
        assert_eq!(u8::from(MINOR_THIRD), 3);
        assert_eq!(u8::from(MAJOR_THIRD), 4);
        assert_eq!(u8::from(PERFECT_FOURTH), 5);
        assert_eq!(u8::from(AUGMENTED_FOURTH), 6);
        assert_eq!(u8::from(DIMINISHED_FIFTH), 6);
        assert_eq!(u8::from(PERFECT_FIFTH), 7);
        assert_eq!(u8::from(MINOR_SIXTH), 8);
        assert_eq!(u8::from(MAJOR_SIXTH), 9);
        assert_eq!(u8::from(MINOR_SEVENTH), 10);
        assert_eq!(u8::from(MAJOR_SEVENTH), 11);
        assert_eq!(u8::from(OCTAVE), 12);

        // Test that constants are equivalent to their Interval equivalents
        assert_eq!(UNISON, Interval::from(0));
        assert_eq!(MAJOR_THIRD, Interval::from(4));
        assert_eq!(PERFECT_FIFTH, Interval::from(7));
        assert_eq!(OCTAVE, Interval::from(12));
    }

    #[test]
    fn test_custom_intervals() {
        // Test creating and using custom interval values
        let interval_13 = Interval::from(13);
        let interval_15 = Interval::from(15);
        let interval_24 = Interval::from(24);
        let interval_255 = Interval::from(255);

        assert_eq!(interval_13.to_string(), "I13");
        assert_eq!(interval_15.to_string(), "I15");
        assert_eq!(interval_24.to_string(), "I24");
        assert_eq!(interval_255.to_string(), "I255");

        assert_eq!(u8::from(interval_13), 13);
        assert_eq!(u8::from(interval_15), 15);
        assert_eq!(u8::from(interval_24), 24);
        assert_eq!(u8::from(interval_255), 255);
    }

    #[test]
    fn test_interval_relationships() {
        // Test common interval relationships
        assert_eq!(u8::from(MAJOR_SECOND), 2);
        assert_eq!(u8::from(MINOR_SECOND), 1);
        assert_eq!(u8::from(MAJOR_SECOND) - u8::from(MINOR_SECOND), 1);

        assert_eq!(u8::from(MAJOR_THIRD), 4);
        assert_eq!(u8::from(MINOR_THIRD), 3);
        assert_eq!(u8::from(MAJOR_THIRD) - u8::from(MINOR_THIRD), 1);

        assert_eq!(u8::from(MAJOR_SIXTH), 9);
        assert_eq!(u8::from(MINOR_SIXTH), 8);
        assert_eq!(u8::from(MAJOR_SIXTH) - u8::from(MINOR_SIXTH), 1);

        assert_eq!(u8::from(MAJOR_SEVENTH), 11);
        assert_eq!(u8::from(MINOR_SEVENTH), 10);
        assert_eq!(u8::from(MAJOR_SEVENTH) - u8::from(MINOR_SEVENTH), 1);

        // Test perfect intervals
        assert_eq!(u8::from(PERFECT_FOURTH), 5);
        assert_eq!(u8::from(PERFECT_FIFTH), 7);
        assert_eq!(u8::from(PERFECT_FOURTH) + u8::from(PERFECT_FIFTH), 12);
        assert_eq!(
            u8::from(PERFECT_FOURTH) + u8::from(PERFECT_FIFTH),
            u8::from(OCTAVE)
        );
    }

    #[test]
    fn test_tritone_equivalence() {
        // Test that augmented fourth and diminished fifth are equivalent (tritone)
        assert_eq!(AUGMENTED_FOURTH, DIMINISHED_FIFTH);
        assert_eq!(u8::from(AUGMENTED_FOURTH), u8::from(DIMINISHED_FIFTH));
        assert_eq!(u8::from(AUGMENTED_FOURTH), 6);
        assert_eq!(u8::from(DIMINISHED_FIFTH), 6);
    }

    #[test]
    fn test_octave_relationships() {
        // Test octave relationships
        assert_eq!(u8::from(OCTAVE), 12);
        assert_eq!(u8::from(UNISON), 0);
        assert_eq!(u8::from(OCTAVE) - u8::from(UNISON), 12);

        // Test that octave is exactly 12 semitones
        let octave_interval = Interval::from(12);
        assert_eq!(octave_interval, OCTAVE);
        assert_eq!(u8::from(octave_interval), 12);
    }

    #[test]
    fn test_inner_method() {
        // Test the inner() method
        assert_eq!(UNISON.inner(), 0);
        assert_eq!(MAJOR_THIRD.inner(), 4);
        assert_eq!(PERFECT_FIFTH.inner(), 7);
        assert_eq!(OCTAVE.inner(), 12);
        assert_eq!(Interval::from(15).inner(), 15);
        assert_eq!(Interval::from(255).inner(), 255);
    }
}
