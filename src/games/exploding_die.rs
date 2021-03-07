use super::*;

/// Stores data for an "exploding" 1 through `N` sided die.
///
/// When rolled, if a maximum value is rolled, then the die is rolled again and
/// added to the total. Successive rolls can also trigger additional rolls on a
/// maximum value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ExplodingDie(StandardDie);
impl ExplodingDie {
  /// Constructs an exploding die.
  ///
  /// ## Panics
  /// If the count is 0.
  #[inline]
  pub const fn new(sides: u32) -> Self {
    Self(StandardDie::new(sides))
  }

  /// The number of sides of this die.
  #[inline]
  pub const fn sides(self) -> i32 {
    self.0.sides()
  }

  /// Sample from the generator to perform an exploding roll.
  #[inline]
  pub fn sample<G: Gen32 + ?Sized>(self, gen: &mut G) -> i32 {
    let mut t: i32 = 0;
    while self.0.sample(gen) == self.0.sides() {
      t = t.wrapping_add(1).wrapping_add(self.sides());
    }
    t.wrapping_add(self.0.sample(gen) as i32)
  }
}

/// A 4-sided exploding die.
pub const X4: ExplodingDie = ExplodingDie::new(4);
/// A 6-sided exploding die.
pub const X6: ExplodingDie = ExplodingDie::new(6);
/// An 8-sided exploding die.
pub const X8: ExplodingDie = ExplodingDie::new(8);
/// A 10-sided exploding die.
pub const X10: ExplodingDie = ExplodingDie::new(10);
/// A 12-sided exploding die.
pub const X12: ExplodingDie = ExplodingDie::new(12);
/// A 20-sided exploding die.
pub const X20: ExplodingDie = ExplodingDie::new(20);
