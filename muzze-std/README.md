# Muzze Standard Library

A Rust library for musical computations and data structures, providing efficient representations of musical concepts using bit vectors and specialized data types.

## Features

- **Scale**: Musical scale representation with predefined scales and builders
- **ScaleBuilder**: Fluent interface for constructing custom scales
- **ScaleStepBuilder**: Build scales using step patterns (whole steps, half steps)
- **Degreex**: Musical degree representation with accidentals (sharps, flats, etc.)
- **Bit Vector Support**: Built on top of `muzze-bitflags` for efficient bit operations

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
use muzze_std::{MAJOR, NATURAL_MINOR, ScaleBuilder, ScaleStepBuilder};

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
    .add_step(2)  // Whole step
    .add_step(2)  // Whole step
    .add_step(1)  // Half step
    .add_step(2)  // Whole step
    .add_step(2)  // Whole step
    .add_step(2)  // Whole step
    .add_step(1)  // Half step
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

### Working with Musical Degrees

```rust
use muzze_std::{THIRD, THIRD_FLAT, THIRD_SHARP, FIFTH, SEVENTH};

// Natural third degree
assert_eq!(THIRD.first(), 3);   // Third degree
assert_eq!(THIRD.second(), 0);  // Natural (no accidental)

// Flattened third degree (minor third)
assert_eq!(THIRD_FLAT.first(), 3);   // Third degree
assert_eq!(THIRD_FLAT.second(), 1);  // Flat accidental

// Sharpened third degree (augmented third)
assert_eq!(THIRD_SHARP.first(), 3);   // Third degree
assert_eq!(THIRD_SHARP.second(), 3);  // Sharp accidental

// Perfect fifth
assert_eq!(FIFTH.first(), 5);   // Fifth degree
assert_eq!(FIFTH.second(), 0);  // Natural

// Major seventh
assert_eq!(SEVENTH.first(), 7);   // Seventh degree
assert_eq!(SEVENTH.second(), 0);  // Natural
```

## Available Musical Types

### Predefined Scales

The library includes many predefined scales:

### Diatonic Scales
- `MAJOR` - Major scale (W-W-H-W-W-W-H)
- `NATURAL_MINOR` - Natural minor scale (W-H-W-W-H-W-W)
- `HARMONIC_MINOR` - Harmonic minor scale (W-H-W-W-H-WH-H)
- `MELODIC_MINOR` - Melodic minor scale (W-H-W-W-W-W-H)

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

### Predefined Musical Degrees

The library includes predefined musical degrees with accidentals:

#### Third Degree Variations
- `THIRD` - Natural third degree (major third)
- `THIRD_FLAT` - Flattened third degree (minor third)
- `THIRD_DOUBLEFLAT` - Double flattened third degree (diminished third)
- `THIRD_SHARP` - Sharpened third degree (augmented third)
- `THIRD_DOUBLESHARP` - Double sharpened third degree (doubly augmented third)

#### Fifth Degree Variations
- `FIFTH` - Natural fifth degree (perfect fifth)
- `FIFTH_FLAT` - Flattened fifth degree (diminished fifth/tritone)
- `FIFTH_DOUBLEFLAT` - Double flattened fifth degree (doubly diminished fifth)
- `FIFTH_SHARP` - Sharpened fifth degree (augmented fifth)
- `FIFTH_DOUBLESHARP` - Double sharpened fifth degree (doubly augmented fifth)

#### Seventh Degree Variations
- `SEVENTH` - Natural seventh degree (major seventh)
- `SEVENTH_FLAT` - Flattened seventh degree (minor seventh)
- `SEVENTH_DOUBLEFLAT` - Double flattened seventh degree (diminished seventh)
- `SEVENTH_SHARP` - Sharpened seventh degree (augmented seventh)
- `SEVENTH_DOUBLESHARP` - Double sharpened seventh degree (doubly augmented seventh)

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

### Degree and Accidental Encoding

Musical degrees are encoded using a packed representation where:
- The degree (1-7) is stored in the lower 4 bits
- The accidental type is stored in the upper 4 bits:
  - 0 = Natural (no accidental)
  - 1 = Flat (♭)
  - 2 = Double flat (♭♭)
  - 3 = Sharp (♯)
  - 4 = Double sharp (♯♯)

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
let steps: Vec<u8> = custom_scale.steps().collect();
// Result: [2, 3, 2, 3, 2]
```

### Analyzing Scale Properties

```rust
use muzze_std::HARMONIC_MINOR;

// Get all intervals in the harmonic minor scale
let intervals: Vec<u8> = HARMONIC_MINOR.intervals().collect();
// Result: [2, 3, 5, 7, 8, 11, 12]

// Get the step pattern
let steps: Vec<u8> = HARMONIC_MINOR.steps().collect();
// Result: [2, 1, 2, 2, 1, 3, 1] (note the augmented 2nd: 3)

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
- Musical degree representation with accidentals
- Predefined musical scales and degrees
- Built on top of `muzze-bitflags` for efficient bit operations
- Comprehensive test suite
