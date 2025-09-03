//! Chord Degree Types
//!
//! This module provides types for representing chord degrees and their accidentals.
//! Chord degrees represent the position of notes within a chord (1st, 3rd, 5th, etc.)
//! and can be modified with accidentals (natural, flat, sharp, double flat).

use std::fmt::Display;

/// Represents the accidental modification for a chord degree
///
/// This enum defines the possible accidental modifications that can be applied
/// to chord degrees. Each variant corresponds to a specific pitch modification
/// in musical theory.
///
/// # Examples
///
/// ```rust
/// use muzze_std::{DegreeAccidental, DEGREE_NATURAL, DEGREE_FLAT};
///
/// let natural = DEGREE_NATURAL;
/// let flat = DEGREE_FLAT;
/// assert_eq!(natural, DegreeAccidental::Natural);
/// assert_eq!(flat, DegreeAccidental::Flat);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DegreeAccidental {
    /// Natural accidental - no pitch modification
    Natural = 1,
    /// Flat accidental - lowers pitch by one semitone
    Flat = 2,
    /// Double flat accidental - lowers pitch by two semitones
    DoubleFlat = 3,
    /// Sharp accidental - raises pitch by one semitone
    Sharp = 4,
}

impl Display for DegreeAccidental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DegreeAccidental::Natural => write!(f, ""),
            DegreeAccidental::Flat => write!(f, "♭"),
            DegreeAccidental::DoubleFlat => write!(f, "♭♭"),
            DegreeAccidental::Sharp => write!(f, "♯"),
        }
    }
}

impl From<DegreeAccidental> for u8 {
    #[inline]
    fn from(accidental: DegreeAccidental) -> Self {
        accidental as u8
    }
}

impl TryFrom<u8> for DegreeAccidental {
    type Error = &'static str;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DegreeAccidental::Natural),
            2 => Ok(DegreeAccidental::Flat),
            3 => Ok(DegreeAccidental::DoubleFlat),
            4 => Ok(DegreeAccidental::Sharp),
            _ => Err("Invalid degree accidental value: {value}"),
        }
    }
}

/// Natural accidental constant for chord degrees
///
/// This represents no pitch modification for a chord degree.
/// It's equivalent to `DegreeAccidental::Natural`.
pub const DEGREE_NATURAL: DegreeAccidental = DegreeAccidental::Natural;

/// Flat accidental constant for chord degrees
///
/// This represents a flat accidental that lowers the pitch by one semitone.
/// It's equivalent to `DegreeAccidental::Flat`.
pub const DEGREE_FLAT: DegreeAccidental = DegreeAccidental::Flat;

/// Double flat accidental constant for chord degrees
///
/// This represents a double flat accidental that lowers the pitch by two semitones.
/// It's equivalent to `DegreeAccidental::DoubleFlat`.
pub const DEGREE_DOUBLEFLAT: DegreeAccidental = DegreeAccidental::DoubleFlat;

/// Sharp accidental constant for chord degrees
///
/// This represents a sharp accidental that raises the pitch by one semitone.
/// It's equivalent to `DegreeAccidental::Sharp`.
pub const DEGREE_SHARP: DegreeAccidental = DegreeAccidental::Sharp;

/// Represents a chord degree with its accidental modification
///
/// A `Degree` represents a specific position within a chord (1st, 3rd, 5th, etc.)
/// along with any accidental modification. This is used to describe the
/// theoretical structure of chords in music theory.
///
/// # Examples
///
/// ```rust
/// use muzze_std::{Degree, THIRD, FLAT_THIRD, SHARP_FIFTH};
///
/// // Using predefined constants
/// let major_third = THIRD;        // Natural 3rd
/// let minor_third = FLAT_THIRD;   // Flat 3rd
/// let augmented_fifth = SHARP_FIFTH; // Sharp 5th
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Degree {
    /// The degree number (1-7 for standard chord degrees)
    degree: u8,
    /// The accidental modification for this degree
    accidental: DegreeAccidental,
}

impl Degree {
    /// Creates a new `Degree` with the specified degree number and accidental
    ///
    /// # Arguments
    /// * `degree` - The degree number (typically 1-7 for chord degrees)
    /// * `accidental` - The accidental modification for this degree
    ///
    /// # Returns
    /// A new `Degree` instance with the specified degree and accidental
    pub(crate) const fn new(degree: u8, accidental: DegreeAccidental) -> Self {
        Self { degree, accidental }
    }

    pub const fn degree(&self) -> u8 {
        self.degree
    }

    pub const fn accidental(&self) -> DegreeAccidental {
        self.accidental
    }
}

impl Display for Degree {
    /// Formats the degree as its string representation
    ///
    /// The root degree (1st degree) is displayed as "R" for clarity,
    /// while all other degrees show their accidental (if any) followed
    /// by the degree number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::{ROOT, THIRD, FLAT_THIRD, SHARP_FIFTH};
    ///
    /// assert_eq!(format!("{}", ROOT), "R");
    /// assert_eq!(format!("{}", THIRD), "3");
    /// assert_eq!(format!("{}", FLAT_THIRD), "♭3");
    /// assert_eq!(format!("{}", SHARP_FIFTH), "♯5");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.degree == 1 {
            write!(f, "R")
        } else {
            write!(f, "{}{}", self.accidental, self.degree)
        }
    }
}

/// Root degree constant - 1st degree with natural accidental
///
/// This represents the root note of a chord, which is the fundamental
/// note that gives the chord its name and tonal center. When displayed,
/// the root degree shows as "R" instead of "1".
pub const ROOT: Degree = Degree::new(1, DEGREE_NATURAL);

/// Second degree constant - 2nd degree with natural accidental
pub const SECOND: Degree = Degree::new(2, DEGREE_NATURAL);

/// Third degree constant - 3rd degree with natural accidental
///
/// This represents a major third interval from the root. In major chords,
/// this creates the characteristic "major" sound.
pub const THIRD: Degree = Degree::new(3, DEGREE_NATURAL);

/// Flat third degree constant - 3rd degree with flat accidental
///
/// This represents a minor third interval from the root. In minor chords,
/// this creates the characteristic "minor" sound.
pub const FLAT_THIRD: Degree = Degree::new(3, DEGREE_FLAT);

/// Fourth degree constant - 4th degree with natural accidental
///
/// This represents a perfect fourth interval from the root. This degree
/// is commonly used in suspended chords (sus4) and some jazz voicings.
pub const FOURTH: Degree = Degree::new(4, DEGREE_NATURAL);

/// Fifth degree constant - 5th degree with natural accidental
///
/// This represents a perfect fifth interval from the root. This is a
/// fundamental interval in most chord types and provides harmonic stability.
pub const FIFTH: Degree = Degree::new(5, DEGREE_NATURAL);

/// Sixth degree constant - 6th degree with natural accidental
///
/// This represents a minor sixth interval from the root. This is used
/// in minor sixth chords and creates a mellow, bluesy sound.
pub const SIXTH: Degree = Degree::new(6, DEGREE_NATURAL);

/// Flat fifth degree constant - 5th degree with flat accidental
///
/// This represents a diminished fifth interval from the root. This is used
/// in diminished chords and creates the characteristic "tritone" sound.
pub const FLAT_FIFTH: Degree = Degree::new(5, DEGREE_FLAT);

/// Sharp fifth degree constant - 5th degree with sharp accidental
///
/// This represents an augmented fifth interval from the root. This is used
/// in augmented chords and creates a bright, tense sound.
pub const SHARP_FIFTH: Degree = Degree::new(5, DEGREE_SHARP);

/// Seventh degree constant - 7th degree with natural accidental
///
/// This represents a major seventh interval from the root. This is used
/// in major 7th chords and creates a sophisticated, jazzy sound.
pub const SEVENTH: Degree = Degree::new(7, DEGREE_NATURAL);

/// Flat seventh degree constant - 7th degree with flat accidental
///
/// This represents a minor seventh interval from the root. This is used
/// in dominant 7th chords and minor 7th chords, creating a bluesy sound.
pub const FLAT_SEVENTH: Degree = Degree::new(7, DEGREE_FLAT);

/// Double flat seventh degree constant - 7th degree with double flat accidental
///
/// This represents a diminished seventh interval from the root. This is used
/// in diminished 7th chords and creates a very tense, unstable sound.
pub const DOUBLEFLAT_SEVENTH: Degree = Degree::new(7, DEGREE_DOUBLEFLAT);

/// Ninth degree constant - 9th degree with natural accidental
///
/// This represents a major ninth interval from the root. This is used
/// in major ninth chords and creates a bright, tense sound.
pub const NINTH: Degree = Degree::new(9, DEGREE_NATURAL);

/// Eleventh degree constant - 11th degree with natural accidental
///
/// This represents a major eleventh interval from the root. This is used
/// in major eleventh chords and creates a bright, tense sound.
pub const ELEVENTH: Degree = Degree::new(11, DEGREE_NATURAL);

/// Thirteenth degree constant - 13th degree with natural accidental
///
/// This represents a major thirteenth interval from the root. This is used
/// in major thirteenth chords and creates a bright, tense sound.
pub const THIRTEENTH: Degree = Degree::new(13, DEGREE_NATURAL);

#[cfg(test)]
mod tests {
    use super::*;

    // DegreeAccidental tests
    #[test]
    fn test_accidental_display() {
        assert_eq!(format!("{DEGREE_NATURAL}"), "");
        assert_eq!(format!("{DEGREE_FLAT}"), "♭");
        assert_eq!(format!("{DEGREE_DOUBLEFLAT}"), "♭♭");
        assert_eq!(format!("{DEGREE_SHARP}"), "♯");
    }

    #[test]
    fn test_accidental_from_u8() {
        assert_eq!(DegreeAccidental::try_from(1), Ok(DEGREE_NATURAL));
        assert_eq!(DegreeAccidental::try_from(2), Ok(DEGREE_FLAT));
        assert_eq!(DegreeAccidental::try_from(3), Ok(DEGREE_DOUBLEFLAT));
        assert_eq!(DegreeAccidental::try_from(4), Ok(DEGREE_SHARP));
    }

    #[test]
    // #[should_panic(expected = "Invalid degree accidental value: 5")]
    fn test_accidental_from_invalid_u8() {
        let res = DegreeAccidental::try_from(5);
        assert!(res.is_err());
    }

    #[test]
    fn test_degree_accidental_to_u8() {
        assert_eq!(u8::from(DEGREE_NATURAL), 1);
        assert_eq!(u8::from(DEGREE_FLAT), 2);
        assert_eq!(u8::from(DEGREE_DOUBLEFLAT), 3);
        assert_eq!(u8::from(DEGREE_SHARP), 4);
    }

    #[test]
    fn test_degree_accidental_roundtrip() {
        for i in 1..=4 {
            let accidental = DegreeAccidental::try_from(i);
            assert!(accidental.is_ok());
            assert_eq!(u8::from(accidental.unwrap()), i);
        }
    }

    // Degree tests
    #[test]
    fn test_degree_display() {
        assert_eq!(format!("{ROOT}"), "R");
        assert_eq!(format!("{THIRD}"), "3");
        assert_eq!(format!("{FLAT_THIRD}"), "♭3");
        assert_eq!(format!("{SHARP_FIFTH}"), "♯5");
        assert_eq!(format!("{DOUBLEFLAT_SEVENTH}"), "♭♭7");
    }

    #[test]
    fn test_degree_new() {
        let custom_degree = Degree::new(2, DEGREE_SHARP);
        assert_eq!(format!("{custom_degree}"), "♯2");
    }

    #[test]
    fn test_degree_constants() {
        assert_eq!(ROOT.degree, 1);
        assert_eq!(ROOT.accidental, DEGREE_NATURAL);

        assert_eq!(SECOND.degree, 2);
        assert_eq!(SECOND.accidental, DEGREE_NATURAL);

        assert_eq!(THIRD.degree, 3);
        assert_eq!(THIRD.accidental, DEGREE_NATURAL);

        assert_eq!(FLAT_THIRD.degree, 3);
        assert_eq!(FLAT_THIRD.accidental, DEGREE_FLAT);

        assert_eq!(FOURTH.degree, 4);
        assert_eq!(FOURTH.accidental, DEGREE_NATURAL);

        assert_eq!(FIFTH.degree, 5);
        assert_eq!(FIFTH.accidental, DEGREE_NATURAL);

        assert_eq!(FLAT_FIFTH.degree, 5);
        assert_eq!(FLAT_FIFTH.accidental, DEGREE_FLAT);

        assert_eq!(SHARP_FIFTH.degree, 5);
        assert_eq!(SHARP_FIFTH.accidental, DEGREE_SHARP);

        assert_eq!(SIXTH.degree, 6);
        assert_eq!(SIXTH.accidental, DEGREE_NATURAL);

        assert_eq!(SEVENTH.degree, 7);
        assert_eq!(SEVENTH.accidental, DEGREE_NATURAL);

        assert_eq!(FLAT_SEVENTH.degree, 7);
        assert_eq!(FLAT_SEVENTH.accidental, DEGREE_FLAT);

        assert_eq!(DOUBLEFLAT_SEVENTH.degree, 7);
        assert_eq!(DOUBLEFLAT_SEVENTH.accidental, DEGREE_DOUBLEFLAT);

        assert_eq!(NINTH.degree, 9);
        assert_eq!(NINTH.accidental, DEGREE_NATURAL);

        assert_eq!(ELEVENTH.degree, 11);
        assert_eq!(ELEVENTH.accidental, DEGREE_NATURAL);

        assert_eq!(THIRTEENTH.degree, 13);
        assert_eq!(THIRTEENTH.accidental, DEGREE_NATURAL);
    }
}
