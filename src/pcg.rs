use crate::formulas::{lcg64_jump, lcg64_step, xsh_rr_u64_to_u32};

const PCG32_MUL: u64 = 6364136223846793005;

/// A [Permuted congruential generator][wp] with 32-bit output.
///
/// [wp]: https://en.wikipedia.org/wiki/Permuted_congruential_generator
#[derive(Debug, Clone)]
pub struct PCG32 {
  /// The generator's state, changes with each step.
  pub state: u64,

  /// The generator's increment, doesn't change.
  ///
  /// If this is not an odd value the generator will have a reduced period
  /// (though the generator will still otherwise work).
  pub inc: u64,
}
impl PCG32 {
  /// Creates a new generator by directly using the value given.
  #[inline]
  #[must_use]
  pub const fn new(state: u64, inc: u64) -> Self {
    Self { state, inc }
  }

  /// Create a new generator seeded with data from
  /// [getrandom](getrandom::getrandom).
  #[cfg(feature = "getrandom")]
  #[inline]
  pub fn from_getrandom() -> Result<Self, getrandom::Error> {
    use bytemuck::bytes_of_mut;

    let mut buf = [0_u64; 2];
    getrandom::getrandom(bytes_of_mut(&mut buf))?;

    Ok(Self::new(buf[0], buf[1]))
  }

  /// Generate the next `u32` in the sequence.
  #[inline]
  pub fn next_u32(&mut self) -> u32 {
    let new_state = lcg64_step(PCG32_MUL, self.inc, self.state);
    let out = xsh_rr_u64_to_u32(self.state);
    self.state = new_state;
    out
  }

  /// Jump the generator the given number of steps forward in the sequence.
  ///
  /// Because the sequence is a loop, you can go "back" by `x` steps just by
  /// passing `x.wrapping_neg()`.
  #[inline]
  pub fn jump(&mut self, delta: u64) {
    self.state = lcg64_jump(PCG32_MUL, self.inc, self.state, delta);
  }
}

/// A [Permuted congruential generator][wp] with 32-bit output extended to `K`
/// dimensions.
///
/// [wp]: https://en.wikipedia.org/wiki/Permuted_congruential_generator
///
/// The `K` value determines the number of elements in the extension array.
///
/// * The generator's output sequence will be "`K`-dimensionally
///   equidistributed". In other words, any sequence of up to `K` elements in a
///   row will exist *somewhere* within the complete output sequence.
/// * For best results, `K` should be a power of 2. The type works with other
///   `K` values, but when `K` is a power of 2 then selecting an element from
///   the extension array is faster.
#[derive(Debug, Clone)]
pub struct PCG32X<const K: usize> {
  /// The generator's state, changes with each step.
  pub state: u64,

  /// The generator's increment, doesn't change.
  ///
  /// If this is not an odd value the generator will have a reduced period
  /// (though the generator will still otherwise work).
  pub inc: u64,

  /// The generator's extension array. The generator's base output is XOR'd with
  /// random elements of this array to determine the final output at each step.
  pub ext: [u32; K],
}
impl<const K: usize> PCG32X<K> {
  /// Creates a new generator by directly using the value given.
  #[inline]
  #[must_use]
  pub const fn new(state: u64, inc: u64, ext: [u32; K]) -> Self {
    Self { state, inc, ext }
  }

  /// Create a new generator seeded with data from
  /// [getrandom](getrandom::getrandom).
  #[cfg(feature = "getrandom")]
  #[inline]
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
  /// This will completely scramble the generator's position within the output
  /// sequence.
  #[cfg(feature = "getrandom")]
  #[inline]
  pub fn ext_getrandom(&mut self) -> Result<(), getrandom::Error> {
    use bytemuck::bytes_of_mut;

    getrandom::getrandom(bytes_of_mut(&mut self.ext))
  }

  /// Generate the next `u32` in the sequence.
  #[inline]
  pub fn next_u32(&mut self) -> u32 {
    let new_state = lcg64_step(PCG32_MUL, self.inc, self.state);
    let out = if K > 0 {
      let ext_index: usize = self.state as usize % K;
      let out = xsh_rr_u64_to_u32(self.state) ^ self.ext[ext_index];
      if self.state == 0 {
        self.ext_add(1)
      }
      out
    } else {
      xsh_rr_u64_to_u32(self.state)
    };
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
    if K == 0 {
      return;
    }
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
