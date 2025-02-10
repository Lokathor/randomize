#![allow(clippy::if_same_then_else)]
#![allow(clippy::collapsible_if)]

use randomize::{Gen32, PCG32K};

type Gen = PCG32K<1024>;

fn main() {
  let mut gen: Gen = Gen::from_getrandom().unwrap();
  let trials = 100000000;

  let mut total = 0_u64;
  for _ in 0..trials {
    //total += combi_weapon(&mut gen);
    //total += sternguard_rifle(&mut gen, false);
    total += heavy_rifle(&mut gen, false);
  }
  println!("{}", (total as f64) / (trials as f64));
}

/// **Returns:** Damage from a single shot.
pub fn combi_weapon(gen: &mut Gen) -> u64 {
  let to_hit = 4;
  let to_wound = 4;
  //
  if gen.d6() >= to_hit || gen.d6() >= to_hit {
    if gen.d6() >= to_wound || gen.d6() >= to_wound {
      return 1;
    }
  }

  0
}

/// **Returns:** Damage from a single shot.
pub fn sternguard_rifle(gen: &mut Gen, heavy: bool) -> u64 {
  let to_hit = 3 - (heavy as i32);
  //
  if gen.d6() >= to_hit || gen.d6() >= to_hit {
    let wound_roll = gen.d6();
    match wound_roll {
      6 => {
        return 1;
      }
      _ => {
        let wound_roll = gen.d6();
        match wound_roll {
          6 => return 1,
          4..=5 => {
            if gen.d6() < 3 {
              return 1;
            }
          }
          _ => (),
        }
      }
    }
  }

  0
}

/// **Returns:** Damage from a single shot.
pub fn heavy_rifle(gen: &mut Gen, heavy: bool) -> u64 {
  let to_hit = 4 - (heavy as i32);
  //
  let mut hits = 0;
  let hit_roll = gen.d6();
  if hit_roll == 6 {
    hits = 2;
  } else if hit_roll >= to_hit {
    hits = 1;
  } else {
    let hit_roll = gen.d6();
    if hit_roll == 6 {
      hits = 2;
    } else if hit_roll >= to_hit {
      hits = 1;
    } else {
      //
    }
  }

  let mut damage = 0;
  for _ in 0..hits {
    let wound_roll = gen.d6();
    match wound_roll {
      6 => {
        damage += 2;
      }
      _ => {
        let wound_roll = gen.d6();
        match wound_roll {
          6 => damage += 2,
          4..=5 => {
            if gen.d6() < 3 {
              damage += 2;
            }
          }
          _ => (),
        }
      }
    }
  }

  damage
}
