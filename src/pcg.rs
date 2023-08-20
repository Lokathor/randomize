use crate::formulas::{lcg64_jump, lcg64_step, xsh_rr_u64_to_u32};

const PCG32_MUL: u64 = 6364136223846793005;

/// A [Permuted congruential generator][wp] with 32-bit output.
///
/// [wp]: https://en.wikipedia.org/wiki/Permuted_congruential_generator
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

  pub fn jump(&mut self, delta: u64) {
    self.state = lcg64_jump(PCG32_MUL, self.inc, self.state, delta);
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

    let mut out = Self::new(state_inc[0], state_inc[1], [0_u32; K]);
    out.ext_getrandom()?;

    Ok(out)
  }

  /// Runs [getrandom](getrandom::getrandom) on the extension array.
  ///
  /// This will completely scramble the output sequence.
  #[cfg(feature = "getrandom")]
  pub fn ext_getrandom(&mut self) -> Result<(), getrandom::Error> {
    use bytemuck::bytes_of_mut;

    getrandom::getrandom(bytes_of_mut(&mut self.ext))
  }

  pub fn next_u32(&mut self) -> u32 {
    let new_state = lcg64_step(PCG32_MUL, self.inc, self.state);
    let ext_index: usize = self.state as usize % K;
    let out = xsh_rr_u64_to_u32(self.state) ^ self.ext[ext_index];
    if self.state == 0 {
      self.ext_add(1)
    }
    self.state = new_state;
    out
  }

  /// Adds a value to the extension array.
  ///
  /// The given `delta` is added to the lowest index element of the extension
  /// array, and if that addition carries then the carry will add one to the
  /// next higher element, and so on. Normally the PCG will call this
  /// automatically whenever its state value passes 0.
  #[inline(never)]
  fn ext_add(&mut self, delta: u32) {
    let (new_ext, carry) = self.ext[0].overflowing_add(delta);
    self.ext[0] = new_ext;
    if carry {
      for ext in &mut self.ext[1..] {
        let (new_ext, carry) = ext.overflowing_add(1);
        *ext = new_ext;
        if carry {
          continue;
        } else {
          break;
        }
      }
    }
  }
}
