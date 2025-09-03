# Muzze

A Rust workspace for musical computations and data structures, providing efficient representations of musical concepts using bit vectors and specialized data types.

[![Rust](https://github.com/veminovici/muzze/actions/workflows/rust.yml/badge.svg)](https://github.com/veminovici/muzze/actions/workflows/rust.yml)

## Workspace Structure

This workspace contains the following crates:

- **`muzze-bitflags`** - Bit vector and packed data structures optimized for musical computations
- **`muzze-std`** - The core library providing musical data structures and algorithms built on top of muzze-bitflags

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building the Workspace

To build all crates in the workspace:

```bash
cargo build
```

To run tests for all crates:

```bash
cargo test
```

To build in release mode:

```bash
cargo build --release
```

### Working with Individual Crates

To work with a specific crate, navigate to its directory:

```bash
# Work with the bit vector library
cd muzze-bitflags
cargo build
cargo test

# Work with the musical library
cd muzze-std
cargo build
cargo test
```

## Project Overview

Muzze is designed to provide high-performance musical computation capabilities in Rust. The library focuses on:

- **Efficiency**: Using bit vectors and packed data structures for optimal performance
- **Musical Accuracy**: Following standard Western music theory conventions
- **Extensibility**: Builder patterns and trait-based design for easy customization
- **Zero-Cost Abstractions**: Leveraging Rust's type system for compile-time optimizations

## Library Architecture

Muzze follows a modular architecture with two complementary libraries:

### muzze-bitflags
The foundational library providing efficient bit vector and packed data structures:
- **BitVec16**: 16-bit vector with bit-level operations
- **U4Vec16**: 16-element vector of 4-bit values (0-15)
- **U4x2**: Packed two 4-bit values in a single u8

### muzze-std
The musical library built on top of muzze-bitflags:
- **Scale System**: Musical scales with predefined patterns and builders
- **Degree System**: Musical degrees with accidentals
- **Musical Theory**: Implementation of Western music theory concepts

This separation allows users to choose the appropriate level of abstraction for their needs, from low-level bit operations to high-level musical computations.

## Core Features

### muzze-bitflags Library
- **BitVec16**: 16-bit vector with efficient bit-level operations and iteration
- **U4Vec16**: Vector of 16 elements, each being a 4-bit unsigned integer (0-15)
- **U4x2**: Packed representation of two 4-bit unsigned integers in a single u8

### muzze-std Library
- **Scale**: Musical scale representation with bit-vector backing
- **ScaleBuilder**: Fluent interface for constructing custom scales
- **ScaleStepBuilder**: Build scales using step patterns (whole steps, half steps)
- **Degreex**: Musical degree representation with accidentals (sharps, flats, etc.)
- **Predefined Scales**: Major, minor, pentatonic, blues, jazz, and bebop scales
- **Predefined Degrees**: Third, fifth, and seventh degrees with all accidental variations

### Performance Characteristics
- Compile-time evaluation using `const` functions
- Zero-allocation iterators for scale operations
- Efficient memory layout with packed data structures
- Bit manipulation for fast interval calculations

## Usage Examples

### Musical Scales

```rust
use muzze_std::{MAJOR, ScaleStepBuilder};

// Create a major scale using step patterns
let major_scale = ScaleStepBuilder::default()
    .add_step(2)  // Whole step
    .add_step(2)  // Whole step
    .add_step(1)  // Half step
    .add_step(2)  // Whole step
    .add_step(2)  // Whole step
    .add_step(2)  // Whole step
    .add_step(1)  // Half step
    .build();

// Apply to a root note
const C: u8 = 60; // Middle C
let c_major_notes: Vec<u8> = major_scale.apply(C).collect();
// Result: [60, 62, 64, 65, 67, 69, 71, 72]
```

### Musical Degrees

```rust
use muzze_std::{THIRD, THIRD_FLAT, THIRD_SHARP, FIFTH};

// Natural third degree
assert_eq!(THIRD.first(), 3);   // Third degree
assert_eq!(THIRD.second(), 0);  // Natural (no accidental)

// Flattened third degree (minor third)
assert_eq!(THIRD_FLAT.first(), 3);   // Third degree
assert_eq!(THIRD_FLAT.second(), 1);  // Flat accidental

// Sharpened third degree (augmented third)
assert_eq!(THIRD_SHARP.first(), 3);   // Third degree
assert_eq!(THIRD_SHARP.second(), 3);  // Sharp accidental
```

### Bit Vector Operations

```rust
use muzze_bitflags::{BitVec16, BitVec16Builder};

// Create a bit vector from a u16
let bits = BitVec16::from_u16(0b0000_1101_0101_1010);

// Check if specific bits are set
assert!(bits.bit(1));  // Bit 1 is set
assert!(!bits.bit(0)); // Bit 0 is not set

// Build a bit vector using the builder pattern
let custom_bits = BitVec16Builder::default()
    .set_index(0)
    .set_index(2)
    .set_index(4)
    .build();

// Get all set indices
let set_indices: Vec<usize> = custom_bits.indeces_on().collect();
// Result: [0, 2, 4]
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p muzze-bitflags
cargo test -p muzze-std

# Run tests with output
cargo test -- --nocapture
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Documentation

To generate and view documentation:

```bash
cargo doc --open
```

## Musical Theory

The library implements standard Western music theory concepts:

- **Interval System**: Semitone-based representation (0=unison, 12=octave)
- **Scale Patterns**: Bit-vector representation of scale degrees
- **Step Patterns**: Whole steps (2 semitones) and half steps (1 semitone)
- **Scale Types**: Diatonic, pentatonic, blues, jazz, and bebop scales
- **Degree System**: Musical degrees (1-7) with accidentals (natural, flat, sharp, double flat, double sharp)
- **Accidental Encoding**: Packed representation where degree is in lower 4 bits and accidental type in upper 4 bits

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

### Development Guidelines

- Follow Rust naming conventions
- Add comprehensive documentation
- Include tests for all public APIs
- Use `const` functions where possible
- Optimize for performance

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### Version 0.1.0
- Initial release
- **muzze-bitflags**: Bit vector and packed data structures (BitVec16, U4Vec16, U4x2)
- **muzze-std**: Musical scale and degree representations with builders
- Predefined musical scales and degrees with accidentals
- Comprehensive test suite
- Modular architecture with separate bit vector and musical libraries

## Future Plans

- Chord representation and analysis
- Rhythm and timing structures
- MIDI integration
- Audio synthesis support
- Advanced scale analysis algorithms
- Microtonal support

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Musical Theory Resources](https://www.musictheory.net/)

## Support

For questions, issues, or contributions, please:

- Open an issue on GitHub
- Check the documentation
- Review the test examples
- Join the discussion in the project's community channels
