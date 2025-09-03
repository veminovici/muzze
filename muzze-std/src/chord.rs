//! Chord Degree Types
//!
//! This module provides types for representing chord degrees and their accidentals.
//! Chord degrees represent the position of notes within a chord (1st, 3rd, 5th, etc.)
//! and can be modified with accidentals (natural, flat, sharp, double flat).

use std::fmt::Display;

use muzze_bitflags::{u4vec16::U4Vec16Builder, U4Vec16};

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

impl From<u8> for DegreeAccidental {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            1 => DegreeAccidental::Natural,
            2 => DegreeAccidental::Flat,
            3 => DegreeAccidental::DoubleFlat,
            4 => DegreeAccidental::Sharp,
            _ => panic!("Invalid degree accidental value: {value}"),
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
    const fn new(degree: u8, accidental: DegreeAccidental) -> Self {
        Self { degree, accidental }
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

/// Represents a musical chord as a collection of degrees
///
/// A `Chord` is a fundamental musical structure consisting of multiple notes
/// played simultaneously. This implementation uses a `U4Vec16` to efficiently
/// store up to 16 chord degrees, where each degree can have its own accidental
/// modification (natural, flat, sharp, double flat).
///
/// The chord degrees are stored as 4-bit values representing the accidental
/// type for each degree position (1-16). A value of 0 represents natural,
/// 1 represents flat, 2 represents double flat, and 3 represents sharp.
///
/// # Examples
///
/// ```rust
/// use muzze_std::{Chord, ChordBuilder, ROOT, THIRD, FIFTH};
///
/// // Create a major triad using the builder
/// let major_triad = ChordBuilder::with_root()
///     .set_degree(THIRD)
///     .set_degree(FIFTH)
///     .build();
///
/// // Iterate over the chord degrees
/// for degree in major_triad.degrees() {
///     println!("{}", degree);
/// }
/// ```
///
/// # Musical Theory
///
/// Chords are built by stacking thirds (or other intervals) on top of a root note.
/// Common chord types include:
/// - **Triads**: Three-note chords (root, third, fifth)
/// - **Seventh chords**: Four-note chords adding a seventh degree
/// - **Suspended chords**: Chords where the third is replaced by another degree
///
/// # Performance
///
/// - **Efficient storage**: Uses 64 bits total (16 degrees × 4 bits each)
/// - **Fast iteration**: O(1) access to individual degrees
/// - **Memory compact**: No heap allocations required
/// - **Const construction**: Can be created at compile time
pub struct Chord(U4Vec16);

impl Chord {
    /// Creates a new `Chord` from a `U4Vec16` containing degree accidentals
    ///
    /// # Arguments
    /// * `degrees` - A `U4Vec16` where each 4-bit value represents the accidental
    ///   for the corresponding degree (0=natural, 1=flat, 2=double flat, 3=sharp)
    ///
    /// # Returns
    /// A new `Chord` instance with the specified degree accidentals
    ///
    /// # Example
    /// ```rust
    /// use muzze_std::{Chord, ChordBuilder, ROOT, FLAT_THIRD};
    ///
    /// let chord = ChordBuilder::with_root()
    ///     .set_degree(FLAT_THIRD)
    ///     .build();
    /// ```
    #[inline]
    const fn new(degrees: U4Vec16) -> Self {
        Self(degrees)
    }

    /// Returns an iterator over all degrees in the chord
    ///
    /// This method provides access to all 16 possible degrees (1-16) in the chord.
    /// Each degree is represented as a `Degree` struct containing the degree number
    /// and its accidental modification.
    ///
    /// # Returns
    /// An iterator that yields `Degree` instances for each position in the chord
    ///
    /// # Example
    /// ```rust
    /// use muzze_std::{Chord, ChordBuilder, ROOT, THIRD, FIFTH};
    ///
    /// let chord = ChordBuilder::with_root()
    ///     .set_degree(THIRD)
    ///     .set_degree(FIFTH)
    ///     .build();
    ///
    /// let degrees: Vec<_> = chord.degrees().collect();
    /// assert_eq!(degrees.len(), 3);
    /// assert_eq!(degrees[0], ROOT);   // 1st degree
    /// assert_eq!(degrees[1], THIRD);  // 3rd degree
    /// assert_eq!(degrees[2], FIFTH);  // 5th degree
    /// ```
    #[inline]
    pub fn degrees(&self) -> impl Iterator<Item = Degree> {
        self.0.iter_items().enumerate().filter_map(|(index, acc)| {
            if acc == 0 {
                None
            } else {
                Some(Degree::new(index as u8 + 1, DegreeAccidental::from(acc)))
            }
        })
    }
}

impl Display for Chord {
    /// Formats the chord as its string representation
    ///
    /// The chord is displayed as a hyphen-separated list of degrees,
    /// where the root degree shows as "R" and other degrees show their
    /// accidental (if any) followed by the degree number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use muzze_std::{MAJOR_TRIAD, MINOR_TRIAD, DOMINANT_SEVENTH};
    ///
    /// assert_eq!(format!("{}", MAJOR_TRIAD), "R-3-5");
    /// assert_eq!(format!("{}", MINOR_TRIAD), "R-♭3-5");
    /// assert_eq!(format!("{}", DOMINANT_SEVENTH), "R-3-5-♭7");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xs = self
            .0
            .iter_items()
            .enumerate()
            .filter_map(|(index, accidental)| {
                if accidental == 0 {
                    None
                } else {
                    Some(
                        Degree::new(index as u8 + 1, DegreeAccidental::from(accidental))
                            .to_string(),
                    )
                }
            })
            .collect::<Vec<_>>();
        write!(f, "{}", xs.join("-"))
    }
}

/// Major triad chord constant
///
/// A major triad consists of a root, major third, and perfect fifth.
/// This is the most fundamental major chord and forms the basis of
/// major harmony in Western music.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5)
/// **Quality**: Major (bright, stable sound)
/// **Display**: R-3-5
pub const MAJOR_TRIAD: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .build();

/// Minor triad chord constant
///
/// A minor triad consists of a root, minor third, and perfect fifth.
/// This is the most fundamental minor chord and forms the basis of
/// minor harmony in Western music.
///
/// **Degrees**: Root (R), Minor Third (♭3), Perfect Fifth (5)
/// **Quality**: Minor (dark, melancholic sound)
/// **Display**: R-♭3-5
pub const MINOR_TRIAD: Chord = ChordBuilder::with_root()
    .set_degree(FLAT_THIRD)
    .set_degree(FIFTH)
    .build();

/// Diminished triad chord constant
///
/// A diminished triad consists of a root, minor third, and diminished fifth.
/// This chord creates a very tense, unstable sound and is commonly used
/// as a passing chord or in diminished harmony.
///
/// **Degrees**: Root (R), Minor Third (♭3), Diminished Fifth (♭5)
/// **Quality**: Diminished (tense, unstable sound)
/// **Display**: R-♭3-♭5
pub const DIMINISHED_TRIAD: Chord = ChordBuilder::with_root()
    .set_degree(FLAT_THIRD)
    .set_degree(FLAT_FIFTH)
    .build();

/// Augmented triad chord constant
///
/// An augmented triad consists of a root, major third, and augmented fifth.
/// This chord creates a bright, tense sound and is commonly used in
/// augmented harmony and as a passing chord.
///
/// **Degrees**: Root (R), Major Third (3), Augmented Fifth (♯5)
/// **Quality**: Augmented (bright, tense sound)
/// **Display**: R-3-♯5
pub const AUGMENTED_TRIAD: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(SHARP_FIFTH)
    .build();

/// Major seventh chord constant
///
/// A major seventh chord consists of a root, major third, perfect fifth,
/// and major seventh. This chord has a sophisticated, jazzy sound and is
/// commonly used in jazz and contemporary music.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Major Seventh (7)
/// **Quality**: Major 7th (sophisticated, jazzy sound)
/// **Display**: R-3-5-7
pub const MAJOR_SEVENTH_CHORD: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(SEVENTH)
    .build();

/// Minor seventh chord constant
///
/// A minor seventh chord consists of a root, minor third, perfect fifth,
/// and minor seventh. This chord has a mellow, bluesy sound and is
/// commonly used in jazz, blues, and contemporary music.
///
/// **Degrees**: Root (R), Minor Third (♭3), Perfect Fifth (5), Minor Seventh (♭7)
/// **Quality**: Minor 7th (mellow, bluesy sound)
/// **Display**: R-♭3-5-♭7
pub const MINOR_SEVENTH_CHORD: Chord = ChordBuilder::with_root()
    .set_degree(FLAT_THIRD)
    .set_degree(FIFTH)
    .set_degree(FLAT_SEVENTH)
    .build();

/// Dominant seventh chord constant
///
/// A dominant seventh chord consists of a root, major third, perfect fifth,
/// and minor seventh. This chord has a strong, bluesy sound and is
/// commonly used as a dominant chord in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Minor Seventh (♭7)
/// **Quality**: Dominant 7th (strong, bluesy sound)
/// **Display**: R-3-5-♭7
pub const DOMINANT_SEVENTH: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(FLAT_SEVENTH)
    .build();

/// Half-diminished seventh chord constant
///
/// A half-diminished seventh chord consists of a root, minor third, diminished fifth,
/// and minor seventh. This chord has a tense, unstable sound and is commonly used
/// as a minor ii chord in functional harmony.
///
/// **Degrees**: Root (R), Minor Third (♭3), Diminished Fifth (♭5), Minor Seventh (♭7)
/// **Quality**: Half-diminished 7th (tense, unstable sound)
/// **Display**: R-♭3-♭5-♭7
pub const HALF_DIMINISHED_SEVENTH: Chord = ChordBuilder::with_root()
    .set_degree(FLAT_THIRD)
    .set_degree(FLAT_FIFTH)
    .set_degree(FLAT_SEVENTH)
    .build();

/// Diminished seventh chord constant
///
/// A diminished seventh chord consists of a root, minor third, diminished fifth,
/// and diminished seventh. This chord has a very tense, unstable sound and is
/// commonly used as a passing chord or in diminished harmony.
///
/// **Degrees**: Root (R), Minor Third (♭3), Diminished Fifth (♭5), Diminished Seventh (♭♭7)
/// **Quality**: Diminished 7th (very tense, unstable sound)
/// **Display**: R-♭3-♭5-♭♭7
pub const DIMINISHED_SEVENTH: Chord = ChordBuilder::with_root()
    .set_degree(FLAT_THIRD)
    .set_degree(FLAT_FIFTH)
    .set_degree(DOUBLEFLAT_SEVENTH)
    .build();

/// Augmented seventh chord constant
///
/// An augmented seventh chord consists of a root, major third, augmented fifth,
/// and minor seventh. This chord has a bright, tense sound and is commonly used
/// in augmented harmony and as a passing chord.
///
/// **Degrees**: Root (R), Major Third (3), Augmented Fifth (♯5), Minor Seventh (♭7)
/// **Quality**: Augmented 7th (bright, tense sound)
/// **Display**: R-3-♯5-♭7
pub const AUGMENTED_SEVENTH: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(SHARP_FIFTH)
    .set_degree(FLAT_SEVENTH)
    .build();

/// Minor major seventh chord constant
///
/// A minor major seventh chord consists of a root, minor third, perfect fifth,
/// and major seventh. This chord has a mellow, bluesy sound and is commonly used
/// in jazz, blues, and contemporary music.
///
/// **Degrees**: Root (R), Minor Third (♭3), Perfect Fifth (5), Major Seventh (7)
/// **Quality**: Minor major 7th (mellow, bluesy sound)
/// **Display**: R-♭3-5-7
pub const MINOR_MAJOR_SEVENTH: Chord = ChordBuilder::with_root()
    .set_degree(FLAT_THIRD)
    .set_degree(FIFTH)
    .set_degree(SEVENTH)
    .build();

/// Sixth chord constant
///
/// A sixth chord consists of a root, major third, and sixth. This chord has a
/// mellow, bluesy sound and is commonly used in jazz, blues, and contemporary music.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Sixth (6)
/// **Quality**: Sixth (mellow, bluesy sound)
/// **Display**: R-3-5-6
pub const SIXTH_CHORD: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(SIXTH)
    .build();

/// Sixth minor chord constant
///
/// A sixth minor chord consists of a root, minor third, and sixth. This chord has a
/// mellow, bluesy sound and is commonly used in jazz, blues, and contemporary music.
///
/// **Degrees**: Root (R), Minor Third (♭3), Perfect Fifth (5), Sixth (6)
/// **Quality**: Sixth minor (mellow, bluesy sound)
/// **Display**: R-♭3-5-6
pub const SIXTH_MINOR_CHORD: Chord = ChordBuilder::with_root()
    .set_degree(FLAT_THIRD)
    .set_degree(FIFTH)
    .set_degree(SIXTH)
    .build();

/// Sixth ninth chord constant
///
/// A sixth ninth chord consists of a root, major third, and ninth. This chord has a
/// mellow, bluesy sound and is commonly used in jazz, blues, and contemporary music.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Sixth (6), Ninth (9)
/// **Quality**: Sixth ninth (mellow, bluesy sound)
/// **Display**: R-3-5-6-9
pub const SIXTH_NINTH_CHORD: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(SIXTH)
    .set_degree(NINTH)
    .build();

/// Fifth chord constant
///
/// A fifth chord consists of a root, perfect fifth, and perfect sixth. This chord has a
/// mellow, bluesy sound and is commonly used in jazz, blues, and contemporary music.
///
/// **Degrees**: Root (R), Perfect Fifth (5)
/// **Quality**: Fifth (mellow, bluesy sound)
/// **Display**: R-5
pub const FIFTH_CHORD: Chord = ChordBuilder::with_root().set_degree(FIFTH).build();

/// Dominant ninth chord constant
///
/// A dominant ninth chord consists of a root, major third, perfect fifth,
/// and minor seventh. This chord has a strong, bluesy sound and is commonly used
/// as a dominant chord in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Minor Seventh (♭7), Ninth (9)
/// **Quality**: Dominant 9th (strong, bluesy sound)
/// **Display**: R-3-5-♭7-9
pub const DOMINANT_NINTH: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(FLAT_SEVENTH)
    .set_degree(NINTH)
    .build();

/// Minor ninth chord constant
///
/// A minor ninth chord consists of a root, minor third, perfect fifth,
/// and minor seventh. This chord has a mellow, bluesy sound and is commonly used
/// as a minor chord in functional harmony.
///
/// **Degrees**: Root (R), Minor Third (♭3), Perfect Fifth (5), Minor Seventh (♭7), Ninth (9)
/// **Quality**: Minor 9th (mellow, bluesy sound)
/// **Display**: R-♭3-5-♭7-9
pub const MINOR_NINTH: Chord = ChordBuilder::with_root()
    .set_degree(FLAT_THIRD)
    .set_degree(FIFTH)
    .set_degree(FLAT_SEVENTH)
    .set_degree(NINTH)
    .build();

/// Major ninth chord constant
///
/// A major ninth chord consists of a root, major third, perfect fifth,
/// and major seventh. This chord has a bright, tense sound and is commonly used
/// as a major chord in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Major Seventh (7), Ninth (9)
/// **Quality**: Major 9th (bright, tense sound)
/// **Display**: R-3-5-7-9
pub const MAJOR_NINTH: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(SEVENTH)
    .set_degree(NINTH)
    .build();

/// Eleventh chord constant
///
/// An eleventh chord consists of a root, major third, perfect fifth,
/// major seventh, and eleventh. This chord has a bright, tense sound and is
/// commonly used in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Major Seventh (7), Eleventh (11)
/// **Quality**: Eleventh (bright, tense sound)
/// **Display**: R-3-5-7-11
pub const ELEVENTH_CHORD: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(SEVENTH)
    .set_degree(NINTH)
    .set_degree(ELEVENTH)
    .build();

/// Minor eleventh chord constant
///
/// A minor eleventh chord consists of a root, minor third, perfect fifth,
/// minor seventh, and eleventh. This chord has a mellow, bluesy sound and is
/// commonly used in jazz, blues, and contemporary music.
///
/// **Degrees**: Root (R), Minor Third (♭3), Perfect Fifth (5), Minor Seventh (♭7), Eleventh (11)
/// **Quality**: Minor 11th (mellow, bluesy sound)
/// **Display**: R-♭3-5-♭7-11
pub const MINOR_ELEVENTH: Chord = ChordBuilder::with_root()
    .set_degree(FLAT_THIRD)
    .set_degree(FIFTH)
    .set_degree(FLAT_SEVENTH)
    .set_degree(NINTH)
    .set_degree(ELEVENTH)
    .build();

/// Thirteenth chord constant
///
/// A thirteenth chord consists of a root, major third, perfect fifth,
/// minor seventh, eleventh, and thirteenth. This chord has a bright, tense sound and is
/// commonly used in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Minor Seventh (♭7), Eleventh (11), Thirteenth (13)
/// **Quality**: Thirteenth (bright, tense sound)
/// **Display**: R-3-5-♭7-11-13
pub const THIRTEENTH_CHORD: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(FLAT_SEVENTH)
    .set_degree(NINTH)
    .set_degree(ELEVENTH)
    .set_degree(THIRTEENTH)
    .build();

/// Minor thirteenth chord constant
///
/// A minor thirteenth chord consists of a root, minor third, perfect fifth,
/// minor seventh, eleventh, and thirteenth. This chord has a mellow, bluesy sound and is
/// commonly used in jazz, blues, and contemporary music.
///
/// **Degrees**: Root (R), Minor Third (♭3), Perfect Fifth (5), Minor Seventh (♭7), Eleventh (11), Thirteenth (13)
/// **Quality**: Minor 13th (mellow, bluesy sound)
/// **Display**: R-♭3-5-♭7-11-13
pub const MINOR_THIRTEENTH: Chord = ChordBuilder::with_root()
    .set_degree(FLAT_THIRD)
    .set_degree(FIFTH)
    .set_degree(FLAT_SEVENTH)
    .set_degree(NINTH)
    .set_degree(ELEVENTH)
    .set_degree(THIRTEENTH)
    .build();

/// Major thirteenth chord constant
///
/// A major thirteenth chord consists of a root, major third, perfect fifth,
/// minor seventh, eleventh, and thirteenth. This chord has a bright, tense sound and is
/// commonly used in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Seventh (7), Eleventh (11), Thirteenth (13)
/// **Quality**: Major 13th (bright, tense sound)
/// **Display**: R-3-5-7-11-13
pub const MAJOR_THIRTEENTH: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(SEVENTH)
    .set_degree(NINTH)
    .set_degree(ELEVENTH)
    .set_degree(THIRTEENTH)
    .build();

/// Major eleventh chord constant
///
/// A major eleventh chord consists of a root, major third, perfect fifth,
/// major seventh, and eleventh. This chord has a bright, tense sound and is
/// commonly used in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Major Seventh (7), Eleventh (11)
/// **Quality**: Major 11th (bright, tense sound)
/// **Display**: R-3-5-7-11
pub const MAJOR_ELEVENTH: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(SEVENTH)
    .set_degree(NINTH)
    .set_degree(ELEVENTH)
    .build();

/// A suspended second chord consists of a root, perfect second, and perfect fifth.
/// This chord has a suspended, unresolved sound and is commonly used in
/// contemporary music and jazz.
///
/// **Degrees**: Root (R), Perfect Second (2), Perfect Fifth (5)
/// **Quality**: Suspended 2nd (suspended, unresolved sound)
/// **Display**: R-2-5
pub const SUSPENDED_SECOND: Chord = ChordBuilder::with_root()
    .set_degree(SECOND)
    .set_degree(FIFTH)
    .build();

/// Suspended fourth chord constant
///
/// A suspended fourth chord consists of a root, perfect fourth, and perfect fifth.
/// This chord has a suspended, unresolved sound and is commonly used in
/// contemporary music and jazz.
///
/// **Degrees**: Root (R), Perfect Fourth (4), Perfect Fifth (5)
/// **Quality**: Suspended 4th (suspended, unresolved sound)
/// **Display**: R-4-5
pub const SUSPENDED_FOURTH: Chord = ChordBuilder::with_root()
    .set_degree(FOURTH)
    .set_degree(FIFTH)
    .build();

/// Added second chord constant
///
/// A added second chord consists of a root, perfect second, and perfect fifth.
/// This chord has a bright, tense sound and is commonly used in functional harmony.
///
/// **Degrees**: Root (R), Perfect Second (2), Major Third (3), Perfect Fifth (5)
/// **Quality**: Added 2nd (bright, tense sound)
/// **Display**: R-2-3-5
pub const ADDED_SECOND: Chord = ChordBuilder::with_root()
    .set_degree(SECOND)
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .build();

/// Added ninth chord constant
///
/// A added ninth chord consists of a root, major third, perfect fifth,
/// and ninth. This chord has a bright, tense sound and is commonly used
/// in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Ninth (9)
/// **Quality**: Added 9th (bright, tense sound)
/// **Display**: R-3-5-9
pub const ADDED_NINTH: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(NINTH)
    .build();

/// Added eleventh chord constant
///
/// A added eleventh chord consists of a root, major third, perfect fifth,
/// major seventh, and eleventh. This chord has a bright, tense sound and is commonly used
/// in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Eleventh (11)
/// **Quality**: Added 11th (bright, tense sound)
/// **Display**: R-3-5-11
pub const ADDED_ELEVENTH: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .set_degree(ELEVENTH)
    .build();

/// Dominant seventh flat five chord constant
///
/// A dominant seventh flat five chord consists of a root, major third, flat fifth,
/// and flat seventh. This chord has a strong, bluesy sound and is commonly used
/// as a dominant chord in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Flat Fifth (♭5), Flat Seventh (♭7)
/// **Quality**: Dominant 7th flat 5 (strong, bluesy sound)
/// **Display**: R-3-♭5-♭7
pub const DOMINANT_SEVENTH_FLAT_FIVE: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FLAT_FIFTH)
    .set_degree(FLAT_SEVENTH)
    .build();

/// Dominant seventh sharp five chord constant
///
/// A dominant seventh sharp five chord consists of a root, major third, sharp fifth,
/// and flat seventh. This chord has a strong, bluesy sound and is commonly used
/// as a dominant chord in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Sharp Fifth (♯5), Flat Seventh (♭7)
/// **Quality**: Dominant 7th sharp 5 (strong, bluesy sound)
/// **Display**: R-3-♯5-♭7
pub const DOMINANT_SEVENTH_SHARP_FIVE: Chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(SHARP_FIFTH)
    .set_degree(FLAT_SEVENTH)
    .build();

/// A fluent builder for constructing `Chord` instances
///
/// `ChordBuilder` provides a convenient way to construct chords using a fluent interface.
/// It uses a `U4Vec16Builder` internally to efficiently build chord structures with
/// up to 16 degrees, where each degree can have its own accidental modification.
///
/// The builder pattern allows for method chaining, making it easy to construct
/// complex chords in a readable and maintainable way.
///
/// # Examples
///
/// ```rust
/// use muzze_std::{ChordBuilder, ROOT, THIRD, FIFTH, FLAT_THIRD, SHARP_FIFTH, FLAT_SEVENTH};
///
/// // Build a major triad
/// let major_triad = ChordBuilder::with_root()
///     .set_degree(THIRD)
///     .set_degree(FIFTH)
///     .build();
///
/// // Build a minor triad with augmented fifth
/// let minor_augmented = ChordBuilder::with_root()
///     .set_degree(FLAT_THIRD)
///     .set_degree(SHARP_FIFTH)
///     .build();
///
/// // Build a complex jazz chord
/// let jazz_chord = ChordBuilder::with_root()
///     .set_degree(THIRD)
///     .set_degree(SHARP_FIFTH)
///     .set_degree(FLAT_SEVENTH)
///     .build();
/// ```
///
/// # Performance
///
/// - **Const construction**: All methods are const, allowing compile-time evaluation
/// - **Efficient storage**: Uses 64 bits total (16 degrees × 4 bits each)
/// - **No allocations**: Stack-only operations with no heap allocations
/// - **Method chaining**: Fluent interface for readable chord construction
pub struct ChordBuilder(U4Vec16Builder);

impl ChordBuilder {
    /// Creates a new `ChordBuilder` with a root degree
    ///
    /// This method initializes a new chord builder with the root degree (1st degree)
    /// set to natural. This provides a sensible default starting point for chord
    /// construction, as most chords include the root note.
    ///
    /// # Returns
    /// A new `ChordBuilder` instance with the root degree set to natural
    ///
    /// # Example
    /// ```rust
    /// use muzze_std::{ChordBuilder, ROOT};
    ///
    /// let builder = ChordBuilder::with_root();
    /// let chord = builder.build();
    ///
    /// // The root degree should be set
    /// let degrees: Vec<_> = chord.degrees().collect();
    /// assert_eq!(degrees[0], ROOT);
    /// ```
    #[inline]
    pub const fn with_root() -> Self {
        Self(U4Vec16Builder::new()).set_degree(ROOT)
    }

    /// Sets a degree in the chord being built
    ///
    /// This method adds or modifies a degree in the chord. If the same degree
    /// is set multiple times, the last setting will overwrite the previous one.
    /// The degree number is used to determine the position in the chord (1-16),
    /// and the accidental determines the pitch modification for that degree.
    ///
    /// # Arguments
    /// * `degree` - The `Degree` to set in the chord, containing the degree number
    ///   and accidental modification
    ///
    /// # Returns
    /// A new `ChordBuilder` instance with the specified degree set
    ///
    /// # Example
    /// ```rust
    /// use muzze_std::{ChordBuilder, ROOT, THIRD, FLAT_THIRD};
    ///
    /// let builder = ChordBuilder::with_root()
    ///     .set_degree(THIRD)     // Set major third (3rd degree)
    ///     .set_degree(FLAT_THIRD); // Overwrite with minor third
    ///
    /// let chord = builder.build();
    /// let degrees: Vec<_> = chord.degrees().collect();
    /// assert_eq!(degrees[0], ROOT);        // Root is still set
    /// assert_eq!(degrees[1], FLAT_THIRD);  // Third is now minor
    /// ```
    #[inline]
    pub const fn set_degree(self, degree: Degree) -> Self {
        Self(
            self.0
                .set_item(degree.degree as usize - 1, degree.accidental as u8),
        )
    }

    /// Builds the final `Chord` from the builder
    ///
    /// This method consumes the builder and returns the constructed `Chord`.
    /// After calling this method, the builder can no longer be used.
    ///
    /// # Returns
    /// A new `Chord` instance with all the degrees that were set during construction
    ///
    /// # Example
    /// ```rust
    /// use muzze_std::{ChordBuilder, ROOT, THIRD, FIFTH};
    ///
    /// let chord = ChordBuilder::with_root()
    ///     .set_degree(THIRD)
    ///     .set_degree(FIFTH)
    ///     .build(); // Builder is consumed here
    ///
    /// // chord is now ready to use
    /// let degrees: Vec<_> = chord.degrees().collect();
    /// assert_eq!(degrees[0], ROOT);
    /// assert_eq!(degrees[1], THIRD);
    /// assert_eq!(degrees[2], FIFTH);
    /// ```
    #[inline]
    pub const fn build(self) -> Chord {
        Chord::new(self.0.build())
    }
}

impl Default for ChordBuilder {
    /// Creates a default `ChordBuilder` with a root degree
    ///
    /// This implementation provides a sensible default for `ChordBuilder`
    /// by creating a new builder with the root degree set to natural.
    /// This is equivalent to calling `ChordBuilder::new()`.
    ///
    /// # Returns
    /// A new `ChordBuilder` instance with the root degree set to natural
    ///
    /// # Example
    /// ```rust
    /// use muzze_std::ChordBuilder;
    ///
    /// let builder = ChordBuilder::default();
    /// let chord = builder.build();
    ///
    /// // The root degree should be set
    /// let degrees: Vec<_> = chord.degrees().collect();
    /// assert_eq!(degrees[0], muzze_std::ROOT);
    /// ```
    #[inline]
    fn default() -> Self {
        Self::with_root()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // DegreeAccidental tests
    #[test]
    fn test_degree_accidental_display() {
        assert_eq!(format!("{DEGREE_NATURAL}"), "");
        assert_eq!(format!("{DEGREE_FLAT}"), "♭");
        assert_eq!(format!("{DEGREE_DOUBLEFLAT}"), "♭♭");
        assert_eq!(format!("{DEGREE_SHARP}"), "♯");
    }

    #[test]
    fn test_degree_accidental_from_u8() {
        assert_eq!(DegreeAccidental::from(1), DEGREE_NATURAL);
        assert_eq!(DegreeAccidental::from(2), DEGREE_FLAT);
        assert_eq!(DegreeAccidental::from(3), DEGREE_DOUBLEFLAT);
        assert_eq!(DegreeAccidental::from(4), DEGREE_SHARP);
    }

    #[test]
    #[should_panic(expected = "Invalid degree accidental value: 5")]
    fn test_degree_accidental_from_invalid_u8() {
        let _ = DegreeAccidental::from(5);
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
            let accidental = DegreeAccidental::from(i);
            assert_eq!(u8::from(accidental), i);
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

    // Chord tests
    #[test]
    fn test_chord_new() {
        let builder = U4Vec16Builder::new()
            .set_item(0, DEGREE_NATURAL as u8)
            .set_item(2, DEGREE_NATURAL as u8)
            .set_item(4, DEGREE_NATURAL as u8);
        let chord = Chord::new(builder.build());

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees.len(), 3);
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
    }

    #[test]
    fn test_chord_degrees_iterator() {
        let builder = U4Vec16Builder::new()
            .set_item(0, DEGREE_NATURAL as u8)
            .set_item(2, DEGREE_FLAT as u8)
            .set_item(4, DEGREE_SHARP as u8);
        let chord = Chord::new(builder.build());

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], SHARP_FIFTH);
    }

    // ChordBuilder tests
    #[test]
    fn test_chord_builder_new() {
        let builder = ChordBuilder::with_root();
        let chord = builder.build();
        let degrees: Vec<Degree> = chord.degrees().collect();

        // Should only have the root degree
        assert_eq!(degrees.len(), 1);
        assert_eq!(degrees[0], ROOT);
    }

    #[test]
    fn test_chord_builder_default() {
        let builder = ChordBuilder::default();
        let chord = builder.build();
        let degrees: Vec<Degree> = chord.degrees().collect();

        // All degrees should be natural (0)
        for degree in degrees {
            assert_eq!(degree.accidental, DEGREE_NATURAL);
        }
    }

    #[test]
    fn test_chord_builder_set_degree() {
        let builder = ChordBuilder::with_root()
            .set_degree(THIRD)
            .set_degree(FIFTH);
        let chord = builder.build();

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
    }

    #[test]
    fn test_chord_builder_set_degree_chaining() {
        let builder = ChordBuilder::with_root()
            .set_degree(FLAT_THIRD)
            .set_degree(SHARP_FIFTH)
            .set_degree(FLAT_SEVENTH);
        let chord = builder.build();

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], SHARP_FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
    }

    #[test]
    fn test_chord_builder_set_degree_overwrite() {
        let builder = ChordBuilder::with_root()
            .set_degree(THIRD)
            .set_degree(FLAT_THIRD); // This should overwrite the previous THIRD
        let chord = builder.build();

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[1], FLAT_THIRD);
    }

    #[test]
    fn test_chord_builder_build_consumes() {
        let builder = ChordBuilder::with_root();
        let _chord = builder.build();
        // builder is consumed, so we can't use it again
    }

    // Predefined chord tests
    #[test]
    fn test_major_triad() {
        let degrees: Vec<Degree> = MAJOR_TRIAD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
    }

    #[test]
    fn test_minor_triad() {
        let degrees: Vec<Degree> = MINOR_TRIAD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], FIFTH);
    }

    #[test]
    fn test_diminished_triad() {
        let degrees: Vec<Degree> = DIMINISHED_TRIAD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], FLAT_FIFTH);
    }

    #[test]
    fn test_augmented_triad() {
        let degrees: Vec<Degree> = AUGMENTED_TRIAD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], SHARP_FIFTH);
    }

    #[test]
    fn test_major_seventh_chord() {
        let degrees: Vec<Degree> = MAJOR_SEVENTH_CHORD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], SEVENTH);
    }

    #[test]
    fn test_minor_seventh_chord() {
        let degrees: Vec<Degree> = MINOR_SEVENTH_CHORD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
    }

    #[test]
    fn test_dominant_seventh() {
        let degrees: Vec<Degree> = DOMINANT_SEVENTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
    }

    #[test]
    fn test_half_diminished_seventh() {
        let degrees: Vec<Degree> = HALF_DIMINISHED_SEVENTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], FLAT_FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
    }

    #[test]
    fn test_diminished_seventh() {
        let degrees: Vec<Degree> = DIMINISHED_SEVENTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], FLAT_FIFTH);
        assert_eq!(degrees[3], DOUBLEFLAT_SEVENTH);
    }

    #[test]
    fn test_augmented_seventh() {
        let degrees: Vec<Degree> = AUGMENTED_SEVENTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], SHARP_FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
    }

    #[test]
    fn test_suspended_fourth() {
        let degrees: Vec<Degree> = SUSPENDED_FOURTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FOURTH);
        assert_eq!(degrees[2], FIFTH);
    }

    #[test]
    fn test_minor_major_seventh() {
        let degrees: Vec<Degree> = MINOR_MAJOR_SEVENTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], SEVENTH);
    }

    #[test]
    fn test_sixth_chord() {
        let degrees: Vec<Degree> = SIXTH_CHORD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], SIXTH);
    }

    #[test]
    fn test_sixth_minor_chord() {
        let degrees: Vec<Degree> = SIXTH_MINOR_CHORD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], SIXTH);
    }

    #[test]
    fn test_sixth_ninth_chord() {
        let degrees: Vec<Degree> = SIXTH_NINTH_CHORD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], SIXTH);
        assert_eq!(degrees[4], NINTH);
    }

    #[test]
    fn test_fifth_chord() {
        let degrees: Vec<Degree> = FIFTH_CHORD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FIFTH);
    }

    #[test]
    fn test_dominant_ninth() {
        let degrees: Vec<Degree> = DOMINANT_NINTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
        assert_eq!(degrees[4], NINTH);
    }

    #[test]
    fn test_minor_ninth() {
        let degrees: Vec<Degree> = MINOR_NINTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
        assert_eq!(degrees[4], NINTH);
    }

    #[test]
    fn test_major_ninth() {
        let degrees: Vec<Degree> = MAJOR_NINTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], SEVENTH);
        assert_eq!(degrees[4], NINTH);
    }

    #[test]
    fn test_eleventh_chord() {
        let degrees: Vec<Degree> = ELEVENTH_CHORD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], SEVENTH);
        assert_eq!(degrees[4], NINTH);
        assert_eq!(degrees[5], ELEVENTH);
    }

    #[test]
    fn test_minor_eleventh() {
        let degrees: Vec<Degree> = MINOR_ELEVENTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
        assert_eq!(degrees[4], NINTH);
        assert_eq!(degrees[5], ELEVENTH);
    }

    #[test]
    fn test_major_eleventh() {
        let degrees: Vec<Degree> = MAJOR_ELEVENTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], SEVENTH);
        assert_eq!(degrees[4], NINTH);
        assert_eq!(degrees[5], ELEVENTH);
    }

    #[test]
    fn test_thirteenth_chord() {
        let degrees: Vec<Degree> = THIRTEENTH_CHORD.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
        assert_eq!(degrees[4], NINTH);
        assert_eq!(degrees[5], ELEVENTH);
        assert_eq!(degrees[6], THIRTEENTH);
    }

    #[test]
    fn test_minor_thirteenth() {
        let degrees: Vec<Degree> = MINOR_THIRTEENTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
        assert_eq!(degrees[4], NINTH);
        assert_eq!(degrees[5], ELEVENTH);
        assert_eq!(degrees[6], THIRTEENTH);
    }

    #[test]
    fn test_major_thirteenth() {
        let degrees: Vec<Degree> = MAJOR_THIRTEENTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], SEVENTH);
        assert_eq!(degrees[4], NINTH);
        assert_eq!(degrees[5], ELEVENTH);
        assert_eq!(degrees[6], THIRTEENTH);
    }

    #[test]
    fn test_suspended_second() {
        let degrees: Vec<Degree> = SUSPENDED_SECOND.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], SECOND);
        assert_eq!(degrees[2], FIFTH);
    }

    #[test]
    fn test_added_second() {
        let degrees: Vec<Degree> = ADDED_SECOND.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], SECOND);
        assert_eq!(degrees[2], THIRD);
        assert_eq!(degrees[3], FIFTH);
    }

    #[test]
    fn test_added_ninth() {
        let degrees: Vec<Degree> = ADDED_NINTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], NINTH);
    }

    #[test]
    fn test_added_eleventh() {
        let degrees: Vec<Degree> = ADDED_ELEVENTH.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(degrees[3], ELEVENTH);
    }

    #[test]
    fn test_dominant_seventh_flat_five() {
        let degrees: Vec<Degree> = DOMINANT_SEVENTH_FLAT_FIVE.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FLAT_FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
    }

    #[test]
    fn test_dominant_seventh_sharp_five() {
        let degrees: Vec<Degree> = DOMINANT_SEVENTH_SHARP_FIVE.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], SHARP_FIFTH);
        assert_eq!(degrees[3], FLAT_SEVENTH);
    }

    // Edge case tests
    #[test]
    fn test_chord_builder_edge_values() {
        let builder = ChordBuilder::with_root()
            .set_degree(Degree::new(1, DEGREE_NATURAL))
            .set_degree(Degree::new(16, DEGREE_SHARP));
        let chord = builder.build();

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], Degree::new(16, DEGREE_SHARP));
    }

    #[test]
    fn test_chord_builder_all_accidentals() {
        let builder = ChordBuilder::with_root()
            .set_degree(Degree::new(2, DEGREE_FLAT))
            .set_degree(Degree::new(3, DEGREE_DOUBLEFLAT))
            .set_degree(Degree::new(4, DEGREE_SHARP));
        let chord = builder.build();

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], Degree::new(2, DEGREE_FLAT));
        assert_eq!(degrees[2], Degree::new(3, DEGREE_DOUBLEFLAT));
        assert_eq!(degrees[3], Degree::new(4, DEGREE_SHARP));
    }

    #[test]
    fn test_chord_builder_complex_pattern() {
        let builder = ChordBuilder::with_root()
            .set_degree(FLAT_THIRD)
            .set_degree(FOURTH)
            .set_degree(SHARP_FIFTH)
            .set_degree(FLAT_SEVENTH);
        let chord = builder.build();

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], FOURTH);
        assert_eq!(degrees[3], SHARP_FIFTH);
        assert_eq!(degrees[4], FLAT_SEVENTH);
    }

    #[test]
    fn test_chord_builder_sparse_pattern() {
        let builder = ChordBuilder::with_root()
            .set_degree(SHARP_FIFTH)
            .set_degree(FLAT_SEVENTH);
        let chord = builder.build();

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], SHARP_FIFTH);
        assert_eq!(degrees[2], FLAT_SEVENTH);
    }

    // Display implementation tests
    #[test]
    fn test_chord_display_empty_chord() {
        let chord = ChordBuilder::with_root().build();
        let display = format!("{chord}");
        assert_eq!(display, "R");
    }

    #[test]
    fn test_chord_display_major_triad() {
        let display = format!("{MAJOR_TRIAD}");
        assert_eq!(display, "R-3-5");
    }

    #[test]
    fn test_chord_display_minor_triad() {
        let display = format!("{MINOR_TRIAD}");
        assert_eq!(display, "R-♭3-5");
    }

    #[test]
    fn test_chord_display_diminished_triad() {
        let display = format!("{DIMINISHED_TRIAD}");
        assert_eq!(display, "R-♭3-♭5");
    }

    #[test]
    fn test_chord_display_augmented_triad() {
        let display = format!("{AUGMENTED_TRIAD}");
        assert_eq!(display, "R-3-♯5");
    }

    #[test]
    fn test_chord_display_major_seventh_chord() {
        let display = format!("{MAJOR_SEVENTH_CHORD}");
        assert_eq!(display, "R-3-5-7");
    }

    #[test]
    fn test_chord_display_minor_seventh_chord() {
        let display = format!("{MINOR_SEVENTH_CHORD}");
        assert_eq!(display, "R-♭3-5-♭7");
    }

    #[test]
    fn test_chord_display_dominant_seventh() {
        let display = format!("{DOMINANT_SEVENTH}");
        assert_eq!(display, "R-3-5-♭7");
    }

    #[test]
    fn test_chord_display_half_diminished_seventh() {
        let display = format!("{HALF_DIMINISHED_SEVENTH}");
        assert_eq!(display, "R-♭3-♭5-♭7");
    }

    #[test]
    fn test_chord_display_diminished_seventh() {
        let display = format!("{DIMINISHED_SEVENTH}");
        assert_eq!(display, "R-♭3-♭5-♭♭7");
    }

    #[test]
    fn test_chord_display_augmented_seventh() {
        let display = format!("{AUGMENTED_SEVENTH}");
        assert_eq!(display, "R-3-♯5-♭7");
    }

    #[test]
    fn test_chord_display_suspended_fourth() {
        let display = format!("{SUSPENDED_FOURTH}");
        assert_eq!(display, "R-4-5");
    }

    #[test]
    fn test_chord_display_minor_major_seventh() {
        let display = format!("{MINOR_MAJOR_SEVENTH}");
        assert_eq!(display, "R-♭3-5-7");
    }

    #[test]
    fn test_chord_display_sixth_chord() {
        let display = format!("{SIXTH_CHORD}");
        assert_eq!(display, "R-3-5-6");
    }

    #[test]
    fn test_chord_display_sixth_minor_chord() {
        let display = format!("{SIXTH_MINOR_CHORD}");
        assert_eq!(display, "R-♭3-5-6");
    }

    #[test]
    fn test_chord_display_sixth_ninth_chord() {
        let display = format!("{SIXTH_NINTH_CHORD}");
        assert_eq!(display, "R-3-5-6-9");
    }

    #[test]
    fn test_chord_display_fifth_chord() {
        let display = format!("{FIFTH_CHORD}");
        assert_eq!(display, "R-5");
    }

    #[test]
    fn test_chord_display_dominant_ninth() {
        let display = format!("{DOMINANT_NINTH}");
        assert_eq!(display, "R-3-5-♭7-9");
    }

    #[test]
    fn test_chord_display_minor_ninth() {
        let display = format!("{MINOR_NINTH}");
        assert_eq!(display, "R-♭3-5-♭7-9");
    }

    #[test]
    fn test_chord_display_major_ninth() {
        let display = format!("{MAJOR_NINTH}");
        assert_eq!(display, "R-3-5-7-9");
    }

    #[test]
    fn test_chord_display_eleventh_chord() {
        let display = format!("{ELEVENTH_CHORD}");
        assert_eq!(display, "R-3-5-7-9-11");
    }

    #[test]
    fn test_chord_display_minor_eleventh() {
        let display = format!("{MINOR_ELEVENTH}");
        assert_eq!(display, "R-♭3-5-♭7-9-11");
    }

    #[test]
    fn test_chord_display_major_eleventh() {
        let display = format!("{MAJOR_ELEVENTH}");
        assert_eq!(display, "R-3-5-7-9-11");
    }

    #[test]
    fn test_chord_display_thirteenth_chord() {
        let display = format!("{THIRTEENTH_CHORD}");
        assert_eq!(display, "R-3-5-♭7-9-11-13");
    }

    #[test]
    fn test_chord_display_minor_thirteenth() {
        let display = format!("{MINOR_THIRTEENTH}");
        assert_eq!(display, "R-♭3-5-♭7-9-11-13");
    }

    #[test]
    fn test_chord_display_major_thirteenth() {
        let display = format!("{MAJOR_THIRTEENTH}");
        assert_eq!(display, "R-3-5-7-9-11-13");
    }

    #[test]
    fn test_chord_display_suspended_second() {
        let display = format!("{SUSPENDED_SECOND}");
        assert_eq!(display, "R-2-5");
    }

    #[test]
    fn test_chord_display_added_second() {
        let display = format!("{ADDED_SECOND}");
        assert_eq!(display, "R-2-3-5");
    }

    #[test]
    fn test_chord_display_added_ninth() {
        let display = format!("{ADDED_NINTH}");
        assert_eq!(display, "R-3-5-9");
    }

    #[test]
    fn test_chord_display_added_eleventh() {
        let display = format!("{ADDED_ELEVENTH}");
        assert_eq!(display, "R-3-5-11");
    }

    #[test]
    fn test_chord_display_dominant_seventh_flat_five() {
        let display = format!("{DOMINANT_SEVENTH_FLAT_FIVE}");
        assert_eq!(display, "R-3-♭5-♭7");
    }

    #[test]
    fn test_chord_display_dominant_seventh_sharp_five() {
        let display = format!("{DOMINANT_SEVENTH_SHARP_FIVE}");
        assert_eq!(display, "R-3-♯5-♭7");
    }

    #[test]
    fn test_chord_display_custom_chord() {
        let chord = ChordBuilder::with_root()
            .set_degree(Degree::new(2, DEGREE_SHARP))
            .set_degree(FLAT_THIRD)
            .set_degree(Degree::new(4, DEGREE_DOUBLEFLAT))
            .set_degree(SHARP_FIFTH)
            .set_degree(FLAT_SEVENTH)
            .build();

        let display = format!("{chord}");
        assert_eq!(display, "R-♯2-♭3-♭♭4-♯5-♭7");
    }

    #[test]
    fn test_chord_display_all_accidentals() {
        let chord = ChordBuilder::with_root()
            .set_degree(Degree::new(2, DEGREE_FLAT))
            .set_degree(Degree::new(3, DEGREE_DOUBLEFLAT))
            .set_degree(Degree::new(4, DEGREE_SHARP))
            .build();

        let display = format!("{chord}");
        assert_eq!(display, "R-♭2-♭♭3-♯4");
    }

    #[test]
    fn test_chord_display_edge_degrees() {
        let chord = ChordBuilder::with_root()
            .set_degree(Degree::new(16, DEGREE_SHARP))
            .build();

        let display = format!("{chord}");
        assert_eq!(display, "R-♯16");
    }

    #[test]
    fn test_chord_display_complex_jazz_chord() {
        let chord = ChordBuilder::with_root()
            .set_degree(Degree::new(2, DEGREE_SHARP))
            .set_degree(THIRD)
            .set_degree(Degree::new(4, DEGREE_SHARP))
            .set_degree(SHARP_FIFTH)
            .set_degree(Degree::new(6, DEGREE_FLAT))
            .set_degree(FLAT_SEVENTH)
            .set_degree(Degree::new(9, DEGREE_NATURAL))
            .build();

        let display = format!("{chord}");
        assert_eq!(display, "R-♯2-3-♯4-♯5-♭6-♭7-9");
    }

    #[test]
    fn test_chord_display_sparse_chord() {
        let chord = ChordBuilder::with_root()
            .set_degree(SHARP_FIFTH)
            .set_degree(FLAT_SEVENTH)
            .build();

        let display = format!("{chord}");
        assert_eq!(display, "R-♯5-♭7");
    }

    #[test]
    fn test_chord_display_alterations() {
        let chord = ChordBuilder::with_root()
            .set_degree(THIRD)
            .set_degree(FLAT_FIFTH)
            .set_degree(SHARP_FIFTH) // This should overwrite the flat fifth
            .set_degree(SEVENTH)
            .set_degree(FLAT_SEVENTH) // This should overwrite the natural seventh
            .build();

        let display = format!("{chord}");
        assert_eq!(display, "R-3-♯5-♭7");
    }

    #[test]
    fn test_chord_display_unicode_symbols() {
        let chord = ChordBuilder::with_root()
            .set_degree(Degree::new(2, DEGREE_FLAT))
            .set_degree(Degree::new(3, DEGREE_DOUBLEFLAT))
            .set_degree(Degree::new(4, DEGREE_SHARP))
            .build();

        let display = format!("{chord}");
        // Verify that Unicode symbols are properly displayed
        assert!(display.contains("♭")); // Flat symbol
        assert!(display.contains("♭♭")); // Double flat symbol
        assert!(display.contains("♯")); // Sharp symbol
        assert!(!display.contains("b")); // Should not contain ASCII 'b'
        assert!(!display.contains("#")); // Should not contain ASCII '#'
    }
}
