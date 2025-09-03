//! Muzze Bit Flags Library
//!
//! This library provides efficient bit vector and packed data structures
//! optimized for musical computations. It includes implementations for
//! 16-bit vectors, 4-bit packed vectors, and other specialized data types.

pub mod bitvec16;
pub mod u4vec16;
pub mod u4x2;

// Re-export the main types for convenience
pub use bitvec16::{BitVec16, BitVec16Builder};
pub use u4vec16::U4Vec16;
pub use u4x2::U4x2;
