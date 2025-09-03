//! Musical Accidental Types
//!
//! This module provides the `Accidental` enum for representing musical accidentals
//! (sharps, flats, naturals, etc.) with their corresponding Unicode symbols and
//! numeric encodings.

use std::fmt::Display;

/// Represents musical accidentals used to modify the pitch of notes
///
/// Accidentals are symbols that modify the pitch of a note by raising or lowering it
/// by one or more semitones. This enum provides a type-safe way to represent these
/// modifications with their corresponding Unicode symbols and numeric encodings.
///
/// # Examples
///
/// ```rust
/// use muzze_std::Accidental;
///
/// // Natural accidental (no modification)
/// let natural = Accidental::Natural;
/// assert_eq!(natural.to_string(), "");
///
/// // Sharp accidental (raises pitch by semitone)
/// let sharp = Accidental::Sharp;
/// assert_eq!(sharp.to_string(), "♯");
///
/// // Flat accidental (lowers pitch by semitone)
/// let flat = Accidental::Flat;
/// assert_eq!(flat.to_string(), "♭");
/// ```
///
/// # Numeric Encoding
///
/// Each accidental has a corresponding numeric value:
/// - Natural: 0
/// - Reset: 15
/// - Flat: 2
/// - DoubleFlat: 3
/// - Sharp: 8
/// - DoubleSharp: 9
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Accidental {
    /// Natural accidental - no pitch modification
    ///
    /// Represents the absence of any accidental, indicating that the note
    /// should be played at its natural pitch without any modification.
    Natural = 0,

    /// Reset accidental - explicitly cancels previous accidentals
    ///
    /// Used to explicitly cancel any previous accidentals in the same measure,
    /// returning the note to its natural state. Displayed as ♮.
    Reset = 15,

    /// Flat accidental - lowers pitch by one semitone
    ///
    /// Lowers the pitch of a note by one semitone (half step).
    /// Displayed as ♭.
    Flat = 2,

    /// Double flat accidental - lowers pitch by two semitones
    ///
    /// Lowers the pitch of a note by two semitones (whole step).
    /// Displayed as ♭♭.
    DoubleFlat = 3,

    /// Sharp accidental - raises pitch by one semitone
    ///
    /// Raises the pitch of a note by one semitone (half step).
    /// Displayed as ♯.
    Sharp = 8,

    /// Double sharp accidental - raises pitch by two semitones
    ///
    /// Raises the pitch of a note by two semitones (whole step).
    /// Displayed as ♯♯.
    DoubleSharp = 9,
}

/// Natural accidental constant - no pitch modification
///
/// This is a convenience constant for the natural accidental, which represents
/// the absence of any pitch modification. It's equivalent to `Accidental::Natural`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::NATURAL;
///
/// assert_eq!(NATURAL.to_string(), "");
/// assert_eq!(u8::from(NATURAL), 0);
/// ```
pub const NATURAL: Accidental = Accidental::Natural;

/// Flat accidental constant - lowers pitch by one semitone
///
/// This is a convenience constant for the flat accidental, which lowers
/// the pitch of a note by one semitone (half step). It's equivalent to `Accidental::Flat`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::FLAT;
///
/// assert_eq!(FLAT.to_string(), "♭");
/// assert_eq!(u8::from(FLAT), 2);
/// ```
pub const FLAT: Accidental = Accidental::Flat;

/// Sharp accidental constant - raises pitch by one semitone
///
/// This is a convenience constant for the sharp accidental, which raises
/// the pitch of a note by one semitone (half step). It's equivalent to `Accidental::Sharp`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::SHARP;
///
/// assert_eq!(SHARP.to_string(), "♯");
/// assert_eq!(u8::from(SHARP), 8);
/// ```
pub const SHARP: Accidental = Accidental::Sharp;

/// Reset accidental constant - explicitly cancels previous accidentals
///
/// This is a convenience constant for the reset accidental, which explicitly
/// cancels any previous accidentals in the same measure, returning the note
/// to its natural state. It's equivalent to `Accidental::Reset`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::RESET_ACCIDENTAL;
///
/// assert_eq!(RESET_ACCIDENTAL.to_string(), "♮");
/// assert_eq!(u8::from(RESET_ACCIDENTAL), 15);
/// ```
pub const RESET_ACCIDENTAL: Accidental = Accidental::Reset;

/// Double flat accidental constant - lowers pitch by two semitones
///
/// This is a convenience constant for the double flat accidental, which lowers
/// the pitch of a note by two semitones (whole step). It's equivalent to `Accidental::DoubleFlat`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::DOUBLE_FLAT;
///
/// assert_eq!(DOUBLE_FLAT.to_string(), "♭♭");
/// assert_eq!(u8::from(DOUBLE_FLAT), 3);
/// ```
pub const DOUBLE_FLAT: Accidental = Accidental::DoubleFlat;

/// Double sharp accidental constant - raises pitch by two semitones
///
/// This is a convenience constant for the double sharp accidental, which raises
/// the pitch of a note by two semitones (whole step). It's equivalent to `Accidental::DoubleSharp`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::DOUBLE_SHARP;
///
/// assert_eq!(DOUBLE_SHARP.to_string(), "♯♯");
/// assert_eq!(u8::from(DOUBLE_SHARP), 9);
/// ```
pub const DOUBLE_SHARP: Accidental = Accidental::DoubleSharp;

impl Display for Accidental {
    /// Formats the accidental as its Unicode symbol representation
    ///
    /// Returns the appropriate Unicode symbol for each accidental type:
    /// - Natural: empty string (no symbol)
    /// - Reset: ♮ (natural symbol)
    /// - Flat: ♭ (flat symbol)
    /// - DoubleFlat: ♭♭ (double flat symbol)
    /// - Sharp: ♯ (sharp symbol)
    /// - DoubleSharp: ♯♯ (double sharp symbol)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::Accidental;
    ///
    /// assert_eq!(Accidental::Natural.to_string(), "");
    /// assert_eq!(Accidental::Sharp.to_string(), "♯");
    /// assert_eq!(Accidental::Flat.to_string(), "♭");
    /// assert_eq!(Accidental::DoubleSharp.to_string(), "♯♯");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accidental::Natural => write!(f, ""),
            Accidental::Reset => write!(f, "♮"),
            Accidental::Flat => write!(f, "♭"),
            Accidental::DoubleFlat => write!(f, "♭♭"),
            Accidental::Sharp => write!(f, "♯"),
            Accidental::DoubleSharp => write!(f, "♯♯"),
        }
    }
}

impl From<Accidental> for u8 {
    /// Converts an `Accidental` to its corresponding `u8` value
    ///
    /// This conversion uses the numeric encoding defined for each accidental:
    /// - Natural: 0
    /// - Reset: 15
    /// - Flat: 2
    /// - DoubleFlat: 3
    /// - Sharp: 8
    /// - DoubleSharp: 9
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::Accidental;
    ///
    /// assert_eq!(u8::from(Accidental::Natural), 0);
    /// assert_eq!(u8::from(Accidental::Sharp), 8);
    /// assert_eq!(u8::from(Accidental::Flat), 2);
    /// ```
    fn from(accidental: Accidental) -> Self {
        accidental as u8
    }
}

impl From<u8> for Accidental {
    /// Converts a `u8` value to its corresponding `Accidental`
    ///
    /// This conversion maps numeric values to their corresponding accidental types.
    /// Only specific values are valid; any other value will cause a panic.
    ///
    /// # Arguments
    ///
    /// * `value` - The numeric value to convert (0, 1, 2, 3, 8, or 9)
    ///
    /// # Returns
    ///
    /// The corresponding `Accidental` variant
    ///
    /// # Panics
    ///
    /// Panics if the provided value is not a valid accidental encoding.
    /// Valid values are: 0, 2, 3, 8, 9, 15.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::Accidental;
    ///
    /// assert_eq!(Accidental::from(0), Accidental::Natural);
    /// assert_eq!(Accidental::from(8), Accidental::Sharp);
    /// assert_eq!(Accidental::from(2), Accidental::Flat);
    /// ```
    ///
    /// # Panic Example
    ///
    /// ```rust,should_panic
    /// use muzze_std::Accidental;
    ///
    /// // This will panic because 5 is not a valid accidental value
    /// let _ = Accidental::from(5);
    /// ```
    fn from(value: u8) -> Self {
        match value {
            0 => Accidental::Natural,
            2 => Accidental::Flat,
            3 => Accidental::DoubleFlat,
            8 => Accidental::Sharp,
            9 => Accidental::DoubleSharp,
            15 => Accidental::Reset,
            _ => panic!("Invalid accidental value: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        // Test that each accidental displays the correct Unicode symbol
        assert_eq!(Accidental::Natural.to_string(), "");
        assert_eq!(Accidental::Reset.to_string(), "♮");
        assert_eq!(Accidental::Flat.to_string(), "♭");
        assert_eq!(Accidental::DoubleFlat.to_string(), "♭♭");
        assert_eq!(Accidental::Sharp.to_string(), "♯");
        assert_eq!(Accidental::DoubleSharp.to_string(), "♯♯");
    }

    #[test]
    fn test_from_accidental_to_u8() {
        // Test conversion from Accidental to u8
        assert_eq!(u8::from(Accidental::Natural), 0);
        assert_eq!(u8::from(Accidental::Reset), 15);
        assert_eq!(u8::from(Accidental::Flat), 2);
        assert_eq!(u8::from(Accidental::DoubleFlat), 3);
        assert_eq!(u8::from(Accidental::Sharp), 8);
        assert_eq!(u8::from(Accidental::DoubleSharp), 9);
    }

    #[test]
    fn test_from_u8_to_accidental() {
        // Test conversion from u8 to Accidental
        assert_eq!(Accidental::from(0), Accidental::Natural);
        assert_eq!(Accidental::from(2), Accidental::Flat);
        assert_eq!(Accidental::from(3), Accidental::DoubleFlat);
        assert_eq!(Accidental::from(8), Accidental::Sharp);
        assert_eq!(Accidental::from(9), Accidental::DoubleSharp);
        assert_eq!(Accidental::from(15), Accidental::Reset);
    }

    #[test]
    #[should_panic(expected = "Invalid accidental value: 4")]
    fn test_from_invalid_u8_panics() {
        // Test that invalid u8 values cause panic
        let _ = Accidental::from(4);
    }

    #[test]
    #[should_panic(expected = "Invalid accidental value: 7")]
    fn test_from_invalid_u8_panics_another() {
        // Test another invalid u8 value
        let _ = Accidental::from(7);
    }

    #[test]
    #[should_panic(expected = "Invalid accidental value: 255")]
    fn test_from_invalid_u8_panics_max() {
        // Test maximum u8 value (invalid)
        let _ = Accidental::from(255);
    }

    #[test]
    fn test_roundtrip_conversion() {
        // Test that converting from Accidental to u8 and back preserves the value
        let accidentals = [
            Accidental::Natural,
            Accidental::Reset,
            Accidental::Flat,
            Accidental::DoubleFlat,
            Accidental::Sharp,
            Accidental::DoubleSharp,
        ];

        for accidental in &accidentals {
            let numeric_value = u8::from(*accidental);
            let converted_back = Accidental::from(numeric_value);
            assert_eq!(*accidental, converted_back);
        }
    }

    #[test]
    fn test_equality() {
        // Test that identical accidentals are equal
        assert_eq!(Accidental::Natural, Accidental::Natural);
        assert_eq!(Accidental::Sharp, Accidental::Sharp);
        assert_eq!(Accidental::Flat, Accidental::Flat);

        // Test that different accidentals are not equal
        assert_ne!(Accidental::Natural, Accidental::Sharp);
        assert_ne!(Accidental::Flat, Accidental::DoubleFlat);
        assert_ne!(Accidental::Sharp, Accidental::DoubleSharp);
    }

    #[test]
    fn test_clone_and_copy() {
        // Test that accidentals can be cloned and copied
        let original = Accidental::Sharp;
        let cloned = original.clone();
        let copied = original;

        assert_eq!(original, cloned);
        assert_eq!(original, copied);
        assert_eq!(cloned, copied);
    }

    #[test]
    fn test_debug_formatting() {
        // Test that debug formatting works correctly
        let debug_str = format!("{:?}", Accidental::Sharp);
        assert!(debug_str.contains("Sharp"));

        let debug_str = format!("{:?}", Accidental::Flat);
        assert!(debug_str.contains("Flat"));
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;

        // Test that accidentals can be used as HashMap keys
        let mut map = HashMap::new();
        map.insert(Accidental::Sharp, "raised");
        map.insert(Accidental::Flat, "lowered");
        map.insert(Accidental::Natural, "unchanged");

        assert_eq!(map.get(&Accidental::Sharp), Some(&"raised"));
        assert_eq!(map.get(&Accidental::Flat), Some(&"lowered"));
        assert_eq!(map.get(&Accidental::Natural), Some(&"unchanged"));
        assert_eq!(map.get(&Accidental::DoubleSharp), None);
    }

    #[test]
    fn test_all_variants() {
        // Test that we can iterate over all accidental variants
        let all_accidentals = [
            Accidental::Natural,
            Accidental::Reset,
            Accidental::Flat,
            Accidental::DoubleFlat,
            Accidental::Sharp,
            Accidental::DoubleSharp,
        ];

        // Verify we have exactly 6 variants
        assert_eq!(all_accidentals.len(), 6);

        // Verify each variant has a unique numeric value
        let mut values = all_accidentals
            .iter()
            .map(|&a| u8::from(a))
            .collect::<Vec<_>>();
        values.sort();
        assert_eq!(values, vec![0, 2, 3, 8, 9, 15]);
    }

    #[test]
    fn test_unicode_symbols() {
        // Test that the Unicode symbols are correct and non-empty (except Natural)
        // Note: String::len() returns byte count, not character count
        assert_eq!(Accidental::Natural.to_string().len(), 0);
        assert_eq!(Accidental::Reset.to_string().len(), 3); // ♮ is 3 bytes in UTF-8
        assert_eq!(Accidental::Flat.to_string().len(), 3); // ♭ is 3 bytes in UTF-8
        assert_eq!(Accidental::DoubleFlat.to_string().len(), 6); // ♭♭ is 6 bytes in UTF-8
        assert_eq!(Accidental::Sharp.to_string().len(), 3); // ♯ is 3 bytes in UTF-8
        assert_eq!(Accidental::DoubleSharp.to_string().len(), 6); // ♯♯ is 6 bytes in UTF-8
    }
}
