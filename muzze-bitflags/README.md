# Muzze Bit Flags

A Rust library providing efficient bit vector and packed data structures optimized for musical computations and general-purpose bit manipulation tasks.

## Features

- **BitVec16**: A 16-bit vector with efficient bit-level operations, iteration, and builder pattern
- **BitVec16Builder**: Fluent interface for constructing custom BitVec16 instances
- **U4Vec16**: A vector of 16 elements, each being a 4-bit unsigned integer (0-15) with fast access
- **U4Vec16Builder**: Fluent interface for constructing custom U4Vec16 instances
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

### U4Vec16Builder - Fluent Interface for U4Vec16

```rust
use muzze_bitflags::u4vec16::U4Vec16Builder;

// Build a U4Vec16 with specific items
let vec = U4Vec16Builder::new()
    .set_item(0, 5)   // Set item 0 to 5
    .set_item(1, 10)  // Set item 1 to 10
    .set_item(15, 15) // Set item 15 to 15
    .build();

assert_eq!(vec.item(0), 5);
assert_eq!(vec.item(1), 10);
assert_eq!(vec.item(15), 15);

// Using Default trait
let vec_default = U4Vec16Builder::default()
    .set_item(0, 3)
    .set_item(7, 7)
    .build();

assert_eq!(vec_default.item(0), 3);
assert_eq!(vec_default.item(7), 7);
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
- **Custom Chord Construction**: Use U4Vec16Builder to build custom chord patterns incrementally
- **Interval Tracking**: Use BitVec16 to track which intervals are present in a chord or scale
- **Musical Parameters**: Use U4x2 for pairs of musical parameters like attack/release times

### General Applications

- **Flags and Masks**: BitVec16 for efficient boolean flag storage and bit manipulation
- **Small Integer Arrays**: U4Vec16 for compact storage of values 0-15 (perfect for small counters, indices)
- **Custom Data Construction**: U4Vec16Builder for building custom packed data structures incrementally
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

#### Methods
- `from_u16(value: u16) -> BitVec16` - Create from u16 value
- `from_vec(bits: [bool; 16]) -> BitVec16` - Create from array of 16 booleans
- `inner() -> u16` - Get underlying u16 value
- `capacity() -> usize` - Get total number of bits (always 16)
- `bit(index: usize) -> bool` - Check if bit at index is set (0-15)
- `indeces_on() -> impl Iterator<Item = usize>` - Iterator over set bit indices
- `indeces_off() -> impl Iterator<Item = usize>` - Iterator over unset bit indices
- `iter_bits() -> BitVec16Iter` - Iterator over all bits as booleans

#### Traits
- `Index<usize>` - Support for bracket notation access
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` - Standard traits

#### Examples
```rust
use muzze_bitflags::BitVec16;

// Create from u16 value
let bitvec = BitVec16::from_u16(0b1010_1010_1010_1010);

// Create from boolean array
let bool_array = [true, false, true, false, true, false, true, false,
                  true, false, true, false, true, false, true, false];
let bitvec_from_array = BitVec16::from_vec(bool_array);
assert_eq!(bitvec, bitvec_from_array);

// Check individual bits
assert!(bitvec.bit(1));   // Bit 1 is set
assert!(!bitvec.bit(0));  // Bit 0 is not set
assert!(bitvec.bit(15));  // Bit 15 is set

// Get capacity
assert_eq!(bitvec.capacity(), 16);

// Get underlying value
assert_eq!(bitvec.inner(), 0b1010_1010_1010_1010);

// Iterate over set bits
let set_indices: Vec<usize> = bitvec.indeces_on().collect();
assert_eq!(set_indices, vec![1, 3, 5, 7, 9, 11, 13, 15]);

// Iterate over unset bits
let unset_indices: Vec<usize> = bitvec.indeces_off().collect();
assert_eq!(unset_indices, vec![0, 2, 4, 6, 8, 10, 12, 14]);

// Iterate over all bits
let all_bits: Vec<bool> = bitvec.iter_bits().collect();
assert_eq!(all_bits, vec![false, true, false, true, false, true, false, true,
                          false, true, false, true, false, true, false, true]);

// Bracket notation access
assert_eq!(bitvec[0], false);
assert_eq!(bitvec[1], true);
assert_eq!(bitvec[15], true);
```

### BitVec16Builder

#### Methods
- `new() -> BitVec16Builder` - Create new builder with all bits unset
- `set_index(index: u8) -> BitVec16Builder` - Set bit at index (0-15) to true
- `build() -> BitVec16` - Finalize and return BitVec16

#### Traits
- `Default` - Creates a builder with no bits set

#### Examples
```rust
use muzze_bitflags::{BitVec16, BitVec16Builder};

// Create builder and set specific bits
let custom_bitvec = BitVec16Builder::default()
    .set_index(0)   // Set bit 0
    .set_index(2)   // Set bit 2
    .set_index(15)  // Set bit 15
    .build();

// Verify the result
assert!(custom_bitvec.bit(0));
assert!(!custom_bitvec.bit(1));
assert!(custom_bitvec.bit(2));
assert!(!custom_bitvec.bit(3));
assert!(custom_bitvec.bit(15));

// Get the underlying value
assert_eq!(custom_bitvec.inner(), 0b1000_0000_0000_0101);

// Using new() method
let builder = BitVec16Builder::new();
let empty_bitvec = builder.build();
assert_eq!(empty_bitvec.inner(), 0);

// Chaining multiple set_index calls
let complex_bitvec = BitVec16Builder::new()
    .set_index(1)
    .set_index(3)
    .set_index(5)
    .set_index(7)
    .build();
assert_eq!(complex_bitvec.inner(), 0b1010_1010);
```

### U4Vec16

#### Methods
- `from_u64(value: u64) -> U4Vec16` - Create from u64 value
- `from_vec(items: [u8; 16]) -> U4Vec16` - Create from array of 16 u8 values
- `inner() -> u64` - Get underlying u64 value
- `capacity() -> usize` - Get total number of items (always 16)
- `item(index: usize) -> u8` - Get 4-bit item at index (0-15)
- `reset_item(index: usize)` - Reset item at index to 0 (mutable)
- `set_item(index: usize, value: u8)` - Set item at index to value (mutable)
- `iter_items() -> U4Vec16Iter` - Iterator over all 4-bit items

#### Traits
- `Index<usize>` - Support for bracket notation access
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` - Standard traits

### U4Vec16Builder

#### Methods
- `new() -> U4Vec16Builder` - Create new builder with all items set to 0
- `set_item(index: usize, value: u8) -> U4Vec16Builder` - Set 4-bit item at index (0-15)
- `build() -> U4Vec16` - Finalize construction and return U4Vec16

#### Traits
- `Default` - Creates a builder with no items set

#### Examples
```rust
use muzze_bitflags::U4Vec16;

// Create from u64 value
let vec = U4Vec16::from_u64(0x1234567890ABCDEF);

// Create from array
let items = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
let vec_from_array = U4Vec16::from_vec(items);

// Access individual 4-bit items
assert_eq!(vec.item(0), 0x0F);   // Least significant 4 bits
assert_eq!(vec.item(1), 0x0E);
assert_eq!(vec.item(15), 0x1);   // Most significant 4 bits

// Get capacity
assert_eq!(vec.capacity(), 16);

// Get underlying value
assert_eq!(vec.inner(), 0x1234567890ABCDEF);

// Iterate over all items
let all_items: Vec<u8> = vec.iter_items().collect();
assert_eq!(all_items, vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);

// Bracket notation access
assert_eq!(vec[0], 15);
assert_eq!(vec[1], 14);
assert_eq!(vec[15], 0);

// Mutable operations
let mut mutable_vec = U4Vec16::from_u64(0);
mutable_vec.set_item(0, 5);
mutable_vec.set_item(15, 10);
assert_eq!(mutable_vec.item(0), 5);
assert_eq!(mutable_vec.item(15), 10);

// Reset items
mutable_vec.reset_item(0);
assert_eq!(mutable_vec.item(0), 0);
```

#### Examples
```rust
use muzze_bitflags::u4vec16::U4Vec16Builder;

// Build a U4Vec16 with specific items
let vec = U4Vec16Builder::new()
    .set_item(0, 5)   // Set item 0 to 5
    .set_item(1, 10)  // Set item 1 to 10
    .set_item(15, 15) // Set item 15 to 15
    .build();

assert_eq!(vec.item(0), 5);
assert_eq!(vec.item(1), 10);
assert_eq!(vec.item(15), 15);

// Using Default trait
let vec_default = U4Vec16Builder::default()
    .set_item(0, 3)
    .set_item(7, 7)
    .build();

assert_eq!(vec_default.item(0), 3);
assert_eq!(vec_default.item(7), 7);

// Method chaining with overwrites
let vec_chained = U4Vec16Builder::new()
    .set_item(0, 5)
    .set_item(0, 10)  // Overwrite item 0
    .set_item(1, 15)
    .set_item(1, 3)   // Overwrite item 1
    .build();

assert_eq!(vec_chained.item(0), 10);
assert_eq!(vec_chained.item(1), 3);

// Complex pattern construction
let complex_vec = U4Vec16Builder::new()
    .set_item(0, 15)  // 15
    .set_item(1, 14)  // 14
    .set_item(2, 13)  // 13
    .set_item(3, 12)  // 12
    .set_item(4, 11)  // 11
    .set_item(5, 10)  // 10
    .set_item(6, 9)   // 9
    .set_item(7, 8)   // 8
    .set_item(8, 7)   // 7
    .set_item(9, 6)   // 6
    .set_item(10, 5)  // 5
    .set_item(11, 4)  // 4
    .set_item(12, 3)  // 3
    .set_item(13, 2)  // 2
    .set_item(14, 1)  // 1
    .set_item(15, 0)  // 0
    .build();

// Verify the descending pattern
for i in 0..16 {
    assert_eq!(complex_vec.item(i), (15 - i) as u8);
}

// Sparse pattern (only a few items set)
let sparse_vec = U4Vec16Builder::new()
    .set_item(0, 1)
    .set_item(7, 7)
    .set_item(15, 15)
    .build();

assert_eq!(sparse_vec.item(0), 1);
assert_eq!(sparse_vec.item(7), 7);
assert_eq!(sparse_vec.item(15), 15);

// All other items remain 0
for i in 1..7 {
    assert_eq!(sparse_vec.item(i), 0);
}
for i in 8..15 {
    assert_eq!(sparse_vec.item(i), 0);
}
```

### U4x2

#### Methods
- `new(first: u8, second: u8) -> U4x2` - Create with two 4-bit values (0-15 each)
- `inner() -> u8` - Get underlying u8 value
- `first() -> u8` - Extract first 4-bit value (lower bits)
- `second() -> u8` - Extract second 4-bit value (upper bits)

#### Traits
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` - Standard traits

#### Examples
```rust
use muzze_bitflags::U4x2;

// Pack two 4-bit values into a single u8
let packed = U4x2::new(10, 5);  // First: 10, Second: 5

// Extract the values
assert_eq!(packed.first(), 10);   // Lower 4 bits
assert_eq!(packed.second(), 5);   // Upper 4 bits
assert_eq!(packed.inner(), 0b0101_1010);  // 5 << 4 | 10

// Create with different values
let velocity_pair = U4x2::new(15, 12);  // Max velocity, high velocity
assert_eq!(velocity_pair.first(), 15);
assert_eq!(velocity_pair.second(), 12);
assert_eq!(velocity_pair.inner(), 0b1100_1111);  // 12 << 4 | 15

// Edge cases
let min_values = U4x2::new(0, 0);
assert_eq!(min_values.first(), 0);
assert_eq!(min_values.second(), 0);
assert_eq!(min_values.inner(), 0);

let max_values = U4x2::new(15, 15);
assert_eq!(max_values.first(), 15);
assert_eq!(max_values.second(), 15);
assert_eq!(max_values.inner(), 0b1111_1111);

// Musical application: note velocities
let note_velocities = U4x2::new(8, 12);  // Attack: 8, Release: 12
assert_eq!(note_velocities.first(), 8);   // Attack velocity
assert_eq!(note_velocities.second(), 12); // Release velocity
```

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
- BitVec16Builder for fluent construction
- U4Vec16 with 16-element 4-bit storage
- U4Vec16Builder for fluent construction
- U4x2 packed two 4-bit values
- Comprehensive test suite
- Zero-allocation iterators
- Const function support

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
