pub const fn lcg32_step(mul: u32, add: u32, state: u32) -> u32 {
  state.wrapping_mul(mul).wrapping_add(add)
}

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

pub const fn lcg64_step(mul: u64, add: u64, state: u64) -> u64 {
  state.wrapping_mul(mul).wrapping_add(add)
}

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

pub const fn lcg128_step(mul: u128, add: u128, state: u128) -> u128 {
  state.wrapping_mul(mul).wrapping_add(add)
}

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

pub const fn xsh_rr_u64_to_u32(state: u64) -> u32 {
  let xsh: u32 = (((state >> 18) ^ state) >> 27) as u32;
  let rand_rot: u32 = (state >> 59) as u32;
  xsh.rotate_right(rand_rot)
}
