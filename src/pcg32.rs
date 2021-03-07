use super::*;

const PCG_MULTIPLIER_64: u64 = 6364136223846793005;

// Note(Lokathor): We're using these tiny macro_rules rather than tiny functions
// by request, because it makes the debug performance better, and helps keep the
// debug call stack cleaner.

/// Advances a PCG with 64 bits of state.
macro_rules! pcg_core_state64 {
  ($state:expr, $inc:expr) => {
    $state.wrapping_mul(PCG_MULTIPLIER_64).wrapping_add($inc)
  };
}

/// Permutation: XSH RR `u64` to `u32`.
macro_rules! xsh_rr_u64_to_u32 {
  ($state:expr) => {
    (((($state >> 18) ^ $state) >> 27) as u32).rotate_right(($state >> 59) as u32)
  };
}

make_jump_lcgX!(jump_lcg32, u64);

/// A [permuted congruential
/// generator](https://en.wikipedia.org/wiki/Permuted_congruential_generator)
/// with 32 bits of output per step.
///
/// * Generally you should create new generator values with the
///   [`seed`](Self::seed) constructor. This will shuffle around the inputs
///   somewhat, so it will work alright even with "boring" input values like
///   `seed(0,0)` or whatever.
/// * If you want to exactly save/restore a generator use the `Into` and `From`
///   impls to convert the generator into and from a `[u64; 2]`.
/// * The methods on this type are quite minimal. You're expected to use the
///   [`Gen32`] trait to provide most of the useful operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pcg32 {
  state: u64,
  inc: u64,
}

impl Pcg32 {
  /// Seed a new generator.
  pub const fn seed(seed: u64, inc: u64) -> Self {
    let inc = (inc << 1) | 1;
    let mut state = pcg_core_state64!(0_u64, inc);
    state = state.wrapping_add(seed);
    state = pcg_core_state64!(state, inc);
    Self { state, inc }
  }

  /// Seeds a new generator from the OS's randomness.
  #[cfg(feature = "os_random")]
  pub fn seed_from_os() -> Self {
    let mut x = [0_u64; 2];
    let _ = crate::fill_byte_buffer_from_os_random(unsafe {
      core::slice::from_raw_parts_mut(x.as_mut_ptr().cast::<u8>(), 2 * core::mem::size_of::<u64>())
    });
    let [seed, inc] = x;
    Self::seed(seed, inc)
  }

  /// Gets the next 32-bits of output.
  #[inline]
  pub fn next_u32(&mut self) -> u32 {
    // LLVM do the instruction-level parallelism plz ;_;
    let out = xsh_rr_u64_to_u32!(self.state);
    self.state = pcg_core_state64!(self.state, self.inc);
    out
  }

  /// Jumps the generator by `delta` steps forward.
  ///
  /// The generator sequence loops, so if you want to go "backwards" you can
  /// just subtract the number of steps you want to go back from `u64::MAX` and
  /// jump by that amount.
  #[inline]
  pub fn jump(&mut self, delta: u64) {
    self.state = jump_lcg32(delta, self.state, PCG_MULTIPLIER_64, self.inc);
  }
}

impl Default for Pcg32 {
  fn default() -> Self {
    const THE_DEFAULT: Pcg32 = Pcg32::seed(DEFAULT_PCG_SEED as _, DEFAULT_PCG_INC as _);
    THE_DEFAULT
  }
}

impl From<[u64; 2]> for Pcg32 {
  fn from([state, inc]: [u64; 2]) -> Self {
    Self { state, inc }
  }
}

impl From<Pcg32> for [u64; 2] {
  fn from(pcg: Pcg32) -> Self {
    [pcg.state, pcg.inc]
  }
}

impl Gen32 for Pcg32 {
  fn next_u32(&mut self) -> u32 {
    Pcg32::next_u32(self)
  }
}
