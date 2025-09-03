use std::fmt::Display;

use crate::{
    Degree, DegreeAccidental, DOUBLEFLAT_SEVENTH, ELEVENTH, FIFTH, FLAT_FIFTH, FLAT_SEVENTH,
    FLAT_THIRD, FOURTH, NINTH, ROOT, SECOND, SEVENTH, SHARP_FIFTH, SIXTH, THIRD, THIRTEENTH,
};
use muzze_bitflags::{u4vec16::U4Vec16Builder, U4Vec16};

/// Macro to generate chord constants with consistent documentation and structure
///
/// This macro reduces code duplication by generating chord constants with:
/// - Consistent documentation format
/// - Proper chord construction using ChordBuilder
/// - Standardized naming conventions
///
/// The macro takes the following parameters:
/// - `$name`: The constant name (e.g., `MAJOR_TRIAD`)
/// - `$display_name`: The display name for the chord
/// - `$description`: The description text
/// - `$quality`: The quality description
/// - `$display_format`: The display format string
/// - `[$($degree:expr),*]`: Array of degree constants
macro_rules! chord_const {
    (
        $name:ident,
        $display_name:literal,
        $description:literal,
        $quality:literal,
        $display_format:literal,
        [$($degree:expr),*]
    ) => {
        /// $description
        ///
        /// **Degrees**: $display_format
        /// **Quality**: $quality
        /// **Display**: $display_format
        pub const $name: Chord = ChordBuilder::with_root($display_name)
            $(.set_degree($degree))*
            .build();
    };
}

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
/// let major_triad = ChordBuilder::with_root("my chord")
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
pub struct Chord {
    degrees: U4Vec16,
    name: &'static str,
}

impl Chord {
    /// Creates a new `Chord` from a `U4Vec16` containing degree accidentals
    ///
    /// # Arguments
    /// * `name` - The name of the chord
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
    /// let chord = ChordBuilder::with_root("m")
    ///     .set_degree(FLAT_THIRD)
    ///     .build();
    /// ```
    #[inline]
    const fn new(name: &'static str, degrees: U4Vec16) -> Self {
        Self { name, degrees }
    }

    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
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
    /// let chord = ChordBuilder::with_root("my chord")
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
        self.degrees
            .iter_items()
            .enumerate()
            .filter_map(|(index, acc)| {
                if acc == 0 {
                    None
                } else {
                    let index = index as u8 + 1;
                    let acc = DegreeAccidental::try_from(acc).unwrap();
                    let degree = Degree::new(index, acc);
                    Some(degree)
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
            .degrees
            .iter_items()
            .enumerate()
            .filter_map(|(index, accidental)| {
                if accidental == 0 {
                    None
                } else {
                    let index = index as u8 + 1;
                    let acc = DegreeAccidental::try_from(accidental).unwrap();
                    let degree = Degree::new(index, acc);
                    Some(degree.to_string())
                }
            })
            .collect::<Vec<_>>();
        write!(f, "{}", xs.join("-"))
    }
}

chord_const!(
    MAJOR_TRIAD,
    "major triad",
    "A major triad consists of a root, major third, and perfect fifth. This is the most fundamental major chord and forms the basis of major harmony in Western music.",
    "Major (bright, stable sound)",
    "R-3-5",
    [THIRD, FIFTH]
);

chord_const!(
    MINOR_TRIAD,
    "minor triad",
    "A minor triad consists of a root, minor third, and perfect fifth. This is the most fundamental minor chord and forms the basis of minor harmony in Western music.",
    "Minor (dark, melancholic sound)",
    "R-♭3-5",
    [FLAT_THIRD, FIFTH]
);

chord_const!(
    DIMINISHED_TRIAD,
    "diminished triad",
    "A diminished triad consists of a root, minor third, and diminished fifth. This chord creates a very tense, unstable sound and is commonly used as a passing chord or in diminished harmony.",
    "Diminished (tense, unstable sound)",
    "R-♭3-♭5",
    [FLAT_THIRD, FLAT_FIFTH]
);

chord_const!(
    AUGMENTED_TRIAD,
    "augmented triad",
    "An augmented triad consists of a root, major third, and augmented fifth. This chord creates a bright, tense sound and is commonly used in augmented harmony and as a passing chord.",
    "Augmented (bright, tense sound)",
    "R-3-♯5",
    [THIRD, SHARP_FIFTH]
);

chord_const!(
    MAJOR_SEVENTH_CHORD,
    "major seventh chord",
    "A major seventh chord consists of a root, major third, perfect fifth, and major seventh. This chord has a sophisticated, jazzy sound and is commonly used in jazz and contemporary music.",
    "Major 7th (sophisticated, jazzy sound)",
    "R-3-5-7",
    [THIRD, FIFTH, SEVENTH]
);

chord_const!(
    MINOR_SEVENTH_CHORD,
    "minor seventh chord",
    "A minor seventh chord consists of a root, minor third, perfect fifth, and minor seventh. This chord has a mellow, bluesy sound and is commonly used in jazz, blues, and contemporary music.",
    "Minor 7th (mellow, bluesy sound)",
    "R-♭3-5-♭7",
    [FLAT_THIRD, FIFTH, FLAT_SEVENTH]
);

chord_const!(
    DOMINANT_SEVENTH,
    "dominant seventh chord",
    "A dominant seventh chord consists of a root, major third, perfect fifth, and minor seventh. This chord has a strong, bluesy sound and is commonly used as a dominant chord in functional harmony.",
    "Dominant 7th (strong, bluesy sound)",
    "R-3-5-♭7",
    [THIRD, FIFTH, FLAT_SEVENTH]
);

chord_const!(
    HALF_DIMINISHED_SEVENTH,
    "half-diminished seventh chord",
    "A half-diminished seventh chord consists of a root, minor third, diminished fifth, and minor seventh. This chord has a tense, unstable sound and is commonly used as a minor ii chord in functional harmony.",
    "Half-diminished 7th (tense, unstable sound)",
    "R-♭3-♭5-♭7",
    [FLAT_THIRD, FLAT_FIFTH, FLAT_SEVENTH]
);

chord_const!(
    DIMINISHED_SEVENTH,
    "diminished seventh chord",
    "A diminished seventh chord consists of a root, minor third, diminished fifth, and diminished seventh. This chord has a very tense, unstable sound and is commonly used as a passing chord or in diminished harmony.",
    "Diminished 7th (very tense, unstable sound)",
    "R-♭3-♭5-♭♭7",
    [FLAT_THIRD, FLAT_FIFTH, DOUBLEFLAT_SEVENTH]
);

chord_const!(
    AUGMENTED_SEVENTH,
    "augmented seventh chord",
    "An augmented seventh chord consists of a root, major third, augmented fifth, and minor seventh. This chord has a bright, tense sound and is commonly used in augmented harmony and as a passing chord.",
    "Augmented 7th (bright, tense sound)",
    "R-3-♯5-♭7",
    [THIRD, SHARP_FIFTH, FLAT_SEVENTH]
);

chord_const!(
    MINOR_MAJOR_SEVENTH,
    "minor major seventh chord",
    "A minor major seventh chord consists of a root, minor third, perfect fifth, and major seventh. This chord has a mellow, bluesy sound and is commonly used in jazz, blues, and contemporary music.",
    "Minor major 7th (mellow, bluesy sound)",
    "R-♭3-5-7",
    [FLAT_THIRD, FIFTH, SEVENTH]
);

chord_const!(
    SIXTH_CHORD,
    "sixth chord",
    "A sixth chord consists of a root, major third, and sixth. This chord has a mellow, bluesy sound and is commonly used in jazz, blues, and contemporary music.",
    "Sixth (mellow, bluesy sound)",
    "R-3-5-6",
    [THIRD, FIFTH, SIXTH]
);

/// Sixth minor chord constant
///
/// A sixth minor chord consists of a root, minor third, and sixth. This chord has a
/// mellow, bluesy sound and is commonly used in jazz, blues, and contemporary music.
///
/// **Degrees**: Root (R), Minor Third (♭3), Perfect Fifth (5), Sixth (6)
/// **Quality**: Sixth minor (mellow, bluesy sound)
/// **Display**: R-♭3-5-6
pub const SIXTH_MINOR_CHORD: Chord = ChordBuilder::with_root("sixth minor chord")
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
pub const SIXTH_NINTH_CHORD: Chord = ChordBuilder::with_root("sixth ninth chord")
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
pub const FIFTH_CHORD: Chord = ChordBuilder::with_root("fifth chord")
    .set_degree(FIFTH)
    .build();

/// Dominant ninth chord constant
///
/// A dominant ninth chord consists of a root, major third, perfect fifth,
/// and minor seventh. This chord has a strong, bluesy sound and is commonly used
/// as a dominant chord in functional harmony.
///
/// **Degrees**: Root (R), Major Third (3), Perfect Fifth (5), Minor Seventh (♭7), Ninth (9)
/// **Quality**: Dominant 9th (strong, bluesy sound)
/// **Display**: R-3-5-♭7-9
pub const DOMINANT_NINTH: Chord = ChordBuilder::with_root("dominant ninth chord")
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
pub const MINOR_NINTH: Chord = ChordBuilder::with_root("minor ninth chord")
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
pub const MAJOR_NINTH: Chord = ChordBuilder::with_root("major ninth chord")
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
pub const ELEVENTH_CHORD: Chord = ChordBuilder::with_root("eleventh chord")
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
pub const MINOR_ELEVENTH: Chord = ChordBuilder::with_root("minor eleventh chord")
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
pub const THIRTEENTH_CHORD: Chord = ChordBuilder::with_root("thirteenth chord")
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
pub const MINOR_THIRTEENTH: Chord = ChordBuilder::with_root("minor thirteenth chord")
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
pub const MAJOR_THIRTEENTH: Chord = ChordBuilder::with_root("major thirteenth chord")
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
pub const MAJOR_ELEVENTH: Chord = ChordBuilder::with_root("major eleventh chord")
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
pub const SUSPENDED_SECOND: Chord = ChordBuilder::with_root("suspended second chord")
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
pub const SUSPENDED_FOURTH: Chord = ChordBuilder::with_root("suspended fourth chord")
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
pub const ADDED_SECOND: Chord = ChordBuilder::with_root("added second chord")
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
pub const ADDED_NINTH: Chord = ChordBuilder::with_root("added ninth chord")
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
pub const ADDED_ELEVENTH: Chord = ChordBuilder::with_root("added eleventh chord")
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
pub const DOMINANT_SEVENTH_FLAT_FIVE: Chord =
    ChordBuilder::with_root("dominant seventh flat five chord")
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
pub const DOMINANT_SEVENTH_SHARP_FIVE: Chord =
    ChordBuilder::with_root("dominant seventh sharp five chord")
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
/// let major_triad = ChordBuilder::with_root("my chord")
///     .set_degree(THIRD)
///     .set_degree(FIFTH)
///     .build();
///
/// // Build a minor triad with augmented fifth
/// let minor_augmented = ChordBuilder::with_root("my chord")
///     .set_degree(FLAT_THIRD)
///     .set_degree(SHARP_FIFTH)
///     .build();
///
/// // Build a complex jazz chord
/// let jazz_chord = ChordBuilder::with_root("my chord")
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
pub struct ChordBuilder {
    name: &'static str,
    bldr: U4Vec16Builder,
}

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
    /// let builder = ChordBuilder::with_root("my chord");
    /// let chord = builder.build();
    ///
    /// // The root degree should be set
    /// let degrees: Vec<_> = chord.degrees().collect();
    /// assert_eq!(degrees[0], ROOT);
    /// ```
    #[inline]
    pub const fn with_root(name: &'static str) -> Self {
        let bldr = U4Vec16Builder::new();
        let bldr = Self { name, bldr };
        bldr.set_degree(ROOT)
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
    /// let builder = ChordBuilder::with_root("my chord")
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
        Self {
            name: self.name,
            bldr: self
                .bldr
                .set_item(degree.degree() as usize - 1, degree.accidental() as u8),
        }
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
    /// let chord = ChordBuilder::with_root("my chord")
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
        Chord::new(self.name, self.bldr.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DEGREE_DOUBLEFLAT, DEGREE_FLAT, DEGREE_NATURAL, DEGREE_SHARP};

    // Chord tests
    #[test]
    fn test_chord_new() {
        let builder = U4Vec16Builder::new()
            .set_item(0, DEGREE_NATURAL as u8)
            .set_item(2, DEGREE_NATURAL as u8)
            .set_item(4, DEGREE_NATURAL as u8);
        let chord = Chord::new("test", builder.build());

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees.len(), 3);
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(chord.name, "test");
    }

    #[test]
    fn test_chord_degrees_iterator() {
        let builder = U4Vec16Builder::new()
            .set_item(0, DEGREE_NATURAL as u8)
            .set_item(2, DEGREE_FLAT as u8)
            .set_item(4, DEGREE_SHARP as u8);
        let chord = Chord::new("test", builder.build());

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], FLAT_THIRD);
        assert_eq!(degrees[2], SHARP_FIFTH);
        assert_eq!(chord.name, "test");
    }

    // ChordBuilder tests
    #[test]
    fn test_chord_builder_new() {
        let builder = ChordBuilder::with_root("test");
        let chord = builder.build();
        let degrees: Vec<Degree> = chord.degrees().collect();

        // Should only have the root degree
        assert_eq!(degrees.len(), 1);
        assert_eq!(degrees[0], ROOT);
        assert_eq!(chord.name, "test");
    }

    #[test]
    fn test_chord_builder_set_degree() {
        let builder = ChordBuilder::with_root("test")
            .set_degree(THIRD)
            .set_degree(FIFTH);
        let chord = builder.build();

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], THIRD);
        assert_eq!(degrees[2], FIFTH);
        assert_eq!(chord.name, "test");
    }

    #[test]
    fn test_chord_builder_set_degree_chaining() {
        let builder = ChordBuilder::with_root("test")
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
        let builder = ChordBuilder::with_root("test")
            .set_degree(THIRD)
            .set_degree(FLAT_THIRD); // This should overwrite the previous THIRD
        let chord = builder.build();

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[1], FLAT_THIRD);
    }

    #[test]
    fn test_chord_builder_build_consumes() {
        let builder = ChordBuilder::with_root("test");
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
        let builder = ChordBuilder::with_root("test")
            .set_degree(Degree::new(1, DEGREE_NATURAL))
            .set_degree(Degree::new(16, DEGREE_SHARP));
        let chord = builder.build();

        let degrees: Vec<Degree> = chord.degrees().collect();
        assert_eq!(degrees[0], ROOT);
        assert_eq!(degrees[1], Degree::new(16, DEGREE_SHARP));
    }

    #[test]
    fn test_chord_builder_all_accidentals() {
        let builder = ChordBuilder::with_root("test")
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
        let builder = ChordBuilder::with_root("test")
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
        let builder = ChordBuilder::with_root("test")
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
        let chord = ChordBuilder::with_root("test").build();
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
        let chord = ChordBuilder::with_root("test")
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
        let chord = ChordBuilder::with_root("test")
            .set_degree(Degree::new(2, DEGREE_FLAT))
            .set_degree(Degree::new(3, DEGREE_DOUBLEFLAT))
            .set_degree(Degree::new(4, DEGREE_SHARP))
            .build();

        let display = format!("{chord}");
        assert_eq!(display, "R-♭2-♭♭3-♯4");
    }

    #[test]
    fn test_chord_display_edge_degrees() {
        let chord = ChordBuilder::with_root("test")
            .set_degree(Degree::new(16, DEGREE_SHARP))
            .build();

        let display = format!("{chord}");
        assert_eq!(display, "R-♯16");
    }

    #[test]
    fn test_chord_display_complex_jazz_chord() {
        let chord = ChordBuilder::with_root("test")
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
        let chord = ChordBuilder::with_root("test")
            .set_degree(SHARP_FIFTH)
            .set_degree(FLAT_SEVENTH)
            .build();

        let display = format!("{chord}");
        assert_eq!(display, "R-♯5-♭7");
    }

    #[test]
    fn test_chord_display_alterations() {
        let chord = ChordBuilder::with_root("test")
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
        let chord = ChordBuilder::with_root("test")
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
