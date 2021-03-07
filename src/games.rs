//! Types for rolling dice.

use super::*;

mod standard_die;
pub use standard_die::*;

mod exploding_die;
pub use exploding_die::*;

/// Performs an `XdY` style dice roll, such as `3d6`.
///
/// * If `sides` is 0 or less, the output will be 0.
/// * If `count` is negative the output will be negative.
/// * Runs in `O(abs(count))` time.
/// * Expected `count` values are generally -20 to +20.
///
/// ```
/// use randomize::{games::dice, Pcg32};
/// let g = &mut Pcg32::default();
///
/// let pos = dice(g, 3, 6);
/// assert!(pos >= 3 && pos <= 18);
///
/// let neg = dice(g, -2, 4);
/// assert!(neg >= -8 && neg <= -2);
/// ```
#[inline]
pub fn dice(g: &mut impl Gen32, mut count: i32, sides: i32) -> i32 {
  use core::cmp::Ordering;
  let range = match sides.cmp(&1) {
    Ordering::Less => return 0,
    Ordering::Equal => return count,
    Ordering::Greater => match sides {
      4 => D4,
      6 => D6,
      8 => D8,
      10 => D10,
      12 => D12,
      20 => D20,
      _ => StandardDie::new(sides as u32),
    },
  };
  let mut t = 0_i32;
  if count >= 0 {
    while count > 0 {
      t = t.wrapping_add(range.sample(g));
      count -= 1;
    }
  } else {
    while count < 0 {
      t = t.wrapping_sub(range.sample(g));
      count += 1;
    }
  }
  t
}

/// Performs an Earthdawn 4e "step" roll.
///
/// * The overall average output value is bell-curved(ish) around the input.
/// * Technically this is `O(step)` time to compute. More precisely, starting
///   from step 1, every +7 steps adds +1 RNG call required.
/// * If `step` is negative, the output will be negated.
/// * Expected `step` values are 1 to 30.
#[inline]
pub fn step_ed4(g: &mut impl Gen32, mut step: i32) -> i32 {
  use core::cmp::Ordering;
  match step.cmp(&0) {
    Ordering::Equal => return 0,
    Ordering::Less => -step_ed4(g, -step),
    Ordering::Greater => {
      let mut total: i32 = 0;
      while step > 13 {
        total = total.wrapping_add(X12.sample(g));
        step -= 7;
      }
      total.wrapping_add(match step {
        1 => X4.sample(g).wrapping_sub(2).max(1),
        2 => X4.sample(g).wrapping_sub(1).max(1),
        3 => X4.sample(g),
        4 => X6.sample(g),
        5 => X8.sample(g),
        6 => X10.sample(g),
        7 => X12.sample(g),
        8 => X6.sample(g).wrapping_add(X6.sample(g)),
        9 => X8.sample(g).wrapping_add(X6.sample(g)),
        10 => X8.sample(g).wrapping_add(X8.sample(g)),
        11 => X10.sample(g).wrapping_add(X8.sample(g)),
        12 => X10.sample(g).wrapping_add(X10.sample(g)),
        13 => X12.sample(g).wrapping_add(X10.sample(g)),
        _ => unreachable!(),
      })
    }
  }
}

/// Rolls an After Sundown style dice pool.
///
/// * `size` is the number of D6s to roll.
/// * The output is the number of rolls that showed a 5 or a 6.
/// * If `size` is negative the output will be negative.
#[inline]
pub fn after_sundown(g: &mut impl Gen32, mut size: i32) -> i32 {
  let mut hits = 0;
  let sign = size.signum();
  while size != 0 {
    if D6.sample(g) >= 5 {
      hits += sign;
    }
    size -= sign;
  }
  hits
}

/// Returns a value in `0..x` with the odds modified by `luck`.
///
/// This pertains to a particular video game. If you're not familiar with the
/// game that's fine.
/// * This is a constant time operation.
/// * higher luck pushes the output towards **zero**.
/// * lower luck pushes the output towards **x**.
/// * `luck` is expected to be +/-30.
///
/// ## Panics
/// * If `x` is 0 or less.
#[inline]
pub fn rn_bounded_luck(g: &mut impl Gen32, x: i32, luck: i32) -> i32 {
  assert!(x > 0);
  let adjustment = if x <= 15 { (luck.abs() + 1) / 3 * luck.signum() } else { luck };
  let mut i = g.next_bounded(x as u32) as i32;
  if adjustment != 0 && g.next_bounded(37 + adjustment.abs() as u32) != 0 {
    i -= adjustment;
  }
  i.max(0).min(x - 1)
}

/// Returns a value of 1 or more.
///
/// * The output starts at 1, then has a repeated `1/x` chance of getting +1.
/// * As soon as the value doesn't get a +1, it is returned.
///
/// ## Panics
/// * If `x` is less than 2.
#[inline]
pub fn rn_exponential_decay(g: &mut impl Gen32, x: i32) -> i32 {
  assert!(x > 1);
  let mut temp = 1;
  while g.next_bounded(x as u32) == 0 {
    temp += 1;
  }
  temp
}

/// Returns a... value.
///
/// * The `input` value affects the output.
/// * This runs in constant time.
#[inline]
pub fn rn_z(g: &mut impl Gen32, input: i32) -> i32 {
  let mut x = input as i64;
  let mut temp = 1000_i64;
  temp += g.next_bounded(1000) as i64;
  temp *= rn_exponential_decay(g, 4).min(5) as i64;
  if g.next_bool() {
    x *= temp;
    x /= 1000;
  } else {
    x *= 1000;
    x /= temp;
  }
  x as i32
}
