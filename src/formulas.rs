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

/// Returns `k` with probability `2^(-k-1)`, a "binary exponential
/// distribution".
pub fn next_binary_exp_distr32<F: FnMut() -> u32>(mut f: F) -> u32 {
  // Based on a function provided by <https://github.com/orlp>

  // Note(Lokathor): We want to calculate the number of trailing zeroes on a
  // result `r` from the generator. However, as long as `r` is 0 we act like
  // it's just an "extra" 32 trailing zeros and then generate again.
  let mut extra = 0;
  let mut r: u32 = f();
  while r == 0 {
    extra += 1;
    r = f();
  }
  extra * 32 + r.trailing_zeros()
}

/// Generates an `f32` in the signed or unsigned unit range.
///
/// * signed: `[-1.0, 1.0]`
/// * unsigned: `[0.0, 1.0]`
pub fn ieee754_random_f32<F: FnMut() -> u32>(mut f: F, signed: bool) -> f32 {
  // This function provided by <https://github.com/orlp>

  // Returns random number in [0, 1] or [-1, 1] depending on signed.
  let bit_width = 32;
  let exponent_bias = 127;
  let num_mantissa_bits = 23;
  let num_rest_bits = bit_width - num_mantissa_bits - 1 - signed as i32;
  let r: u32 = f();

  debug_assert!(num_rest_bits >= 0);
  debug_assert!(core::mem::size_of::<u32>() * 8 == bit_width as _);

  let mantissa = r >> (bit_width - num_mantissa_bits);
  let (sign_mask, rand_bit, rest_bits);
  if signed {
    sign_mask = r << (bit_width - 1);
    rand_bit = (r & 2) != 0;
    rest_bits = (r >> 2) & ((1 << num_rest_bits) - 1);
  } else {
    sign_mask = 0;
    rand_bit = (r & 1) != 0;
    rest_bits = (r >> 1) & ((1 << num_rest_bits) - 1);
  }

  // If our mantissa is zero, half of the time we must increase our exponent.
  let increment_exponent = (mantissa == 0 && rand_bit) as i32;

  // We can usually reuse `rest_bits` to save more calls to the rng.
  let computed_rest_bits: i32 = if rest_bits > 0 {
    rest_bits.trailing_zeros() as i32
  } else {
    num_rest_bits + next_binary_exp_distr32(&mut f) as i32
  };
  let mut exponent: i32 = -1 + increment_exponent - computed_rest_bits;

  // It is very unlikely our exponent is invalid at this point, but keep
  // regenerating it until it is valid.
  while exponent < -exponent_bias || exponent > 0 {
    exponent = -1 + increment_exponent - next_binary_exp_distr32(&mut f) as i32;
  }

  let final_exponent = ((exponent + exponent_bias) as u32) << num_mantissa_bits;
  f32::from_bits(sign_mask | final_exponent | mantissa)
}

/// Generates an `f32` in the signed or unsigned unit range.
///
/// * signed: `[-1.0, 1.0]`
/// * unsigned: `[0.0, 1.0]`
pub fn ieee754_random_f64<F: FnMut() -> u32>(mut f: F, signed: bool) -> f64 {
  // This function provided by <https://github.com/orlp>

  // Returns random number in [0, 1] or [-1, 1] depending on signed.
  let bit_width = 64;
  let exponent_bias = 1023;
  let num_mantissa_bits = 52;
  let num_rest_bits = bit_width - num_mantissa_bits - 1 - signed as i32;
  let r: u64 = ((f() as u64) << 32) | (f() as u64);

  debug_assert!(num_rest_bits >= 0);
  debug_assert!(core::mem::size_of::<u32>() * 8 == bit_width as _);

  let mantissa = r >> (bit_width - num_mantissa_bits);
  let (sign_mask, rand_bit, rest_bits);
  if signed {
    sign_mask = r << (bit_width - 1);
    rand_bit = (r & 2) != 0;
    rest_bits = (r >> 2) & ((1 << num_rest_bits) - 1);
  } else {
    sign_mask = 0;
    rand_bit = (r & 1) != 0;
    rest_bits = (r >> 1) & ((1 << num_rest_bits) - 1);
  }

  // If our mantissa is zero, half of the time we must increase our exponent.
  let increment_exponent = (mantissa == 0 && rand_bit) as i32;

  // We can usually reuse `rest_bits` to save more calls to the rng.
  let computed_rest_bits: i32 = if rest_bits > 0 {
    rest_bits.trailing_zeros() as i32
  } else {
    num_rest_bits + next_binary_exp_distr32(&mut f) as i32
  };
  let mut exponent: i32 = -1 + increment_exponent - computed_rest_bits;

  // It is very unlikely our exponent is invalid at this point, but keep
  // regenerating it until it is valid.
  while exponent < -exponent_bias || exponent > 0 {
    exponent = -1 + increment_exponent - next_binary_exp_distr32(&mut f) as i32;
  }

  let final_exponent = ((exponent + exponent_bias) as u64) << num_mantissa_bits;
  f64::from_bits(sign_mask | final_exponent | mantissa)
}
