#![no_std]

//! Pseudo-random number generator crate.
//!
//! NOT FOR CRYPTOGRAPHIC PURPOSES.

use formulas::{lcg64_step, xsh_rr_u64_to_u32};

pub mod formulas;

const PCG32_MUL: u64 = 6364136223846793005;

#[derive(Debug, Clone)]
pub struct PCG32 {
  pub state: u64,
  pub inc: u64,
}
impl PCG32 {
  pub const fn new(state: u64, inc: u64) -> Self {
    Self { state, inc }
  }

  #[cfg(feature = "getrandom")]
  pub fn from_getrandom() -> Result<Self, getrandom::Error> {
    use bytemuck::bytes_of_mut;

    let mut buf = [0_u64; 2];
    getrandom::getrandom(bytes_of_mut(&mut buf))?;

    Ok(Self::new(buf[0], buf[1]))
  }

  pub fn next_u32(&mut self) -> u32 {
    let new_state = lcg64_step(PCG32_MUL, self.inc, self.state);
    let out = xsh_rr_u64_to_u32(self.state);
    self.state = new_state;
    out
  }
}

#[derive(Debug, Clone)]
pub struct PCG32X<const K: usize> {
  pub state: u64,
  pub inc: u64,
  pub ext: [u32; K],
}
impl<const K: usize> PCG32X<K> {
  pub const fn new(state: u64, inc: u64, ext: [u32; K]) -> Self {
    Self { state, inc, ext }
  }

  #[cfg(feature = "getrandom")]
  pub fn from_getrandom() -> Result<Self, getrandom::Error> {
    use bytemuck::bytes_of_mut;

    let mut state_inc = [0_u64; 2];
    getrandom::getrandom(bytes_of_mut(&mut state_inc))?;

    let mut ext = [0_u32; K];
    getrandom::getrandom(bytes_of_mut(&mut ext))?;

    Ok(Self::new(state_inc[0], state_inc[1], ext))
  }

  pub fn next_u32(&mut self) -> u32 {
    let new_state = lcg64_step(PCG32_MUL, self.inc, self.state);
    let ext_index: usize = self.state as usize % K;
    let out = xsh_rr_u64_to_u32(self.state) ^ self.ext[ext_index];
    let mut carry = new_state == 0;
    let mut i = 0;
    while carry && i < K {
      let (new_ext, new_carry) = self.ext[i].overflowing_add(1);
      carry = new_carry;
      self.ext[i] = new_ext;
      i += 1;
    }
    self.state = new_state;
    out
  }
}

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
