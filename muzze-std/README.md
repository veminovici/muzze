# Muzze Standard Library

A Rust library for musical computations and data structures, providing efficient representations of musical concepts using bit vectors and specialized data types.

## Features

- **Scale**: Musical scale representation with predefined scales and builders
- **ScaleBuilder**: Fluent interface for constructing custom scales
- **ScaleStepBuilder**: Build scales using step patterns (whole steps, half steps)
- **Step**: Musical step intervals with semitone values (half steps, whole steps, etc.)
- **Accidental**: Musical accidental types with Unicode symbols (sharps, flats, naturals)

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
use muzze_std::{MAJOR, NATURAL_MINOR, ScaleBuilder, ScaleStepBuilder, HALF, WHOLE};

// Use predefined scales
let major_scale = MAJOR;
let intervals: Vec<u8> = major_scale.intervals().collect();
// Result: [2, 4, 5, 7, 9, 11, 12]

// Build a scale using absolute intervals
let custom_scale = ScaleBuilder::default()
    .set_interval(2)  // Major 2nd
    .set_interval(4)  // Major 3rd
    .set_interval(7)  // Perfect 5th
    .set_interval(12) // Octave
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
use muzze_std::MAJOR;

const C: u8 = 60; // Middle C

// Apply the major scale to C
let c_major_notes: Vec<u8> = MAJOR.apply(C).collect();
// Result: [60, 62, 64, 65, 67, 69, 71, 72] (C, D, E, F, G, A, B, C)
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

## Musical Theory

### Interval Representation

The library uses a semitone-based interval system where:
- 0 = Unison (root note)
- 1 = Minor 2nd
- 2 = Major 2nd
- 3 = Minor 3rd
- 4 = Major 3rd
- 5 = Perfect 4th
- 6 = Augmented 4th/Diminished 5th (tritone)
- 7 = Perfect 5th
- 8 = Minor 6th
- 9 = Major 6th
- 10 = Minor 7th
- 11 = Major 7th
- 12 = Octave

### Scale Patterns

Scales are represented as bit patterns where each bit position (0-15) represents a semitone interval from the root. For example, the major scale pattern `0b0000_1101_0101_1010` represents the intervals [2, 4, 5, 7, 9, 11, 12].

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
use muzze_std::ScaleBuilder;

// Create a custom scale with specific intervals
let custom_scale = ScaleBuilder::default()
    .set_interval(2)  // Major 2nd
    .set_interval(5)  // Perfect 4th
    .set_interval(7)  // Perfect 5th
    .set_interval(10) // Minor 7th
    .set_interval(12) // Octave
    .build();

// Get the step pattern
let steps: Vec<Step> = custom_scale.steps().collect();
// Result: [WHOLE, Step(3), WHOLE, Step(3), WHOLE]
```

### Analyzing Scale Properties

```rust
use muzze_std::{HARMONIC_MINOR, HALF, WHOLE, WHOLE_HALF};

// Get all intervals in the harmonic minor scale
let intervals: Vec<u8> = HARMONIC_MINOR.intervals().collect();
// Result: [2, 3, 5, 7, 8, 11, 12]

// Get the step pattern
let steps: Vec<Step> = HARMONIC_MINOR.steps().collect();
// Result: [WHOLE, HALF, WHOLE, WHOLE, HALF, WHOLE_HALF, HALF]

// Count the number of notes in the scale
let note_count = HARMONIC_MINOR.intervals().count();
// Result: 7
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
- Musical accidental types with Unicode symbols
- Built on top of `muzze-bitflags` for efficient bit operations
- Comprehensive test suite
