//! Free function utilities.

use super::*;

/// Returns `k` with probability `2^(-k-1)`, a "binary exponential
/// distribution".
#[inline]
pub fn next_binary_exp_distr32<G: Gen32 + ?Sized>(g: &mut G) -> u32 {
  // This function provided by <https://github.com/orlp>

  let r: u32 = g.next_u32();
  if r > 0 {
    r.trailing_zeros()
  } else {
    32 + next_binary_exp_distr32(g)
  }
}

/// Gives an `f32` output, in the unsigned (`[0,1]`) or signed (`[-1, 1]`)
/// range.
///
/// This is the "primitive" that the [`Gen32`] trait uses in its default
/// implementations for the `f32` methods. You might find it useful to use
/// yourself directly, so here you go.
pub fn ieee754_random_f32<G: Gen32 + ?Sized>(g: &mut G, signed: bool) -> f32 {
  // This function provided by <https://github.com/orlp>
  
  // Returns random number in [0, 1] or [-1, 1] depending on signed.
  let bit_width = 32;
  let exponent_bias = 127;
  let num_mantissa_bits = 23;
  let num_rest_bits = bit_width - num_mantissa_bits - 1 - signed as i32;
  let r: u32 = g.next_u32();

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

  // We can use rest_bits to save more calls to the rng.
  let mut exponent: i32 = -1 + increment_exponent
    - if rest_bits > 0 {
      rest_bits.trailing_zeros() as i32
    } else {
      num_rest_bits + next_binary_exp_distr32(g) as i32
    };

  // It is very unlikely our exponent is invalid at this point, but keep
  // regenerating it until it is valid.
  while exponent < -exponent_bias || exponent > 0 {
    exponent = -1 + increment_exponent - next_binary_exp_distr32(g) as i32;
  }

  f32::from_bits(sign_mask | (((exponent + exponent_bias) as u32) << num_mantissa_bits) | mantissa)
}

/// Gives an `f64` output, in the unsigned (`[0,1]`) or signed (`[-1, 1]`)
/// range.
///
/// This is the "primitive" that the [`Gen32`] trait uses in its default
/// implementations for the `f64` methods. You might find it useful to use
/// yourself directly, so here you go.
pub fn ieee754_random_f64<G: Gen32 + ?Sized>(g: &mut G, signed: bool) -> f64 {
  // This function provided by <https://github.com/orlp>
  
  // Returns random number in [0, 1] or [-1, 1] depending on signed.
  let bit_width = 64;
  let exponent_bias = 1023;
  let num_mantissa_bits = 52;
  let num_rest_bits = bit_width - num_mantissa_bits - 1 - signed as i32;
  let r: u64 = g.next_u64();

  debug_assert!(num_rest_bits >= 0);
  debug_assert!(core::mem::size_of::<u64>() * 8 == bit_width as _);

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

  // We can use rest_bits to save more calls to the rng.
  let mut exponent: i32 = -1 + increment_exponent
    - if rest_bits > 0 {
      rest_bits.trailing_zeros() as i32
    } else {
      num_rest_bits + next_binary_exp_distr32(g) as i32
    };

  // It is very unlikely our exponent is invalid at this point, but keep
  // regenerating it until it is valid.
  while exponent < -exponent_bias || exponent > 0 {
    exponent = -1 + increment_exponent - next_binary_exp_distr32(g) as i32;
  }

  f64::from_bits(sign_mask | (((exponent + exponent_bias) as u64) << num_mantissa_bits) | mantissa)
}
