#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_inline_in_public_items)]
#![cfg_attr(docs_rs, feature(doc_cfg))]

//! Pseudo-random number generator crate.
//!
//! NOT FOR CRYPTOGRAPHIC PURPOSES.
//!
//! ## Using This Crate
//!
//! * Create a [PCG32] or [PCG32K] value as your generator.
//!   * If you enable this crate's `getrandom` cargo feature then both types
//!     will have constructor functions to handle seeding a generator from the
//!     [getrandom](getrandom::getrandom) function.
//! * Call `next_u32` on the generator to get pseudo-random `u32` values.
//! * At your option, import the [Gen32] trait for various extension methods.

pub mod formulas;
use formulas::ieee754_random_f32;

mod pcg;
pub use pcg::*;

mod bounded_rand;
pub use bounded_rand::*;

/// A trait for pseudo-random number generators with 32-bit output per step.
pub trait Gen32 {
  /// Makes the generator create the next output.
  ///
  /// All `u32` values should have equal chance of occuring.
  fn next_u32(&mut self) -> u32;

  /// Gives a uniformly distributed value.
  #[inline]
  fn next_i32(&mut self) -> i32 {
    self.next_u32() as i32
  }

  /// Gives a uniformly distributed value.
  #[inline]
  fn next_bool(&mut self) -> bool {
    (self.next_u32() as i32) < 0
  }

  /// Gives a value in the range `0.0 ..= 1.0`
  #[inline]
  fn next_f32_unit(&mut self) -> f32 {
    ieee754_random_f32(|| self.next_u32(), false)
  }

  /// Gives a value in the range `1 ..= 4`
  #[inline]
  fn d4(&mut self) -> i32 {
    let base = self.next_u32() >> 30;
    base as i32 + 1
  }

  /// Gives a value in the range `1 ..= 6`
  #[inline]
  fn d6(&mut self) -> i32 {
    let base = BoundedRandU16::_6.sample(|| (self.next_u32() >> 16) as u16);
    i32::from(base) + 1
  }

  /// Gives a value in the range `1 ..= 8`
  #[inline]
  fn d8(&mut self) -> i32 {
    let base = self.next_u32() >> 29;
    base as i32 + 1
  }

  /// Gives a value in the range `1 ..= 10`
  #[inline]
  fn d10(&mut self) -> i32 {
    let base = BoundedRandU16::_10.sample(|| (self.next_u32() >> 16) as u16);
    i32::from(base) + 1
  }

  /// Gives a value in the range `1 ..= 12`
  #[inline]
  fn d12(&mut self) -> i32 {
    let base = BoundedRandU16::_12.sample(|| (self.next_u32() >> 16) as u16);
    i32::from(base) + 1
  }

  /// Gives a value in the range `1 ..= 20`
  #[inline]
  fn d20(&mut self) -> i32 {
    let base = BoundedRandU16::_20.sample(|| (self.next_u32() >> 16) as u16);
    i32::from(base) + 1
  }
}

impl Gen32 for PCG32 {
  #[inline]
  fn next_u32(&mut self) -> u32 {
    PCG32::next_u32(self)
  }
}
impl<const K: usize> Gen32 for PCG32K<K> {
  #[inline]
  fn next_u32(&mut self) -> u32 {
    PCG32K::<K>::next_u32(self)
  }
}
