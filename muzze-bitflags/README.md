# Muzze Bit Flags

A Rust library providing efficient bit vector and packed data structures optimized for musical computations.

## Features

- **BitVec16**: A 16-bit vector with efficient bit-level operations and iteration
- **U4Vec16**: A vector of 16 elements, each being a 4-bit unsigned integer (0-15)
- **U4x2**: A packed representation of two 4-bit unsigned integers in a single u8

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
muzze-bitflags = "0.1.0"
```

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
```

## Use Cases

### Musical Applications

- **Scale Representation**: Use BitVec16 to represent musical scales where each bit represents a semitone interval
- **Note Velocities**: Use U4x2 to pack note velocity pairs efficiently
- **Chord Analysis**: Use U4Vec16 to store chord tone information compactly

### General Applications

- **Flags and Masks**: BitVec16 for efficient boolean flag storage
- **Small Integer Arrays**: U4Vec16 for compact storage of values 0-15
- **Data Packing**: U4x2 for memory-efficient storage of small value pairs

## Performance

The library is designed for high performance:

- **Zero-allocation iterators** for efficient iteration
- **Const functions** for compile-time evaluation
- **Bit manipulation** for fast operations
- **Packed data structures** for memory efficiency

## API Reference

### BitVec16

- `from_u16(value)` - Create from u16 value
- `bit(index)` - Check if bit at index is set
- `bit0()` through `bit15()` - Convenience methods for individual bits
- `indeces_on()` - Iterator over set bit indices
- `indeces_off()` - Iterator over unset bit indices
- `iter_bits()` - Iterator over all bits as booleans

### U4Vec16

- `from_u64(value)` - Create from u64 value
- `item(index)` - Get 4-bit item at index
- `item0()` through `item15()` - Convenience methods for individual items
- `iter_items()` - Iterator over all 4-bit items

### U4x2

- `new(first, second)` - Create with two 4-bit values
- `first()` - Extract first 4-bit value
- `second()` - Extract second 4-bit value
- `inner()` - Get underlying u8 value

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
