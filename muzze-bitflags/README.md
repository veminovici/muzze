# Muzze Bit Flags

A Rust library providing efficient bit vector and packed data structures optimized for musical computations and general-purpose bit manipulation tasks.

## Features

- **BitVec16**: A 16-bit vector with efficient bit-level operations, iteration, and builder pattern
- **U4Vec16**: A vector of 16 elements, each being a 4-bit unsigned integer (0-15) with fast access
- **U4x2**: A packed representation of two 4-bit unsigned integers in a single u8 for memory efficiency
- **Zero-allocation iterators** for efficient processing
- **Const functions** for compile-time evaluation
- **Comprehensive test suite** with 100% coverage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
muzze-bitflags = "0.1.0"
```

### Dependencies

This library depends on:
- `bitflags = "2.5.3"` - For efficient bit flag operations

The library is designed to be lightweight with minimal dependencies while providing maximum performance.

## Quick Start

### BitVec16 - 16-bit Vector

```rust
use muzze_bitflags::{BitVec16, BitVec16Builder};

// Create from a u16 value
let bitvec = BitVec16::from_u16(0b1010_1010_1010_1010);

// Check individual bits
assert!(bitvec.bit(1));
assert!(!bitvec.bit(0));

// Build using the builder pattern
let custom_bitvec = BitVec16Builder::default()
    .set_index(0)
    .set_index(2)
    .set_index(15)
    .build();

// Iterate over set bits
let set_indices: Vec<usize> = bitvec.indeces_on().collect();
// Result: [1, 3, 5, 7, 9, 11, 13, 15]

// Get the underlying u16 value
assert_eq!(bitvec.inner(), 0b1010_1010_1010_1010);
```

### U4Vec16 - 16-element Vector of 4-bit Values

```rust
use muzze_bitflags::U4Vec16;

// Create from a u64 value
let vec = U4Vec16::from_u64(0x1234567890ABCDEF);

// Access individual 4-bit items
assert_eq!(vec.item(0), 0x0F);  // Least significant 4 bits
assert_eq!(vec.item(15), 0x1);  // Most significant 4 bits

// Use convenience methods
assert_eq!(vec.item0(), 0x0F);
assert_eq!(vec.item15(), 0x1);

// Iterate over all items
let items: Vec<u8> = vec.iter_items().collect();
assert_eq!(items.len(), 16);

// Access using bracket notation
assert_eq!(vec[0], 0x0F);
assert_eq!(vec[15], 0x1);

// Get the underlying u64 value
assert_eq!(vec.inner(), 0x1234567890ABCDEF);
```

### U4x2 - Packed Two 4-bit Values

```rust
use muzze_bitflags::U4x2;

// Pack two 4-bit values into a single u8
let packed = U4x2::new(10, 5);  // First: 10, Second: 5

// Extract the values
assert_eq!(packed.first(), 10);
assert_eq!(packed.second(), 5);
assert_eq!(packed.inner(), 0b0101_1010);  // 5 << 4 | 10

// Create with different values
let velocity_pair = U4x2::new(15, 12);  // Max velocity, high velocity
assert_eq!(velocity_pair.first(), 15);
assert_eq!(velocity_pair.second(), 12);
```

## Use Cases

### Musical Applications

- **Scale Representation**: Use BitVec16 to represent musical scales where each bit represents a semitone interval
- **Note Velocities**: Use U4x2 to pack note velocity pairs efficiently (0-15 range)
- **Chord Analysis**: Use U4Vec16 to store chord tone information compactly
- **Interval Tracking**: Use BitVec16 to track which intervals are present in a chord or scale
- **Musical Parameters**: Use U4x2 for pairs of musical parameters like attack/release times

### General Applications

- **Flags and Masks**: BitVec16 for efficient boolean flag storage and bit manipulation
- **Small Integer Arrays**: U4Vec16 for compact storage of values 0-15 (perfect for small counters, indices)
- **Data Packing**: U4x2 for memory-efficient storage of small value pairs
- **Configuration Storage**: Use packed structures to store multiple configuration values
- **Game Development**: Efficient storage for game state, player stats, or inventory items
- **Embedded Systems**: Memory-efficient data structures for resource-constrained environments

## Performance

The library is designed for high performance:

- **Zero-allocation iterators** for efficient iteration without heap allocations
- **Const functions** for compile-time evaluation and optimization
- **Bit manipulation** for fast operations using native CPU instructions
- **Packed data structures** for memory efficiency and cache-friendly access
- **ExactSizeIterator** implementation for optimal collection operations
- **Inline functions** for maximum performance in hot paths
- **Minimal dependencies** to reduce compilation time and binary size

## API Reference

### BitVec16

- `from_u16(value)` - Create from u16 value
- `inner()` - Get underlying u16 value
- `capacity()` - Get total number of bits (always 16)
- `bit(index)` - Check if bit at index is set (0-15)
- `indeces_on()` - Iterator over set bit indices
- `indeces_off()` - Iterator over unset bit indices
- `iter_bits()` - Iterator over all bits as booleans
- `Index<usize>` - Support for bracket notation access

### BitVec16Builder

- `new()` - Create new builder with all bits unset
- `set_index(index)` - Set bit at index (0-15) to true
- `build()` - Finalize and return BitVec16

### U4Vec16

- `from_u64(value)` - Create from u64 value
- `inner()` - Get underlying u64 value
- `capacity()` - Get total number of items (always 16)
- `item(index)` - Get 4-bit item at index (0-15)
- `iter_items()` - Iterator over all 4-bit items
- `Index<usize>` - Support for bracket notation access

### U4x2

- `new(first, second)` - Create with two 4-bit values (0-15 each)
- `inner()` - Get underlying u8 value
- `first()` - Extract first 4-bit value (lower bits)
- `second()` - Extract second 4-bit value (upper bits)

## Testing

The library includes comprehensive tests covering all functionality:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run documentation tests
cargo test --doc
```

## Benchmarks

Performance benchmarks are available to demonstrate the efficiency of the data structures:

```bash
# Run benchmarks (if available)
cargo bench
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

### Development Guidelines

- Follow Rust naming conventions
- Add comprehensive documentation
- Include tests for all public APIs
- Use `const` functions where possible
- Optimize for performance

## Changelog

### Version 0.1.0
- Initial release
- BitVec16 with builder pattern and iteration
- U4Vec16 with 16-element 4-bit storage
- U4x2 packed two 4-bit values
- Comprehensive test suite
- Zero-allocation iterators
- Const function support

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
