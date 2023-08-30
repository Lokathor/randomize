#![no_std]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_inline_in_public_items)]
#![cfg_attr(docs_rs, feature(doc_cfg))]

//! Pseudo-random number generator crate.
//!
//! NOT FOR CRYPTOGRAPHIC PURPOSES.
//!
//! ## Cargo Features
//!
//! * `getrandom`: This adds additional methods that use the `getrandom` crate
//!   to initialize a generator from an external randomness source.

mod bounded_rand;
pub use bounded_rand::*;

mod pcg;
pub use pcg::*;

pub mod formulas;

/// A trait for pseudo-random number generators with 32-bit output per step.
pub trait Gen32 {
  /// Makes the generator create the next output.
  fn next_u32(&mut self) -> u32;
}

impl Gen32 for PCG32 {
  #[inline]
  fn next_u32(&mut self) -> u32 {
    PCG32::next_u32(self)
  }
}
impl<const K: usize> Gen32 for PCG32X<K> {
  #[inline]
  fn next_u32(&mut self) -> u32 {
    PCG32X::<K>::next_u32(self)
  }
}
