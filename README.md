# Muzze

A Rust workspace for musical computations and data structures, providing efficient representations of musical concepts using bit vectors and specialized data types.

## Workspace Structure

This workspace contains the following crates:

- **`muzze-std`** - The core library providing musical data structures and algorithms

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

## Core Features

### Musical Data Structures
- **BitVec16**: 16-bit vector optimized for musical interval representations
- **U4Vec16**: Vector of 4-bit unsigned integers for compact musical data storage

### Scale System
- **Scale**: Musical scale representation with bit-vector backing
- **Predefined Scales**: Major, minor, pentatonic, blues, jazz, and bebop scales
- **Builders**: Fluent interfaces for constructing custom scales

### Performance Characteristics
- Compile-time evaluation using `const` functions
- Zero-allocation iterators for scale operations
- Efficient memory layout with packed data structures
- Bit manipulation for fast interval calculations

## Usage Example

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

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
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
- BitVec16 and U4Vec16 data structures
- Scale representation and builders
- Predefined musical scales
- Comprehensive test suite

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
