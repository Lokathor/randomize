/// Advance a 32-bit LCG's state.
pub const fn lcg32_step(mul: u32, add: u32, state: u32) -> u32 {
  state.wrapping_mul(mul).wrapping_add(add)
}

/// Advance a 32-bit LCG by `delta` steps in `log2(delta)` time.
pub const fn lcg32_jump(mul: u32, add: u32, state: u32, mut delta: u32) -> u32 {
  let mut cur_mult: u32 = mul;
  let mut cur_plus: u32 = add;
  let mut acc_mult: u32 = 1;
  let mut acc_plus: u32 = 0;
  while delta > 0 {
    if (delta & 1) > 0 {
      acc_mult = acc_mult.wrapping_mul(cur_mult);
      acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
    }
    cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
    cur_mult = cur_mult.wrapping_mul(cur_mult);
    delta /= 2;
  }
  state.wrapping_mul(acc_mult).wrapping_add(acc_plus)
}

/// Advance a 32-bit LCG's state.
pub const fn lcg64_step(mul: u64, add: u64, state: u64) -> u64 {
  state.wrapping_mul(mul).wrapping_add(add)
}

/// Advance a 32-bit LCG by `delta` steps in `log2(delta)` time.
pub const fn lcg64_jump(mul: u64, add: u64, state: u64, mut delta: u64) -> u64 {
  let mut cur_mult: u64 = mul;
  let mut cur_plus: u64 = add;
  let mut acc_mult: u64 = 1;
  let mut acc_plus: u64 = 0;
  while delta > 0 {
    if (delta & 1) > 0 {
      acc_mult = acc_mult.wrapping_mul(cur_mult);
      acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
    }
    cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
    cur_mult = cur_mult.wrapping_mul(cur_mult);
    delta /= 2;
  }
  state.wrapping_mul(acc_mult).wrapping_add(acc_plus)
}

/// Advance a 32-bit LCG's state.
pub const fn lcg128_step(mul: u128, add: u128, state: u128) -> u128 {
  state.wrapping_mul(mul).wrapping_add(add)
}

/// Advance a 32-bit LCG by `delta` steps in `log2(delta)` time.
pub const fn lcg128_jump(mul: u128, add: u128, state: u128, mut delta: u128) -> u128 {
  let mut cur_mult: u128 = mul;
  let mut cur_plus: u128 = add;
  let mut acc_mult: u128 = 1;
  let mut acc_plus: u128 = 0;
  while delta > 0 {
    if (delta & 1) > 0 {
      acc_mult = acc_mult.wrapping_mul(cur_mult);
      acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
    }
    cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
    cur_mult = cur_mult.wrapping_mul(cur_mult);
    delta /= 2;
  }
  state.wrapping_mul(acc_mult).wrapping_add(acc_plus)
}

/// "Xor-shift high bits" then "randomized rotate", `u64` down to `u32`.
pub const fn xsh_rr_u64_to_u32(state: u64) -> u32 {
  // Note(Lokathor): Bit randomness quality is better in the higher bits. The
  // top 5 bits specify the random rotation, while the next 32 bits are the
  // "source" of the final value. The xor-shift amount is half the total bits in
  // play, so it's (32+5)/2==18.
  let rot_amount: u32 = (state >> (64 - 5)) as u32;
  let xor_shifted: u64 = state ^ (state >> ((32 + 5) / 2));
  let kept_bits: u32 = (xor_shifted >> (32 - 5)) as u32;
  kept_bits.rotate_right(rot_amount)
}

/// "Xor-shift low bits" then "randomized rotate", `u128` to `u64`.
pub const fn xsl_rr_u128_to_u64(state: u128) -> u64 {
  // Note(Lokathor): Similar ideas to how `xsh_rr_u64_to_u32` works. At 128-bit
  // size we now use 6 bits to select the random rotation. The xor-shift is by
  // exactly 64 bits because on modern computers a `u128` will actually be two
  // 64-bit registers. Having the xor-shift be by exactly 64 bits makes the
  // operation is a simple `reg1 ^ reg2`.
  let rot_amount: u32 = (state >> (128 - 6)) as u32;
  let folded_bits: u64 = (state ^ (state >> 64)) as u64;
  folded_bits.rotate_right(rot_amount)
}
