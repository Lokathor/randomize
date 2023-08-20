#![no_std]

//! Pseudo-random number generator crate.
//!
//! NOT FOR CRYPTOGRAPHIC PURPOSES.

pub mod formulas;
pub mod pcg;

/// Stores the values to sample a `u32` number in `0 .. N`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundedRandU32 {
  /// number of possible outputs. outputs will be in `0 .. count`
  count: u32,
  /// Multiplication threshold thing.
  ///
  /// <https://arxiv.org/abs/1805.10941>
  threshold: u32,
}
impl BoundedRandU32 {
  pub const _4: Self = Self::new(4);
  pub const _6: Self = Self::new(6);
  pub const _8: Self = Self::new(8);
  pub const _10: Self = Self::new(10);
  pub const _12: Self = Self::new(12);
  pub const _20: Self = Self::new(20);

  /// Constructs a new value.
  ///
  /// ## Panics
  /// If the count is 0.
  #[inline]
  pub const fn new(count: u32) -> Self {
    let threshold = count.wrapping_neg() % count;
    Self { count, threshold }
  }

  /// Constructs a new value, or `None` on failure.
  ///
  /// ## Failure
  /// If the count is 0.
  #[inline]
  pub const fn try_new(count: u32) -> Option<Self> {
    if count > 0 {
      Some(Self::new(count))
    } else {
      None
    }
  }

  /// The number of possible outputs.
  #[inline]
  pub const fn count(self) -> u32 {
    self.count
  }

  /// Given a `u32`, try to place it into this bounded range.
  ///
  /// ## Failure
  /// * If the value is such that it doesn't fit evenly it is rejected.
  #[inline]
  pub const fn place_in_range(self, val: u32) -> Option<u32> {
    let mul: u64 = (val as u64).wrapping_mul(self.count as u64);
    let low_part: u32 = mul as u32;
    if low_part < self.threshold {
      None
    } else {
      debug_assert!(((mul >> 32) as u32) < self.count());
      Some((mul >> 32) as u32)
    }
  }

  /// Given a generator function, call it until
  /// [`place_in_range`](Self::place_in_range) succeeds.
  #[inline]
  pub fn sample<F: FnMut() -> u32>(self, mut f: F) -> u32 {
    loop {
      if let Some(output) = self.place_in_range(f()) {
        return output;
      }
    }
  }
}

/// Stores the values to sample a `u16` number in `0 .. N`
///
/// The primary advantage of this type over [BoundedRandU32] is that this type
/// samples using only 32-bit multiplications rather than 64-bit
/// multiplications, so on a 32-bit CPU this will perform much faster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundedRandU16 {
  /// number of possible outputs. outputs will be in `0 .. count`
  count: u16,
  /// Multiplication threshold thing.
  ///
  /// <https://arxiv.org/abs/1805.10941>
  threshold: u16,
}
impl BoundedRandU16 {
  pub const _4: Self = Self::new(4);
  pub const _6: Self = Self::new(6);
  pub const _8: Self = Self::new(8);
  pub const _10: Self = Self::new(10);
  pub const _12: Self = Self::new(12);
  pub const _20: Self = Self::new(20);

  /// Constructs a new value.
  ///
  /// ## Panics
  /// If the count is 0.
  #[inline]
  pub const fn new(count: u16) -> Self {
    let threshold = count.wrapping_neg() % count;
    Self { count, threshold }
  }

  /// Constructs a new value, or `None` on failure.
  ///
  /// ## Failure
  /// If the count is 0.
  #[inline]
  pub const fn try_new(count: u16) -> Option<Self> {
    if count > 0 {
      Some(Self::new(count))
    } else {
      None
    }
  }

  /// The number of possible outputs.
  #[inline]
  pub const fn count(self) -> u16 {
    self.count
  }

  /// Given a `u16`, try to place it into this bounded range.
  ///
  /// ## Failure
  /// * If the value is such that it doesn't fit evenly it is rejected.
  #[inline]
  pub const fn place_in_range(self, val: u16) -> Option<u16> {
    let mul: u32 = (val as u32).wrapping_mul(self.count as u32);
    let low_part: u16 = mul as u16;
    if low_part < self.threshold {
      None
    } else {
      debug_assert!(((mul >> 16) as u16) < self.count());
      Some((mul >> 16) as u16)
    }
  }

  /// Given a generator function, call it until
  /// [`place_in_range`](Self::place_in_range) succeeds.
  #[inline]
  pub fn sample<F: FnMut() -> u16>(self, mut f: F) -> u16 {
    loop {
      if let Some(output) = self.place_in_range(f()) {
        return output;
      }
    }
  }
}
