/// Permutation: XSL RR RR `u64`
#[must_use]
#[inline(always)]
const fn xsl_rr_rr_64_64(state: u64) -> u64 {
  let rot1: u32 = (state >> 59) as u32;
  let high: u32 = (state >> 32) as u32;
  let low: u32 = state as u32;
  let xor_d: u32 = high ^ low;
  let new_low: u32 = xor_d.rotate_right(rot1);
  let new_high: u32 = high.rotate_right(new_low & 31);
  ((new_high as u64) << 32) | new_low as u64
}

/// For random number generators with a primary output of `u32` per use.
///
/// All other methods then default in terms of the `next_u32` method.
pub trait Gen32 {
  /// Produce a `u32`
  fn next_u32(&mut self) -> u32;

  /// Produce a `bool`
  #[inline(always)]
  fn next_bool(&mut self) -> bool {
    (self.next_u32() as i32) < 0
  }

  /// Produce a `u8`
  #[inline(always)]
  fn next_u8(&mut self) -> u8 {
    (self.next_u32() >> 24) as u8
  }

  /// Produce a `u16`
  #[inline(always)]
  fn next_u16(&mut self) -> u16 {
    (self.next_u32() >> 16) as u16
  }

  /// Produce a `u64`
  #[inline(always)]
  fn next_u64(&mut self) -> u64 {
    let l = self.next_u32() as u64;
    let h = self.next_u32() as u64;
    h << 32 | l
  }

  /// Returns an `f32` in the unsigned unit range, `[0, 1]`
  #[inline]
  fn next_f32_unit(&mut self) -> f32 {
    ieee754_random_f32(self, true)
  }

  /// Returns an `f32` in the signed unit range, `[-1, 1]`
  #[inline]
  fn next_f32_signed_unit(&mut self) -> f32 {
    ieee754_random_f32(self, false)
  }

  /// Gives a value within `0 .. B`
  ///
  /// This is often more efficient than making a [`BoundedRandU32`] if you don't
  /// need to use a specific bound value more than once.
  ///
  /// ## Panics
  /// * If the input is 0.
  #[inline]
  fn next_bounded(&mut self, b: u32) -> u32 {
    assert!(b != 0, "Gen32::next_bounded> Bound must be non-zero.");
    let mut x = self.next_u32() as u64;
    let mut mul = (b as u64).wrapping_mul(x);
    let mut low = mul as u32;
    if low < b {
      let threshold = b.wrapping_neg() % b;
      while low < threshold {
        x = self.next_u32() as u64;
        mul = (b as u64).wrapping_mul(x);
        low = mul as u32;
      }
    }
    let high = (mul >> 32) as u32;
    high
  }

  /// Performs an `XdY` style dice roll.
  ///
  /// * If `count` or `sides` are less than 0, the output is 0.
  /// * Requires linear time to compute based on `count`. Expected inputs are 20
  ///   or less.
  #[inline]
  fn dice(&mut self, mut count: i32, sides: i32) -> i32 {
    use core::cmp::Ordering;
    let range = match sides.cmp(&1) {
      Ordering::Less => return 0,
      Ordering::Equal => return count.max(0),
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
    while count > 0 {
      t = t.wrapping_add(range.sample(self));
      count -= 1;
    }
    t
  }

  /// Performs a "step" roll according to the 4e chart.
  ///
  /// This relates to a particular paper and pencil RPG. If you're not familiar
  /// with the game that's fine.
  /// * The average output of any positive value is approximately equal to the
  ///   input, with no hard upper bound.
  /// * The output of any non-positive value is 1.
  /// * Requires linear time to compute. Expected inputs are 30 or less.
  #[inline]
  fn step_ed4(&mut self, mut step: i32) -> i32 {
    if step < 1 {
      1
    } else {
      let mut total: i32 = 0;
      while step > 13 {
        total = total.wrapping_add(X12.sample(self));
        step -= 7;
      }
      total.wrapping_add(match step {
        1 => X4.sample(self).wrapping_sub(2).max(1),
        2 => X4.sample(self).wrapping_sub(1).max(1),
        3 => X4.sample(self),
        4 => X6.sample(self),
        5 => X8.sample(self),
        6 => X10.sample(self),
        7 => X12.sample(self),
        8 => X6.sample(self).wrapping_add(X6.sample(self)),
        9 => X8.sample(self).wrapping_add(X6.sample(self)),
        10 => X8.sample(self).wrapping_add(X8.sample(self)),
        11 => X10.sample(self).wrapping_add(X8.sample(self)),
        12 => X10.sample(self).wrapping_add(X10.sample(self)),
        13 => X12.sample(self).wrapping_add(X10.sample(self)),
        _ => unreachable!(),
      })
    }
  }

  /// Rolls an After Sundown style dice pool.
  ///
  /// This relates to a particular paper and pencil RPG. If you're not familiar
  /// with the game that's fine.
  /// * `size` D6s are rolled. This returns the number of them that are a 5 or
  ///   6.
  #[inline]
  fn sundown_pool(&mut self, mut size: u32) -> u32 {
    let mut hits = 0;
    while size > 0 {
      if D6.sample(self) >= 5 {
        hits += 1
      }
      size -= 1;
    }
    hits
  }

  /// Returns a value in `0..x` with the odds modified by `luck`.
  ///
  /// This pertains to a particular video game. If you're not familiar with the
  /// game that's fine.
  /// * This is a constant time operation.
  /// * higher luck pushes the output towards zero.
  /// * lower luck pushes the output towards the upper value.
  /// * `luck` is expected to be +/-30
  ///
  /// ## Panics
  /// * If `x` is 0 or less.
  #[inline]
  fn rn_bounded_luck(&mut self, x: i32, luck: i32) -> i32 {
    assert!(x > 0);
    let adjustment = if x <= 15 { (luck.abs() + 1) / 3 * luck.signum() } else { luck };
    let mut i = self.next_bounded(x as u32) as i32;
    if adjustment != 0 && self.next_bounded(37 + adjustment.abs() as u32) != 0 {
      i -= adjustment;
      i = i.max(0).min(x - 1);
    }
    i
  }

  /// Returns a value of 1 or more.
  ///
  /// * The output starts at 1, then has a repeated `1/x` chance of getting +1.
  /// * As soon as the value doesn't get a +1, it is returned.
  ///
  /// ## Panics
  /// * If `x` is less than 2.
  #[inline]
  fn rn_exponential_decay(&mut self, x: i32) -> i32 {
    assert!(x > 1);
    let mut temp = 1;
    while self.next_bounded(x as u32) == 0 {
      temp += 1;
    }
    temp
  }

  /// Returns a value.
  ///
  /// This pertains to a particular video game. If you're not familiar with the
  /// game that's fine.
  /// * The input value affects the output.
  /// * This runs in constant time.
  #[inline]
  fn rn_z(&mut self, i: i32) -> i32 {
    let mut x = i as i64;
    let mut temp = 1000_i64;
    temp += self.next_bounded(1000) as i64;
    temp *= self.rn_exponential_decay(4).min(5) as i64;
    if self.next_bool() {
      x *= temp;
      x /= 1000;
    } else {
      x *= 1000;
      x /= temp;
    }
    x as i32
  }

  /// Gets a value out of the slice given (by copy).
  ///
  /// * The default impl will not pick past index `u32::MAX`.
  #[inline(always)]
  fn pick<T>(&mut self, buf: &[T]) -> T
  where
    Self: Sized,
    T: Copy,
  {
    let end: u32 = saturating_usize_as_u32(buf.len());
    buf[usize::try_from(self.next_bounded(end)).unwrap()]
  }

  /// Gets a value out of the slice given (by shared ref).
  ///
  /// * The default impl will not pick past index `u32::MAX`.
  #[inline(always)]
  fn pick_ref<'b, T>(&mut self, buf: &'b [T]) -> &'b T
  where
    Self: Sized,
  {
    let end: u32 = saturating_usize_as_u32(buf.len());
    &buf[usize::try_from(self.next_bounded(end)).unwrap()]
  }

  /// Gets a value out of the slice given (by unique ref).
  ///
  /// * The default impl will not pick past index `u32::MAX`.
  #[inline(always)]
  fn pick_mut<'b, T>(&mut self, buf: &'b mut [T]) -> &'b mut T
  where
    Self: Sized,
  {
    let end: u32 = saturating_usize_as_u32(buf.len());
    &mut buf[usize::try_from(self.next_bounded(end)).unwrap()]
  }

  /// Shuffles a slice in `O(len)` time.
  ///
  /// * The default impl shuffles only the first `u32::MAX` elements.
  #[inline]
  fn shuffle<T>(&mut self, buf: &mut [T])
  where
    Self: Sized,
  {
    // Note(Lokathor): The "standard" Fisher-Yates shuffle goes backward from
    // the end of the slice, but this version allows us to access memory forward
    // from the start to the end, so that we play more nicely with the
    // fetch-ahead of most modern CPUs.
    let mut possibility_count: u32 = buf.len().try_into().unwrap_or(u32::max_value());
    let mut this_index: usize = 0;
    let end = buf.len() - 1;
    while this_index < end {
      let offset = self.next_bounded(possibility_count) as usize;
      buf.swap(this_index, this_index + offset);
      possibility_count -= 1;
      this_index += 1;
    }
  }
}
