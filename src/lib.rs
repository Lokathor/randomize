#![no_std]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_inline_in_public_items)]

//! Pseudo-random number generator crate.
//!
//! NOT FOR CRYPTOGRAPHIC PURPOSES.

mod bounded_rand;
pub use bounded_rand::*;

mod pcg;
pub use pcg::*;

pub mod formulas;
