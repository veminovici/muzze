//! Musical Step Types
//!
//! This module provides the `Step` struct for representing musical step intervals
//! (half steps, whole steps, etc.) with their corresponding semitone values and
//! display representations.

use std::fmt::Display;

/// Represents a musical step interval in semitones
///
/// A `Step` represents the distance between two notes in semitones. This struct
/// provides a type-safe way to represent common musical intervals with their
/// corresponding semitone values and display representations.
///
/// # Examples
///
/// ```rust
/// use muzze_std::{Step, HALF, WHOLE, WHOLE_HALF};
///
/// // Using predefined constants
/// let half_step = HALF;
/// assert_eq!(half_step.to_string(), "H");
/// assert_eq!(u8::from(half_step), 1);
///
/// // Creating custom steps
/// let custom_step = Step::from(4);
/// assert_eq!(custom_step.to_string(), "S4");
/// ```
///
/// # Semitone Values
///
/// Common step intervals and their semitone values:
/// - Half step (H): 1 semitone
/// - Whole step (W): 2 semitones  
/// - Whole-half step (WH): 3 semitones
/// - Custom steps: Any positive integer value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Step(u8);

/// Half step constant - 1 semitone
///
/// This is a convenience constant for the half step interval, which represents
/// the smallest interval in Western music. It's equivalent to `Step(1)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::HALF;
///
/// assert_eq!(HALF.to_string(), "H");
/// assert_eq!(u8::from(HALF), 1);
/// ```
pub const HALF: Step = Step(1);

/// Whole step constant - 2 semitones
///
/// This is a convenience constant for the whole step interval, which represents
/// two semitones. It's equivalent to `Step(2)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::WHOLE;
///
/// assert_eq!(WHOLE.to_string(), "W");
/// assert_eq!(u8::from(WHOLE), 2);
/// ```
pub const WHOLE: Step = Step(2);

/// Whole-half step constant - 3 semitones
///
/// This is a convenience constant for the whole-half step interval, which represents
/// three semitones. This interval is commonly found in harmonic minor scales.
/// It's equivalent to `Step(3)`.
///
/// # Examples
///
/// ```rust
/// use muzze_std::WHOLE_HALF;
///
/// assert_eq!(WHOLE_HALF.to_string(), "WH");
/// assert_eq!(u8::from(WHOLE_HALF), 3);
/// ```
pub const WHOLE_HALF: Step = Step(3);

impl Step {
    /// Returns the underlying semitone value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::Step;
    /// assert_eq!(Step::from(4).inner(), 4);
    /// ```
    #[inline]
    pub const fn inner(&self) -> u8 {
        self.0
    }
}

impl From<Step> for u8 {
    /// Converts a `Step` to its corresponding `u8` value
    ///
    /// This conversion extracts the underlying semitone value from the Step.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::{Step, HALF, WHOLE};
    ///
    /// assert_eq!(u8::from(HALF), 1);
    /// assert_eq!(u8::from(WHOLE), 2);
    /// assert_eq!(u8::from(Step::from(4)), 4);
    /// ```
    #[inline]
    fn from(step: Step) -> Self {
        step.inner()
    }
}

impl From<u8> for Step {
    /// Converts a `u8` value to a `Step`
    ///
    /// This conversion creates a new Step with the specified semitone value.
    /// Any positive integer value is valid for creating custom step intervals.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of semitones for the step interval
    ///
    /// # Returns
    ///
    /// A new `Step` with the specified semitone value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::Step;
    ///
    /// let half_step = Step::from(1);
    /// let whole_step = Step::from(2);
    /// let custom_step = Step::from(5);
    ///
    /// assert_eq!(u8::from(half_step), 1);
    /// assert_eq!(u8::from(whole_step), 2);
    /// assert_eq!(u8::from(custom_step), 5);
    /// ```
    #[inline]
    fn from(value: u8) -> Self {
        Step(value)
    }
}

impl Display for Step {
    /// Formats the step as its string representation
    ///
    /// Returns the appropriate string representation for each step type:
    /// - Half step: "H"
    /// - Whole step: "W"
    /// - Whole-half step: "WH"
    /// - Custom steps: "S{n}" where n is the semitone value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::{Step, HALF, WHOLE, WHOLE_HALF};
    ///
    /// assert_eq!(HALF.to_string(), "H");
    /// assert_eq!(WHOLE.to_string(), "W");
    /// assert_eq!(WHOLE_HALF.to_string(), "WH");
    /// assert_eq!(Step::from(4).to_string(), "S4");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            HALF => write!(f, "H"),
            WHOLE => write!(f, "W"),
            WHOLE_HALF => write!(f, "WH"),
            Step(n) => write!(f, "S{n}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        // Test that each step displays the correct string representation
        assert_eq!(HALF.to_string(), "H");
        assert_eq!(WHOLE.to_string(), "W");
        assert_eq!(WHOLE_HALF.to_string(), "WH");

        // Test custom steps
        assert_eq!(Step::from(4).to_string(), "S4");
        assert_eq!(Step::from(0).to_string(), "S0");
        assert_eq!(Step::from(255).to_string(), "S255");
    }

    #[test]
    fn test_from_step_to_u8() {
        // Test conversion from Step to u8
        assert_eq!(u8::from(HALF), 1);
        assert_eq!(u8::from(WHOLE), 2);
        assert_eq!(u8::from(WHOLE_HALF), 3);
        assert_eq!(u8::from(Step::from(4)), 4);
        assert_eq!(u8::from(Step::from(0)), 0);
        assert_eq!(u8::from(Step::from(255)), 255);
    }

    #[test]
    fn test_from_u8_to_step() {
        // Test conversion from u8 to Step
        assert_eq!(Step::from(1), HALF);
        assert_eq!(Step::from(2), WHOLE);
        assert_eq!(Step::from(3), WHOLE_HALF);
        assert_eq!(Step::from(4), Step(4));
        assert_eq!(Step::from(0), Step(0));
        assert_eq!(Step::from(255), Step(255));
    }

    #[test]
    fn test_roundtrip_conversion() {
        // Test that converting from Step to u8 and back preserves the value
        let test_values = [0, 1, 2, 3, 4, 5, 10, 255];

        for value in &test_values {
            let step = Step::from(*value);
            let converted_back = u8::from(step);
            assert_eq!(*value, converted_back);
        }
    }

    #[test]
    fn test_equality() {
        // Test that identical steps are equal
        assert_eq!(HALF, HALF);
        assert_eq!(WHOLE, WHOLE);
        assert_eq!(WHOLE_HALF, WHOLE_HALF);
        assert_eq!(Step::from(4), Step::from(4));

        // Test that different steps are not equal
        assert_ne!(HALF, WHOLE);
        assert_ne!(WHOLE, WHOLE_HALF);
        assert_ne!(HALF, WHOLE_HALF);
        assert_ne!(Step::from(4), Step::from(5));
    }

    #[test]
    fn test_clone_and_copy() {
        // Test that steps can be cloned and copied
        let original = WHOLE;
        let cloned = original.clone();
        let copied = original;

        assert_eq!(original, cloned);
        assert_eq!(original, copied);
        assert_eq!(cloned, copied);

        // Test with custom step
        let custom_original = Step::from(7);
        let custom_cloned = custom_original.clone();
        let custom_copied = custom_original;

        assert_eq!(custom_original, custom_cloned);
        assert_eq!(custom_original, custom_copied);
        assert_eq!(custom_cloned, custom_copied);
    }

    #[test]
    fn test_debug_formatting() {
        // Test that debug formatting works correctly
        let debug_str = format!("{:?}", HALF);
        assert!(debug_str.contains("Step"));
        assert!(debug_str.contains("1"));

        let debug_str = format!("{:?}", WHOLE);
        assert!(debug_str.contains("Step"));
        assert!(debug_str.contains("2"));

        let debug_str = format!("{:?}", Step::from(42));
        assert!(debug_str.contains("Step"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;

        // Test that steps can be used as HashMap keys
        let mut map = HashMap::new();
        map.insert(HALF, "half");
        map.insert(WHOLE, "whole");
        map.insert(WHOLE_HALF, "whole-half");
        map.insert(Step::from(4), "custom");

        assert_eq!(map.get(&HALF), Some(&"half"));
        assert_eq!(map.get(&WHOLE), Some(&"whole"));
        assert_eq!(map.get(&WHOLE_HALF), Some(&"whole-half"));
        assert_eq!(map.get(&Step::from(4)), Some(&"custom"));
        assert_eq!(map.get(&Step::from(5)), None);
    }

    #[test]
    fn test_constants() {
        // Test that constants have the correct values
        assert_eq!(u8::from(HALF), 1);
        assert_eq!(u8::from(WHOLE), 2);
        assert_eq!(u8::from(WHOLE_HALF), 3);

        // Test that constants are equivalent to their Step equivalents
        assert_eq!(HALF, Step::from(1));
        assert_eq!(WHOLE, Step::from(2));
        assert_eq!(WHOLE_HALF, Step::from(3));
    }

    #[test]
    fn test_custom_steps() {
        // Test creating and using custom step values
        let step_0 = Step::from(0);
        let step_5 = Step::from(5);
        let step_12 = Step::from(12);
        let step_255 = Step::from(255);

        assert_eq!(step_0.to_string(), "S0");
        assert_eq!(step_5.to_string(), "S5");
        assert_eq!(step_12.to_string(), "S12");
        assert_eq!(step_255.to_string(), "S255");

        assert_eq!(u8::from(step_0), 0);
        assert_eq!(u8::from(step_5), 5);
        assert_eq!(u8::from(step_12), 12);
        assert_eq!(u8::from(step_255), 255);
    }

    #[test]
    fn test_step_patterns() {
        // Test common scale step patterns
        let major_scale_steps = [WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF];
        let expected_values = [2, 2, 1, 2, 2, 2, 1];

        for (step, expected) in major_scale_steps.iter().zip(expected_values.iter()) {
            assert_eq!(u8::from(*step), *expected);
        }

        // Test harmonic minor scale pattern (includes WHOLE_HALF)
        let harmonic_minor_steps = [WHOLE, HALF, WHOLE, WHOLE, HALF, WHOLE_HALF, HALF];
        let expected_values = [2, 1, 2, 2, 1, 3, 1];

        for (step, expected) in harmonic_minor_steps.iter().zip(expected_values.iter()) {
            assert_eq!(u8::from(*step), *expected);
        }
    }
}
