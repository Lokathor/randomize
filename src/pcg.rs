use crate::formulas::{lcg32_step, lcg64_jump, lcg64_step, xsh_rr_u64_to_u32, PCG_MUL_64};

/// A [Permuted Congruential Generator][wp] with 32-bit output.
///
/// [wp]: https://en.wikipedia.org/wiki/Permuted_congruential_generator
///
/// * Period: `2**64` when `inc` is odd, otherwise less
#[derive(Debug, Clone)]
pub struct PCG32 {
  /// The generator's state.
  ///
  /// This changes with each step of the generator. It's the generator's
  /// "position" within the output stream.
  pub state: u64,

  /// The generator's increment.
  ///
  /// This doesn't change as the generator advances. Instead it determines which
  /// of the possible output streams the generator will use. Each `inc` value
  /// will give a different ordering of all the possible outputs.
  pub inc: u64,
}
impl PCG32 {
  /// Creates a new generator by directly using the value given.
  ///
  /// When a raw `state` value is selected manually, the initial output of the
  /// generator will frequently be 0. If the initial `state` is not from a
  /// randomization source then you should probably call [seed](Self::seed)
  /// instead.
  #[inline]
  #[must_use]
  pub const fn new(state: u64, inc: u64) -> Self {
    Self { state, inc }
  }

  /// Seed a new generator.
  #[inline]
  pub const fn seed(seed: u64, inc: u64) -> Self {
    let seed = (seed << 1) | 1;
    let inc = (inc << 1) | 1;
    let state = lcg64_step(PCG_MUL_64, inc, seed);
    Self { state, inc }
  }

  /// Create a new generator seeded with data from
  /// [getrandom](getrandom::getrandom).
  ///
  /// This method ensures that the `inc` of the new generator is odd.
  #[cfg(feature = "getrandom")]
  #[cfg_attr(docsrs, doc(cfg(feature = "getrandom")))]
  #[inline]
  pub fn from_getrandom() -> Result<Self, getrandom::Error> {
    use bytemuck::bytes_of_mut;

    let mut buf = [0_u64; 2];
    getrandom::getrandom(bytes_of_mut(&mut buf))?;

    Ok(Self::new(buf[0], buf[1] | 1))
  }

  /// Generate the next `u32` in the sequence.
  #[inline]
  pub fn next_u32(&mut self) -> u32 {
    let new_state = lcg64_step(PCG_MUL_64, self.inc, self.state);
    let out = xsh_rr_u64_to_u32(self.state);
    self.state = new_state;
    out
  }

  /// Jump the generator the given number of steps forward in the sequence.
  ///
  /// This can go `x` steps forward in only about `log2(x)` time.
  ///
  /// Because the sequence is a loop, you can go "back" by `x` steps just by
  /// passing `x.wrapping_neg()` to go sufficiently far forward.
  #[inline]
  pub fn jump(&mut self, delta: u64) {
    self.state = lcg64_jump(PCG_MUL_64, self.inc, self.state, delta);
  }
}

/// A [Permuted Congruential Generator][wp] with 32-bit output, extended to `K`
/// dimensions.
///
/// [wp]: https://en.wikipedia.org/wiki/Permuted_congruential_generator
///
/// This is like the [PCG32], but replaces the single `inc` field with an
/// extension array of `K` elements. At each step of the generator a different
/// element of the array is `XOR`-ed with the output, and every time the
/// generator passes 0 the array is advanced. Given enough time, the entire
/// extension array will increment through all values (as if it were a
/// `32*K`-bit number). This gives a radically larger generator period than the
/// basic `PCG32`, and guarantees that any sequence of `K` outputs in a row will
/// appear at least once within the generator's full output stream. Of course
/// it's highly unlikely that your program will ever advance the generator
/// through all possible states, but just the *possibility* that a given output
/// sequence will definitely eventually occur can be comfort enough.
///
/// For best results, `K` should be a power of 2. The type works with other `K`
/// values, but when `K` is a power of 2 then selecting an element from the
/// extension array is *significantly* faster (a bit mask instead of an integer
/// division).
///
/// * Period: `2**(64+32*k)`
#[derive(Debug, Clone)]
pub struct PCG32K<const K: usize> {
  /// The generator's state.
  ///
  /// This changes with each step of the generator. It's the generator's
  /// "position" within the output stream.
  pub state: u64,

  /// The generator's extension array. The generator's base output is XOR'd with
  /// random elements of this array to determine the final output at each step.
  pub ext: [u32; K],
}
impl<const K: usize> PCG32K<K> {
  /// Creates a new generator by directly using the value given.
  ///
  /// When a raw `state` value is selected manually, the initial output of the
  /// generator will frequently be 0. If the initial `state` is not from a
  /// randomization source then you should probably call [seed](Self::seed)
  /// instead.
  #[inline]
  #[must_use]
  pub const fn new(state: u64, ext: [u32; K]) -> Self {
    Self { state, ext }
  }

  /// Seed a new generator.
  #[inline]
  pub const fn seed(seed: u64, mut ext: [u32; K]) -> Self {
    let seed = (seed << 1) | 1;
    let state = lcg64_step(PCG_MUL_64, 1, seed);
    let mut i = 0;
    while i < K {
      ext[i] = lcg32_step(PCG_MUL_64 as u32, 1, ext[i]);
      i += 1;
    }
    Self { state, ext }
  }

  /// Create a new generator seeded with data from
  /// [getrandom](getrandom::getrandom).
  ///
  /// ## Failure
  /// * If the [getrandom](getrandom::getrandom) call fails the error bubbles
  ///   up.
  #[cfg(feature = "getrandom")]
  #[cfg_attr(docsrs, doc(cfg(feature = "getrandom")))]
  #[inline]
  pub fn from_getrandom() -> Result<Self, getrandom::Error> {
    use bytemuck::bytes_of_mut;

    let mut state = 0_u64;
    getrandom::getrandom(bytes_of_mut(&mut state))?;

    let mut out = Self::new(state, [0_u32; K]);
    out.scramble_ext_array()?;

    Ok(out)
  }

  /// Runs [getrandom](getrandom::getrandom) on the extension array.
  ///
  /// This will completely scramble the generator's position within the output
  /// sequence.
  ///
  /// ## Failure
  /// * If the [getrandom](getrandom::getrandom) call fails the error bubbles
  ///   up.
  #[cfg(feature = "getrandom")]
  #[cfg_attr(docsrs, doc(cfg(feature = "getrandom")))]
  #[inline]
  pub fn scramble_ext_array(&mut self) -> Result<(), getrandom::Error> {
    use bytemuck::bytes_of_mut;

    getrandom::getrandom(bytes_of_mut(&mut self.ext))
  }

  /// Generate the next `u32` in the sequence.
  #[inline]
  pub fn next_u32(&mut self) -> u32 {
    let new_state = lcg64_step(PCG_MUL_64, 1, self.state);
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
  /// next higher element, and so on.
  ///
  /// The generator will call this whenever its state value passes 0. This is an
  /// extremely rare event, and so we've manually "outlined" the extension array
  /// advancement.
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

#[test]
fn test_ext_add() {
  let mut x = PCG32K::<2> { state: 0, ext: [u32::MAX, 0] };
  x.ext_add(1);
  assert_eq!(x.ext[0], 0);
  assert_eq!(x.ext[1], 1);
  //
  let mut x = PCG32K::<3> { state: 0, ext: [u32::MAX, u32::MAX, 0] };
  x.ext_add(1);
  assert_eq!(x.ext[0], 0);
  assert_eq!(x.ext[1], 0);
  assert_eq!(x.ext[2], 1);
}
