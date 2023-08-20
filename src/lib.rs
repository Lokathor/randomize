#![no_std]

//! Pseudo-random number generator crate.
//!
//! NOT FOR CRYPTOGRAPHIC PURPOSES.

use formulas::{lcg64_step, xsh_rr_u64_to_u32};

pub mod formulas;

#[derive(Debug, Clone)]
pub struct PCG32 {
  pub state: u64,
  pub inc: u64,
}
impl PCG32 {
  const MUL: u64 = 6364136223846793005;

  pub const fn new(state: u64, inc: u64) -> Self {
    Self { state, inc }
  }

  #[cfg(feature = "getrandom")]
  pub fn from_os() -> Self {
    let bytes = [0_u8; size_of::<Self>()];
    todo!()
  }

  pub fn next_u32(&mut self) -> u32 {
    let new_state = lcg64_step(Self::MUL, self.inc, self.state);
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
  const MUL: u64 = 6364136223846793005;

  pub const fn new(state: u64, inc: u64, ext: [u32; K]) -> Self {
    Self { state, inc, ext }
  }

  #[cfg(feature = "getrandom")]
  pub fn from_os() -> Self {
    let bytes = [0_u8; size_of::<Self>()];
    todo!()
  }

  pub fn next_u32(&mut self) -> u32 {
    let new_state = lcg64_step(Self::MUL, self.inc, self.state);
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
