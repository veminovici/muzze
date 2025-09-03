# Muzze Standard Library

A Rust library for musical computations and data structures, providing efficient representations of musical concepts using bit vectors and specialized data types.

## Features

- **Scale**: Musical scale representation with predefined scales and builders
- **ScaleBuilder**: Fluent interface for constructing custom scales
- **ScaleStepBuilder**: Build scales using step patterns (whole steps, half steps)
- **Step**: Musical step intervals with semitone values (half steps, whole steps, etc.)
- **Interval**: Musical interval types with standard names and semitone values
- **Accidental**: Musical accidental types with Unicode symbols (sharps, flats, naturals)
- **Chord**: Musical chord representation with predefined chords and builders
- **ChordBuilder**: Fluent interface for constructing custom chords
- **Degree**: Chord degree representation with accidental modifications
- **DegreeAccidental**: Accidental types for chord degrees (natural, flat, sharp, double flat)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
muzze-std = "0.1.0"
```

### Dependencies

`muzze-std` automatically includes `muzze-bitflags` as a dependency, which provides the underlying bit vector types (`BitVec16`, `U4Vec16`, `U4x2`). You can also use `muzze-bitflags` directly if you only need the bit vector functionality:

```toml
[dependencies]
muzze-bitflags = "0.1.0"  # For bit vector operations only
# OR
muzze-std = "0.1.0"       # For complete musical functionality
```

## Quick Start

### Working with Scales

```rust
use muzze_std::{MAJOR, NATURAL_MINOR, ScaleBuilder, ScaleStepBuilder, HALF, WHOLE, MAJOR_SECOND, MAJOR_THIRD, PERFECT_FIFTH, OCTAVE};

// Use predefined scales
let major_scale = MAJOR;
let intervals: Vec<u8> = major_scale.intervals().collect();
// Result: [2, 4, 5, 7, 9, 11, 12] (Major 2nd, Major 3rd, Perfect 4th, Perfect 5th, Major 6th, Major 7th, Octave)

// Build a scale using absolute intervals
let custom_scale = ScaleBuilder::default()
    .set_interval(u8::from(MAJOR_SECOND))  // Major 2nd
    .set_interval(u8::from(MAJOR_THIRD))   // Major 3rd
    .set_interval(u8::from(PERFECT_FIFTH)) // Perfect 5th
    .set_interval(u8::from(OCTAVE))        // Octave
    .build();

// Build a scale using step patterns
let major_scale_built = ScaleStepBuilder::default()
    .add_step(WHOLE)   // Whole step
    .add_step(WHOLE)   // Whole step
    .add_step(HALF)    // Half step
    .add_step(WHOLE)   // Whole step
    .add_step(WHOLE)   // Whole step
    .add_step(WHOLE)   // Whole step
    .add_step(HALF)    // Half step
    .build();

assert_eq!(major_scale_built, MAJOR);
```

### Applying Scales to Notes

```rust
use muzze_std::{MAJOR, MAJOR_SECOND, MAJOR_THIRD, PERFECT_FOURTH, PERFECT_FIFTH, MAJOR_SIXTH, MAJOR_SEVENTH, OCTAVE};

const C: u8 = 60; // Middle C

// Apply the major scale to C
let c_major_notes: Vec<u8> = MAJOR.apply(C).collect();
// Result: [60, 62, 64, 65, 67, 69, 71, 72] (C, D, E, F, G, A, B, C)
// The intervals are: [MAJOR_SECOND, MAJOR_THIRD, PERFECT_FOURTH, PERFECT_FIFTH, MAJOR_SIXTH, MAJOR_SEVENTH, OCTAVE]
```

### Working with Musical Steps

```rust
use muzze_std::{Step, HALF, WHOLE, WHOLE_HALF};

// Using predefined constants
let half_step = HALF;
let whole_step = WHOLE;
let whole_half_step = WHOLE_HALF;

// Display as musical notation
assert_eq!(half_step.to_string(), "H");
assert_eq!(whole_step.to_string(), "W");
assert_eq!(whole_half_step.to_string(), "WH");

// Convert to/from semitone values
assert_eq!(u8::from(half_step), 1);
assert_eq!(u8::from(whole_step), 2);
assert_eq!(u8::from(whole_half_step), 3);

// Create custom steps
let custom_step = Step::from(4);
assert_eq!(custom_step.to_string(), "S4");
assert_eq!(u8::from(custom_step), 4);
```

### Working with Musical Intervals

```rust
use muzze_std::{Interval, MAJOR_THIRD, PERFECT_FIFTH, OCTAVE, AUGMENTED_FOURTH};

// Using predefined constants
let major_third = MAJOR_THIRD;
let perfect_fifth = PERFECT_FIFTH;
let octave = OCTAVE;

// Display as musical notation
assert_eq!(major_third.to_string(), "M3");
assert_eq!(perfect_fifth.to_string(), "P5");
assert_eq!(octave.to_string(), "P8");

// Convert to/from semitone values
assert_eq!(u8::from(major_third), 4);
assert_eq!(u8::from(perfect_fifth), 7);
assert_eq!(u8::from(octave), 12);

// Create custom intervals
let custom_interval = Interval::from(15);
assert_eq!(custom_interval.to_string(), "I15");
assert_eq!(u8::from(custom_interval), 15);

// Test interval relationships
assert_eq!(AUGMENTED_FOURTH, DIMINISHED_FIFTH); // Both are tritones (same semitone value)
assert_eq!(u8::from(AUGMENTED_FOURTH), 6);
assert_eq!(u8::from(MAJOR_THIRD) - u8::from(Interval::from(3)), 1); // Major vs Minor third
```

### Working with Musical Accidentals

```rust
use muzze_std::{Accidental, NATURAL, SHARP, FLAT, DOUBLE_SHARP, RESET_ACCIDENTAL};

// Using enum variants
let natural = Accidental::Natural;
let sharp = Accidental::Sharp;
let flat = Accidental::Flat;
let double_sharp = Accidental::DoubleSharp;

// Using convenience constants
let reset = RESET_ACCIDENTAL;

// Display as Unicode symbols
assert_eq!(natural.to_string(), "");
assert_eq!(sharp.to_string(), "♯");
assert_eq!(flat.to_string(), "♭");
assert_eq!(double_sharp.to_string(), "♯♯");
assert_eq!(reset.to_string(), "♮");

// Convert to/from numeric values
assert_eq!(u8::from(sharp), 8);
assert_eq!(Accidental::from(8), Accidental::Sharp);
assert_eq!(u8::from(RESET_ACCIDENTAL), 15);
```

### Working with Musical Chords

```rust
use muzze_std::{Chord, ChordBuilder, ROOT, THIRD, FIFTH, FLAT_THIRD, SHARP_FIFTH, FLAT_SEVENTH, MAJOR_TRIAD, MINOR_TRIAD, DOMINANT_SEVENTH};

// Use predefined chords
let major_triad = MAJOR_TRIAD;
let minor_triad = MINOR_TRIAD;
let dominant_seventh = DOMINANT_SEVENTH;

// Build custom chords using the builder
let custom_chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(SHARP_FIFTH)
    .set_degree(FLAT_SEVENTH)
    .build();

// Iterate over chord degrees
for degree in custom_chord.degrees() {
    println!("{}", degree); // Prints: R, 3, ♯5, ♭7
}

// Display chord as string
let chord_display = format!("{}", major_triad);
// Result: "R-3-5" - shows only the degrees that are set

// Create complex jazz chords
let jazz_chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(Degree::new(2, DEGREE_SHARP))  // Sharp 2nd
    .set_degree(SHARP_FIFTH)
    .set_degree(FLAT_SEVENTH)
    .set_degree(Degree::new(9, DEGREE_NATURAL)) // Natural 9th
    .build();
```

## Available Musical Types

### Predefined Scales

The library includes many predefined scales:

### Diatonic Scales
- `MAJOR` - Major scale (WHOLE-WHOLE-HALF-WHOLE-WHOLE-WHOLE-HALF)
- `NATURAL_MINOR` - Natural minor scale (WHOLE-HALF-WHOLE-WHOLE-HALF-WHOLE-WHOLE)
- `HARMONIC_MINOR` - Harmonic minor scale (WHOLE-HALF-WHOLE-WHOLE-HALF-WHOLE_HALF-HALF)
- `MELODIC_MINOR` - Melodic minor scale (WHOLE-HALF-WHOLE-WHOLE-WHOLE-WHOLE-HALF)

### Pentatonic Scales
- `PENTATONIC_MAJOR` - Major pentatonic scale
- `PENTATONIC_MINOR` - Minor pentatonic scale

### Blues Scales
- `BLUES_MAJOR` - Major blues scale
- `BLUES_MINOR` - Minor blues scale

### Jazz Scales
- `JAZZ_WHOLE_TONE` - Whole tone scale
- `JAZZ_WHOLEHALF_DIMINISHED` - Whole-half diminished scale

### Bebop Scales
- `BIBOP_MAJOR` - Bebop major scale
- `BIBOP_MINOR` - Bebop minor scale
- `BIBOP_DOMINANT` - Bebop dominant scale

### Musical Steps

The library provides step interval types for representing musical distances:

#### Predefined Step Constants
- `HALF` - Half step (1 semitone) - displayed as "H"
- `WHOLE` - Whole step (2 semitones) - displayed as "W"
- `WHOLE_HALF` - Whole-half step (3 semitones) - displayed as "WH"

#### Custom Steps
- `Step::from(n)` - Create custom step with n semitones - displayed as "S{n}"
- Supports any value from 0 to 255 semitones

### Musical Intervals

The library provides comprehensive interval types for representing musical distances with standard names:

#### Predefined Interval Constants
- `UNISON` - Unison (0 semitones) - displayed as "P1"
- `MINOR_SECOND` - Minor 2nd (1 semitone) - displayed as "m2"
- `MAJOR_SECOND` - Major 2nd (2 semitones) - displayed as "M2"
- `MINOR_THIRD` - Minor 3rd (3 semitones) - displayed as "m3"
- `MAJOR_THIRD` - Major 3rd (4 semitones) - displayed as "M3"
- `PERFECT_FOURTH` - Perfect 4th (5 semitones) - displayed as "P4"
- `AUGMENTED_FOURTH` - Augmented 4th (6 semitones) - displayed as "d5"
- `DIMINISHED_FIFTH` - Diminished 5th (6 semitones) - displayed as "d5"
- `PERFECT_FIFTH` - Perfect 5th (7 semitones) - displayed as "P5"
- `MINOR_SIXTH` - Minor 6th (8 semitones) - displayed as "m6"
- `MAJOR_SIXTH` - Major 6th (9 semitones) - displayed as "M6"
- `MINOR_SEVENTH` - Minor 7th (10 semitones) - displayed as "m7"
- `MAJOR_SEVENTH` - Major 7th (11 semitones) - displayed as "M7"
- `OCTAVE` - Octave (12 semitones) - displayed as "P8"

#### Custom Intervals
- `Interval::from(n)` - Create custom interval with n semitones - displayed as "I{n}"
- Supports any value from 0 to 255 semitones

### Musical Accidentals

The library provides comprehensive accidental types for musical notation:

#### Enum Variants
- `Accidental::Natural` - No pitch modification (value: 0)
- `Accidental::Reset` - Explicitly cancels previous accidentals (value: 15)
- `Accidental::Flat` - Lowers pitch by one semitone (value: 2)
- `Accidental::DoubleFlat` - Lowers pitch by two semitones (value: 3)
- `Accidental::Sharp` - Raises pitch by one semitone (value: 8)
- `Accidental::DoubleSharp` - Raises pitch by two semitones (value: 9)

#### Convenience Constants
- `NATURAL` - Natural accidental constant
- `FLAT` - Flat accidental constant
- `SHARP` - Sharp accidental constant
- `DOUBLE_FLAT` - Double flat accidental constant
- `DOUBLE_SHARP` - Double sharp accidental constant
- `RESET_ACCIDENTAL` - Reset accidental constant

### Musical Chords

The library provides comprehensive chord types for musical harmony:

#### Predefined Chord Constants
- `MAJOR_TRIAD` - Major triad (Root, Major 3rd, Perfect 5th)
- `MINOR_TRIAD` - Minor triad (Root, Minor 3rd, Perfect 5th)
- `DIMINISHED_TRIAD` - Diminished triad (Root, Minor 3rd, Diminished 5th)
- `AUGMENTED_TRIAD` - Augmented triad (Root, Major 3rd, Augmented 5th)
- `MAJOR_SEVENTH_CHORD` - Major 7th chord (Root, Major 3rd, Perfect 5th, Major 7th)
- `MINOR_SEVENTH_CHORD` - Minor 7th chord (Root, Minor 3rd, Perfect 5th, Minor 7th)
- `DOMINANT_SEVENTH` - Dominant 7th chord (Root, Major 3rd, Perfect 5th, Minor 7th)
- `HALF_DIMINISHED_SEVENTH` - Half-diminished 7th chord (Root, Minor 3rd, Diminished 5th, Minor 7th)
- `DIMINISHED_SEVENTH` - Diminished 7th chord (Root, Minor 3rd, Diminished 5th, Diminished 7th)
- `AUGMENTED_SEVENTH` - Augmented 7th chord (Root, Major 3rd, Augmented 5th, Minor 7th)
- `SUSPENDED_FOURTH` - Suspended 4th chord (Root, Perfect 4th, Perfect 5th)

#### Chord Degree Constants
- `ROOT` - Root degree (1st degree, natural)
- `THIRD` - Major third degree (3rd degree, natural)
- `FLAT_THIRD` - Minor third degree (3rd degree, flat)
- `FOURTH` - Fourth degree (4th degree, natural)
- `FIFTH` - Perfect fifth degree (5th degree, natural)
- `FLAT_FIFTH` - Diminished fifth degree (5th degree, flat)
- `SHARP_FIFTH` - Augmented fifth degree (5th degree, sharp)
- `SEVENTH` - Major seventh degree (7th degree, natural)
- `FLAT_SEVENTH` - Minor seventh degree (7th degree, flat)
- `DOUBLEFLAT_SEVENTH` - Diminished seventh degree (7th degree, double flat)

#### Degree Accidental Constants
- `DEGREE_NATURAL` - Natural accidental for chord degrees
- `DEGREE_FLAT` - Flat accidental for chord degrees
- `DEGREE_DOUBLEFLAT` - Double flat accidental for chord degrees
- `DEGREE_SHARP` - Sharp accidental for chord degrees

## API Reference

### Scale

#### Methods
- `from_u16(value: u16) -> Scale` - Create a scale from a u16 bit pattern
- `intervals() -> impl Iterator<Item = Interval>` - Get an iterator over the intervals in the scale
- `steps() -> impl Iterator<Item = Step>` - Get an iterator over the step intervals in the scale
- `apply(root: u8) -> impl Iterator<Item = u8>` - Apply the scale to a root note, returning note values

#### Examples
```rust
use muzze_std::{Scale, MAJOR, MAJOR_THIRD, PERFECT_FIFTH};

// Create a scale from bit pattern
let custom_scale = Scale::from_u16(0b0000_1101_0101_1010);

// Get intervals in the scale
let intervals: Vec<u8> = MAJOR.intervals().map(|i| u8::from(i)).collect();
// Result: [2, 4, 5, 7, 9, 11, 12]

// Get step intervals
let steps: Vec<String> = MAJOR.steps().map(|s| s.to_string()).collect();
// Result: ["W", "W", "H", "W", "W", "W", "H"]

// Apply scale to root note (C = 0)
let notes: Vec<u8> = MAJOR.apply(0).collect();
// Result: [0, 2, 4, 5, 7, 9, 11, 12] (C major scale)
```

#### Predefined Scale Constants
- `MAJOR` - Major scale
- `NATURAL_MINOR` - Natural minor scale
- `HARMONIC_MINOR` - Harmonic minor scale
- `MELODIC_MINOR` - Melodic minor scale
- `PENTATONIC_MAJOR` - Major pentatonic scale
- `PENTATONIC_MINOR` - Minor pentatonic scale
- `BLUES_MAJOR` - Major blues scale
- `BLUES_MINOR` - Minor blues scale
- `JAZZ_WHOLE_TONE` - Whole tone scale
- `JAZZ_WHOLEHALF_DIMINISHED` - Whole-half diminished scale
- `BIBOP_MAJOR` - Bebop major scale
- `BIBOP_MINOR` - Bebop minor scale
- `BIBOP_DOMINANT` - Bebop dominant scale

### ScaleBuilder

#### Methods
- `new() -> ScaleBuilder` - Create a new scale builder
- `set_interval(interval: Interval) -> ScaleBuilder` - Add an interval to the scale
- `build() -> Scale` - Build the final scale

#### Traits
- `Default` - Creates a builder with no intervals set

#### Examples
```rust
use muzze_std::{ScaleBuilder, MAJOR_SECOND, MAJOR_THIRD, PERFECT_FIFTH, OCTAVE};

// Build a custom scale using intervals
let custom_scale = ScaleBuilder::default()
    .set_interval(MAJOR_SECOND)  // Major 2nd
    .set_interval(MAJOR_THIRD)   // Major 3rd
    .set_interval(PERFECT_FIFTH) // Perfect 5th
    .set_interval(OCTAVE)        // Octave
    .build();

// Get the intervals
let intervals: Vec<u8> = custom_scale.intervals().map(|i| u8::from(i)).collect();
// Result: [2, 4, 7, 12]
```

### ScaleStepBuilder

#### Methods
- `new() -> ScaleStepBuilder` - Create a new step-based scale builder
- `add_step(step: Step) -> ScaleStepBuilder` - Add a step interval to the scale
- `build() -> Scale` - Build the final scale

#### Traits
- `Default` - Creates a builder with no steps added

#### Examples
```rust
use muzze_std::{ScaleStepBuilder, WHOLE, HALF};

// Build a scale using step patterns
let major_scale = ScaleStepBuilder::default()
    .add_step(WHOLE)  // Whole step
    .add_step(WHOLE)  // Whole step
    .add_step(HALF)   // Half step
    .add_step(WHOLE)  // Whole step
    .add_step(WHOLE)  // Whole step
    .add_step(WHOLE)  // Whole step
    .add_step(HALF)   // Half step
    .build();

// This creates the same pattern as the predefined MAJOR scale
assert_eq!(major_scale, muzze_std::MAJOR);
```

### Step

#### Methods
- `inner() -> u8` - Get the underlying semitone value
- `from(value: u8) -> Step` - Create a step from a semitone value
- `to_string() -> String` - Get the string representation of the step

#### Predefined Step Constants
- `HALF` - Half step (1 semitone)
- `WHOLE` - Whole step (2 semitones)
- `WHOLE_HALF` - Whole-half step (3 semitones)

#### Traits
- `From<u8> for Step` - Convert from u8 to Step
- `From<Step> for u8` - Convert from Step to u8
- `Display` - String formatting
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` - Standard traits

#### Examples
```rust
use muzze_std::{Step, HALF, WHOLE, WHOLE_HALF};

// Using predefined constants
assert_eq!(HALF.inner(), 1);
assert_eq!(WHOLE.inner(), 2);
assert_eq!(WHOLE_HALF.inner(), 3);

// String representations
assert_eq!(HALF.to_string(), "H");
assert_eq!(WHOLE.to_string(), "W");
assert_eq!(WHOLE_HALF.to_string(), "WH");

// Creating custom steps
let custom_step = Step::from(4);
assert_eq!(custom_step.to_string(), "S4");
assert_eq!(custom_step.inner(), 4);

// Conversions
let semitone_value: u8 = WHOLE.into();
assert_eq!(semitone_value, 2);

let step_from_semitones = Step::from(3);
assert_eq!(step_from_semitones, WHOLE_HALF);
```

### Interval

#### Methods
- `inner() -> u8` - Get the underlying semitone value
- `add_step(step: Step) -> Interval` - Add a step to this interval
- `from(value: u8) -> Interval` - Create an interval from a semitone value
- `to_string() -> String` - Get the string representation of the interval

#### Predefined Interval Constants
- `UNISON` - Unison (0 semitones)
- `MINOR_SECOND` - Minor 2nd (1 semitone)
- `MAJOR_SECOND` - Major 2nd (2 semitones)
- `MINOR_THIRD` - Minor 3rd (3 semitones)
- `MAJOR_THIRD` - Major 3rd (4 semitones)
- `PERFECT_FOURTH` - Perfect 4th (5 semitones)
- `AUGMENTED_FOURTH` - Augmented 4th (6 semitones)
- `DIMINISHED_FIFTH` - Diminished 5th (6 semitones)
- `PERFECT_FIFTH` - Perfect 5th (7 semitones)
- `MINOR_SIXTH` - Minor 6th (8 semitones)
- `MAJOR_SIXTH` - Major 6th (9 semitones)
- `MINOR_SEVENTH` - Minor 7th (10 semitones)
- `MAJOR_SEVENTH` - Major 7th (11 semitones)
- `OCTAVE` - Octave (12 semitones)

#### Traits
- `From<u8> for Interval` - Convert from u8 to Interval
- `From<Interval> for u8` - Convert from Interval to u8
- `Display` - String formatting
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` - Standard traits

#### Examples
```rust
use muzze_std::{Interval, MAJOR_THIRD, PERFECT_FIFTH, OCTAVE, WHOLE, HALF};

// Using predefined constants
assert_eq!(MAJOR_THIRD.inner(), 4);
assert_eq!(PERFECT_FIFTH.inner(), 7);
assert_eq!(OCTAVE.inner(), 12);

// String representations
assert_eq!(MAJOR_THIRD.to_string(), "M3");
assert_eq!(PERFECT_FIFTH.to_string(), "P5");
assert_eq!(OCTAVE.to_string(), "P8");

// Creating custom intervals
let custom_interval = Interval::from(15);
assert_eq!(custom_interval.to_string(), "I15");
assert_eq!(custom_interval.inner(), 15);

// Adding steps to intervals
let major_third_plus_whole = MAJOR_THIRD.add_step(WHOLE);
assert_eq!(major_third_plus_whole.inner(), 6); // 4 + 2 = 6

let octave_plus_half = OCTAVE.add_step(HALF);
assert_eq!(octave_plus_half.inner(), 13); // 12 + 1 = 13

// Conversions
let semitone_value: u8 = PERFECT_FIFTH.into();
assert_eq!(semitone_value, 7);

let interval_from_semitones = Interval::from(4);
assert_eq!(interval_from_semitones, MAJOR_THIRD);
```

### Accidental

#### Enum Variants
- `Accidental::Natural` - No pitch modification
- `Accidental::Reset` - Explicitly cancels previous accidentals
- `Accidental::Flat` - Lowers pitch by one semitone
- `Accidental::DoubleFlat` - Lowers pitch by two semitones
- `Accidental::Sharp` - Raises pitch by one semitone
- `Accidental::DoubleSharp` - Raises pitch by two semitones

#### Predefined Accidental Constants
- `NATURAL` - Natural accidental constant
- `FLAT` - Flat accidental constant
- `SHARP` - Sharp accidental constant
- `DOUBLE_FLAT` - Double flat accidental constant
- `DOUBLE_SHARP` - Double sharp accidental constant
- `RESET_ACCIDENTAL` - Reset accidental constant

#### Methods
- `from(value: u8) -> Accidental` - Create an accidental from a u8 value
- `to_string() -> String` - Get the string representation of the accidental

#### Traits
- `From<u8> for Accidental` - Convert from u8 to Accidental
- `From<Accidental> for u8` - Convert from Accidental to u8
- `Display` - String formatting
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` - Standard traits

#### Examples
```rust
use muzze_std::{Accidental, NATURAL, FLAT, SHARP, DOUBLE_FLAT, DOUBLE_SHARP};

// Using predefined constants
assert_eq!(NATURAL.to_string(), "♮");
assert_eq!(FLAT.to_string(), "♭");
assert_eq!(SHARP.to_string(), "♯");
assert_eq!(DOUBLE_FLAT.to_string(), "𝄫");
assert_eq!(DOUBLE_SHARP.to_string(), "𝄪");

// Creating accidentals from values
let natural = Accidental::from(0);
assert_eq!(natural, NATURAL);

let sharp = Accidental::from(8);
assert_eq!(sharp, SHARP);

// Conversions
let value: u8 = FLAT.into();
assert_eq!(value, 2);

let accidental_from_value = Accidental::from(8);
assert_eq!(accidental_from_value, SHARP);

// Using enum variants directly
let flat_variant = Accidental::Flat;
assert_eq!(flat_variant, FLAT);
assert_eq!(flat_variant.to_string(), "♭");
```

### Chord

#### Methods
- `degrees() -> impl Iterator<Item = Degree>` - Get an iterator over the degrees in the chord
- `to_string() -> String` - Get the string representation of the chord

#### Display Format
- Chords display as hyphen-separated degree representations
- Root degree shows as "R", other degrees show their accidental (if any) followed by the degree number
- Examples: "R-3-5" (major triad), "R-♭3-5" (minor triad), "R-3-5-♭7" (dominant seventh)

#### Predefined Chord Constants
- `MAJOR_TRIAD` - Major triad (R-3-5)
- `MINOR_TRIAD` - Minor triad (R-♭3-5)
- `DIMINISHED_TRIAD` - Diminished triad (R-♭3-♭5)
- `AUGMENTED_TRIAD` - Augmented triad (R-3-♯5)
- `MAJOR_SEVENTH_CHORD` - Major 7th chord (R-3-5-7)
- `MINOR_SEVENTH_CHORD` - Minor 7th chord (R-♭3-5-♭7)
- `DOMINANT_SEVENTH` - Dominant 7th chord (R-3-5-♭7)
- `HALF_DIMINISHED_SEVENTH` - Half-diminished 7th chord (R-♭3-♭5-♭7)
- `DIMINISHED_SEVENTH` - Diminished 7th chord (R-♭3-♭5-♭♭7)
- `AUGMENTED_SEVENTH` - Augmented 7th chord (R-3-♯5-♭7)
- `MINOR_MAJOR_SEVENTH` - Minor Major 7th chord (R-♭3-5-7)
- `SIXTH_CHORD` - 6th chord (R-3-5-6)
- `SIXTH_MINOR_CHORD` - 6th Minor chord (R-♭3-5-6)
- `SIXTH_NINTH_CHORD` - 6th/9th chord (R-3-5-6-9)
- `FIFTH_CHORD` - 5th chord (R-5)
- `DOMINANT_NINTH` - Dominant 9th chord (R-3-5-♭7-9)
- `MINOR_NINTH` - Minor 9th chord (R-♭3-5-♭7-9)
- `MAJOR_NINTH` - Major 9th chord (R-3-5-7-9)
- `ELEVENTH_CHORD` - 11th chord (R-3-5-7-9-11)
- `MINOR_ELEVENTH` - Minor 11th chord (R-♭3-5-♭7-9-11)
- `MAJOR_ELEVENTH` - Major 11th chord (R-3-5-7-9-11)
- `THIRTEENTH_CHORD` - 13th chord (R-3-5-♭7-11-13)
- `MINOR_THIRTEENTH` - Minor 13th chord (R-♭3-5-♭7-9-11-13)
- `MAJOR_THIRTEENTH` - Major 13th chord (R-3-5-7-9-11-13)
- `ADDED_SECOND` - Added 2nd chord (R-2-3-5)
- `ADDED_NINTH` - Added 9th chord (R-3-5-9)
- `ADDED_ELEVENTH` - Added eleventh chord (R-3-5-11)
- `DOMINANT_SEVENTH_FLAT_FIVE` - Dominant 7th flat 5 (R-3-♭5-♭7)
- `DOMINANT_SEVENTH_SHARP_FIVE` - Dominant 7th sharp 5 (R-3-♯5-♭7)
- `SUSPENDED_SECOND` - Suspended 2nd chord (R-2-5)
- `SUSPENDED_FOURTH` - Suspended 4th chord (R-4-5)

#### Traits
- `Display` - String formatting
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` - Standard traits

#### Examples
```rust
use muzze_std::{Chord, MAJOR_TRIAD, MINOR_TRIAD, DOMINANT_SEVENTH};

// Using predefined chords
let major_triad = MAJOR_TRIAD;
let degrees: Vec<_> = major_triad.degrees().collect();
assert_eq!(degrees.len(), 3);
assert_eq!(degrees[0].to_string(), "R");  // Root
assert_eq!(degrees[1].to_string(), "3");  // Major third
assert_eq!(degrees[2].to_string(), "5");  // Perfect fifth

// Display chord as string
let chord_display = format!("{}", MAJOR_TRIAD);
assert_eq!(chord_display, "R-3-5");

// Working with different chord types
let minor_triad = MINOR_TRIAD;
let minor_degrees: Vec<_> = minor_triad.degrees().collect();
assert_eq!(minor_degrees[1].to_string(), "♭3"); // Minor third

let dominant_seventh = DOMINANT_SEVENTH;
let seventh_degrees: Vec<_> = dominant_seventh.degrees().collect();
assert_eq!(seventh_degrees.len(), 4);
assert_eq!(seventh_degrees[3].to_string(), "♭7"); // Minor seventh
```

### ChordBuilder

#### Methods
- `with_root() -> ChordBuilder` - Create a new chord builder with root degree set
- `set_degree(degree: Degree) -> ChordBuilder` - Add or modify a degree in the chord
- `build() -> Chord` - Build the final chord

#### Traits
- `Default` - Creates a builder with root degree set

#### Examples
```rust
use muzze_std::{ChordBuilder, ROOT, THIRD, FIFTH, FLAT_THIRD, SHARP_FIFTH, FLAT_SEVENTH};

// Build a major triad
let major_triad = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .build();

// Build a minor triad with augmented fifth
let minor_augmented = ChordBuilder::with_root()
    .set_degree(FLAT_THIRD)
    .set_degree(SHARP_FIFTH)
    .build();

// Build a complex jazz chord
let jazz_chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(SHARP_FIFTH)
    .set_degree(FLAT_SEVENTH)
    .build();

// Overwrite degrees (last setting wins)
let overwritten_chord = ChordBuilder::with_root()
    .set_degree(THIRD)      // Set major third
    .set_degree(FLAT_THIRD) // Overwrite with minor third
    .build();

let degrees: Vec<_> = overwritten_chord.degrees().collect();
assert_eq!(degrees[1].to_string(), "♭3"); // Minor third (overwrote major third)
```

### Degree

#### Methods
- `new(degree: u8, accidental: DegreeAccidental) -> Degree` - Create a new degree
- `to_string() -> String` - Get the string representation of the degree

#### Display Format
- Root degree (1st degree) displays as "R" for clarity
- All other degrees display as their accidental (if any) followed by the degree number
- Examples: "R" (root), "3" (major third), "♭3" (minor third), "♯5" (augmented fifth)

#### Predefined Degree Constants
- `ROOT` - Root degree (1st degree, natural)
- `SECOND` - Second degree (2nd degree, natural)
- `THIRD` - Major third degree (3rd degree, natural)
- `FLAT_THIRD` - Minor third degree (3rd degree, flat)
- `FOURTH` - Fourth degree (4th degree, natural)
- `FIFTH` - Perfect fifth degree (5th degree, natural)
- `FLAT_FIFTH` - Diminished fifth degree (5th degree, flat)
- `SHARP_FIFTH` - Augmented fifth degree (5th degree, sharp)
- `SIXTH` - Sixth degree (6th degree, natural)
- `SEVENTH` - Major seventh degree (7th degree, natural)
- `FLAT_SEVENTH` - Minor seventh degree (7th degree, flat)
- `DOUBLEFLAT_SEVENTH` - Diminished seventh degree (7th degree, double flat)
- `NINTH` - Ninth degree (9th degree, natural)
- `ELEVENTH` - Eleventh degree (11th degree, natural)
- `THIRTEENTH` - Thirteenth degree (13th degree, natural)

#### Traits
- `Display` - String formatting
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` - Standard traits

#### Examples
```rust
use muzze_std::{Degree, ROOT, THIRD, FLAT_THIRD, SHARP_FIFTH, DEGREE_NATURAL, DEGREE_FLAT, DEGREE_SHARP};

// Using predefined constants
assert_eq!(ROOT.to_string(), "R");
assert_eq!(THIRD.to_string(), "3");
assert_eq!(FLAT_THIRD.to_string(), "♭3");
assert_eq!(SHARP_FIFTH.to_string(), "♯5");

// Creating custom degrees
let custom_degree = Degree::new(2, DEGREE_SHARP);
assert_eq!(custom_degree.to_string(), "♯2");

let flat_ninth = Degree::new(9, DEGREE_FLAT);
assert_eq!(flat_ninth.to_string(), "♭9");

// Working with degree accidentals
let natural_degree = Degree::new(4, DEGREE_NATURAL);
assert_eq!(natural_degree.to_string(), "4");
```

### DegreeAccidental

#### Enum Variants
- `DegreeAccidental::Natural` - No pitch modification (value: 1)
- `DegreeAccidental::Flat` - Lowers pitch by one semitone (value: 2)
- `DegreeAccidental::DoubleFlat` - Lowers pitch by two semitones (value: 3)
- `DegreeAccidental::Sharp` - Raises pitch by one semitone (value: 4)

#### Predefined Degree Accidental Constants
- `DEGREE_NATURAL` - Natural accidental for chord degrees
- `DEGREE_FLAT` - Flat accidental for chord degrees
- `DEGREE_DOUBLEFLAT` - Double flat accidental for chord degrees
- `DEGREE_SHARP` - Sharp accidental for chord degrees

#### Methods
- `from(value: u8) -> DegreeAccidental` - Create a degree accidental from a u8 value
- `to_string() -> String` - Get the string representation of the degree accidental

#### Traits
- `From<u8> for DegreeAccidental` - Convert from u8 to DegreeAccidental
- `From<DegreeAccidental> for u8` - Convert from DegreeAccidental to u8
- `Display` - String formatting
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` - Standard traits

#### Examples
```rust
use muzze_std::{DegreeAccidental, DEGREE_NATURAL, DEGREE_FLAT, DEGREE_DOUBLEFLAT, DEGREE_SHARP};

// Using predefined constants
assert_eq!(DEGREE_NATURAL.to_string(), "");
assert_eq!(DEGREE_FLAT.to_string(), "♭");
assert_eq!(DEGREE_DOUBLEFLAT.to_string(), "♭♭");
assert_eq!(DEGREE_SHARP.to_string(), "♯");

// Creating degree accidentals from values
let natural = DegreeAccidental::from(1);
assert_eq!(natural, DEGREE_NATURAL);

let flat = DegreeAccidental::from(2);
assert_eq!(flat, DEGREE_FLAT);

// Conversions
let value: u8 = DEGREE_SHARP.into();
assert_eq!(value, 4);

let accidental_from_value = DegreeAccidental::from(3);
assert_eq!(accidental_from_value, DEGREE_DOUBLEFLAT);

// Using enum variants directly
let sharp_variant = DegreeAccidental::Sharp;
assert_eq!(sharp_variant, DEGREE_SHARP);
assert_eq!(sharp_variant.to_string(), "♯");
```

## Musical Theory

### Interval Representation

The library uses a semitone-based interval system where:
- 0 = `UNISON` (root note)
- 1 = `MINOR_SECOND`
- 2 = `MAJOR_SECOND`
- 3 = `MINOR_THIRD`
- 4 = `MAJOR_THIRD`
- 5 = `PERFECT_FOURTH`
- 6 = `AUGMENTED_FOURTH`/`DIMINISHED_FIFTH` (tritone)
- 7 = `PERFECT_FIFTH`
- 8 = `MINOR_SIXTH`
- 9 = `MAJOR_SIXTH`
- 10 = `MINOR_SEVENTH`
- 11 = `MAJOR_SEVENTH`
- 12 = `OCTAVE`

### Interval System

Musical intervals are represented with their corresponding standard names and numeric encodings:

- **Unison (P1)**: No pitch difference, same note
- **Minor 2nd (m2)**: One semitone, equivalent to a half step
- **Major 2nd (M2)**: Two semitones, equivalent to a whole step
- **Minor 3rd (m3)**: Three semitones
- **Major 3rd (M3)**: Four semitones
- **Perfect 4th (P4)**: Five semitones
- **Augmented 4th (d5)**: Six semitones, also known as the tritone
- **Diminished 5th (d5)**: Six semitones, equivalent to augmented 4th (tritone)
- **Perfect 5th (P5)**: Seven semitones
- **Minor 6th (m6)**: Eight semitones
- **Major 6th (M6)**: Nine semitones
- **Minor 7th (m7)**: Ten semitones
- **Major 7th (M7)**: Eleven semitones
- **Octave (P8)**: Twelve semitones, where the higher note has double the frequency

The numeric encoding allows for efficient storage and processing of interval information in musical applications, while the standard names provide familiar musical terminology.

### Scale Patterns

Scales are represented as bit patterns where each bit position (0-15) represents a semitone interval from the root. For example, the major scale pattern `0b0000_1101_0101_1010` represents the intervals [MAJOR_SECOND, MAJOR_THIRD, PERFECT_FOURTH, PERFECT_FIFTH, MAJOR_SIXTH, MAJOR_SEVENTH, OCTAVE].

### Step System

Musical steps represent the distance between notes in semitones:

- **Half Step (HALF)**: 1 semitone - the smallest interval in Western music
- **Whole Step (WHOLE)**: 2 semitones - equivalent to two half steps
- **Whole-Half Step (WHOLE_HALF)**: 3 semitones - commonly found in harmonic minor scales
- **Custom Steps (Step::from(n))**: Any number of semitones from 0 to 255

Steps are used to describe scale patterns and can be converted to/from their semitone values for mathematical operations.

### Accidental System

Musical accidentals are represented with their corresponding Unicode symbols and numeric encodings:

- **Natural** (♮): No pitch modification, displayed as empty string
- **Reset** (♮): Explicitly cancels previous accidentals in the same measure
- **Flat** (♭): Lowers pitch by one semitone
- **Double Flat** (♭♭): Lowers pitch by two semitones
- **Sharp** (♯): Raises pitch by one semitone
- **Double Sharp** (♯♯): Raises pitch by two semitones

The numeric encoding allows for efficient storage and processing of accidental information in musical applications.

### Chord Theory

Musical chords are represented as collections of degrees with their accidental modifications:

#### Chord Degree System

Chord degrees represent the position of notes within a chord (1st, 3rd, 5th, etc.) and can be modified with accidentals:

- **Root (R)**: The fundamental note that gives the chord its name and tonal center (displays as "R")
- **Third (3)**: Major third (natural) or minor third (flat) - determines major/minor quality
- **Fifth (5)**: Perfect fifth (natural), diminished fifth (flat), or augmented fifth (sharp)
- **Seventh (7)**: Major seventh (natural), minor seventh (flat), or diminished seventh (double flat)
- **Extensions (9, 11, 13)**: Higher degrees for extended chords

#### Chord Quality Types

The library supports all major chord quality types:

- **Major Chords**: Root + Major 3rd + Perfect 5th (bright, stable sound)
- **Minor Chords**: Root + Minor 3rd + Perfect 5th (dark, melancholic sound)
- **Diminished Chords**: Root + Minor 3rd + Diminished 5th (tense, unstable sound)
- **Augmented Chords**: Root + Major 3rd + Augmented 5th (bright, tense sound)
- **Seventh Chords**: Triads with added seventh degrees (sophisticated, jazzy sound)
- **Suspended Chords**: Chords where the third is replaced by another degree (suspended, unresolved sound)

#### Chord Storage

Chords are stored efficiently using a `U4Vec16` where each 4-bit value represents the accidental type for each degree position (1-16). A value of 0 represents natural, 1 represents flat, 2 represents double flat, and 3 represents sharp.

#### Chord Construction

The `ChordBuilder` provides a fluent interface for constructing chords:

```rust
use muzze_std::{ChordBuilder, ROOT, THIRD, FIFTH, FLAT_THIRD, SHARP_FIFTH, FLAT_SEVENTH};

// Build a major triad
let major_triad = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(FIFTH)
    .build();
// Result: "R-3-5"

// Build a complex jazz chord
let jazz_chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(SHARP_FIFTH)
    .set_degree(FLAT_SEVENTH)
    .set_degree(Degree::new(9, DEGREE_NATURAL)) // Add 9th
    .build();
// Result: "R-3-♯5-♭7-9"
```

## Performance

The library is designed for high performance:
- Uses `const` functions where possible for compile-time evaluation
- Built on top of `muzze-bitflags` for efficient bit vector operations
- Zero-allocation iterators for scale operations
- Efficient memory layout with packed data structures
- Optimized for musical computation use cases

## Examples

### Creating a Custom Scale

```rust
use muzze_std::{ScaleBuilder, MAJOR_SECOND, PERFECT_FOURTH, PERFECT_FIFTH, MINOR_SEVENTH, OCTAVE};

// Create a custom scale with specific intervals
let custom_scale = ScaleBuilder::default()
    .set_interval(u8::from(MAJOR_SECOND))  // Major 2nd
    .set_interval(u8::from(PERFECT_FOURTH)) // Perfect 4th
    .set_interval(u8::from(PERFECT_FIFTH))  // Perfect 5th
    .set_interval(u8::from(MINOR_SEVENTH))  // Minor 7th
    .set_interval(u8::from(OCTAVE))         // Octave
    .build();

// Get the step pattern
let steps: Vec<Step> = custom_scale.steps().collect();
// Result: [WHOLE, Step(3), WHOLE, Step(3), WHOLE]
```

### Analyzing Scale Properties

```rust
use muzze_std::{HARMONIC_MINOR, HALF, WHOLE, WHOLE_HALF, MAJOR_SECOND, MINOR_THIRD, PERFECT_FOURTH, PERFECT_FIFTH, MINOR_SIXTH, MAJOR_SEVENTH, OCTAVE};

// Get all intervals in the harmonic minor scale
let intervals: Vec<u8> = HARMONIC_MINOR.intervals().collect();
// Result: [2, 3, 5, 7, 8, 11, 12] (Major 2nd, Minor 3rd, Perfect 4th, Perfect 5th, Minor 6th, Major 7th, Octave)

// Get the step pattern
let steps: Vec<Step> = HARMONIC_MINOR.steps().collect();
// Result: [WHOLE, HALF, WHOLE, WHOLE, HALF, WHOLE_HALF, HALF]

// Count the number of notes in the scale
let note_count = HARMONIC_MINOR.intervals().count();
// Result: 7
```

### Working with Intervals

```rust
use muzze_std::{Interval, MAJOR_THIRD, PERFECT_FIFTH, OCTAVE, AUGMENTED_FOURTH, DIMINISHED_FIFTH};

// Analyze chord intervals
let major_chord_intervals = [MAJOR_THIRD, PERFECT_FIFTH];
let semitone_values: Vec<u8> = major_chord_intervals.iter().map(|&i| u8::from(i)).collect();
// Result: [4, 7] (Major 3rd + Perfect 5th)

// Test interval relationships
assert_eq!(AUGMENTED_FOURTH, DIMINISHED_FIFTH); // Both are tritones (same semitone value)
assert_eq!(u8::from(AUGMENTED_FOURTH), 6);
assert_eq!(AUGMENTED_FOURTH.to_string(), "d5");
assert_eq!(DIMINISHED_FIFTH.to_string(), "d5"); // Both display as diminished fifth

// Calculate interval differences
let major_third_semitones = u8::from(MAJOR_THIRD);
let perfect_fifth_semitones = u8::from(PERFECT_FIFTH);
let difference = perfect_fifth_semitones - major_third_semitones;
// Result: 3 (minor third between major third and perfect fifth)

// Create compound intervals
let compound_interval = Interval::from(u8::from(OCTAVE) + u8::from(MAJOR_THIRD));
assert_eq!(u8::from(compound_interval), 16); // Octave + Major 3rd
```

### Working with Chords

```rust
use muzze_std::{Chord, ChordBuilder, MAJOR_TRIAD, MINOR_TRIAD, DOMINANT_SEVENTH, ROOT, THIRD, FIFTH, FLAT_THIRD, SHARP_FIFTH, FLAT_SEVENTH, DEGREE_NATURAL, DEGREE_SHARP};

// Analyze predefined chords
let major_triad = MAJOR_TRIAD;
let degrees: Vec<_> = major_triad.degrees().collect();
assert_eq!(degrees.len(), 3);
assert_eq!(degrees[0].to_string(), "R");  // Root
assert_eq!(degrees[1].to_string(), "3");  // Major third
assert_eq!(degrees[2].to_string(), "5");  // Perfect fifth

// Compare different chord types
let minor_triad = MINOR_TRIAD;
let minor_degrees: Vec<_> = minor_triad.degrees().collect();
assert_eq!(minor_degrees[1].to_string(), "♭3"); // Minor third

let dominant_seventh = DOMINANT_SEVENTH;
let seventh_degrees: Vec<_> = dominant_seventh.degrees().collect();
assert_eq!(seventh_degrees.len(), 4);
assert_eq!(seventh_degrees[3].to_string(), "♭7"); // Minor seventh

// Build custom chords
let augmented_triad = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(SHARP_FIFTH)
    .build();

let augmented_degrees: Vec<_> = augmented_triad.degrees().collect();
assert_eq!(augmented_degrees[2].to_string(), "♯5"); // Augmented fifth

// Create complex jazz chords
let jazz_chord = ChordBuilder::with_root()
    .set_degree(THIRD)
    .set_degree(Degree::new(2, DEGREE_SHARP))  // Sharp 2nd
    .set_degree(SHARP_FIFTH)
    .set_degree(FLAT_SEVENTH)
    .set_degree(Degree::new(9, DEGREE_NATURAL)) // Natural 9th
    .build();

let jazz_degrees: Vec<_> = jazz_chord.degrees().collect();
assert_eq!(jazz_degrees.len(), 5);
assert_eq!(jazz_degrees[1].to_string(), "♯2"); // Sharp 2nd
assert_eq!(jazz_degrees[2].to_string(), "3");  // Major third
assert_eq!(jazz_degrees[3].to_string(), "♯5"); // Augmented fifth
assert_eq!(jazz_degrees[4].to_string(), "♭7"); // Minor seventh

// Display chords as strings
let chord_display = format!("{}", major_triad);
assert_eq!(chord_display, "R-3-5");

let complex_display = format!("{}", jazz_chord);
assert_eq!(complex_display, "R-♯2-3-♯5-♭7-9");
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### Version 0.1.0
- Initial release
- Scale representation and builders
- Predefined musical scales
- Musical step intervals with semitone values
- Musical interval types with standard names and semitone values
- Musical accidental types with Unicode symbols
- Chord representation and builders
- Predefined musical chords (triads, seventh chords, suspended chords)
- Chord degree system with accidental modifications
- Degree accidental types for chord construction
- Built on top of `muzze-bitflags` for efficient bit operations
- Comprehensive test suite
