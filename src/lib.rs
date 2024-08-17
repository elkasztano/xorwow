//! Several implementations of the xorwow algorithm described here:
//! [http://www.jstatsoft.org/v08/i14/paper](http://www.jstatsoft.org/v08/i14/paper).
//! The last element of the underlying `u32` array is used as
//! the counter for the so-called Weyl sequence.
//! `xorwowgen::Xorwow160` comes closest to the 'xorwow' function described
//! in the paper. All other implementations are derivative.
//!
//! # Example
//! ```
//! use rand_core::{SeedableRng, RngCore};
//! use xorwowgen::Xorwow160;
//!
//! // initialize the generator
//! let mut rng = Xorwow160::seed_from_u64(1234);
//!
//! // clock it a few times
//! for _ in 0..100 {
//!     rng.next_u32();
//! }
//!
//! assert_eq!(2581263997, rng.next_u32());
//! ```
//!
//! # Features
//!
//! ### serde1
//! Allows (de)serialization of the state array using
//! [serde](https://serde.rs/).

use rand_core::impls::fill_bytes_via_next;
use rand_core::le::read_u32_into;
use rand_core::{Error, RngCore, SeedableRng};
use std::ops::BitXor;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

macro_rules! make_xorwow {
    ($(#[$meta:meta])*
     $name: ident, $nr: expr) => (
        $(#[$meta])*
        #[derive(Debug, Default, Clone, Eq, PartialEq)]
        #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
        pub struct $name {
            s: [u32; $nr]
        }
    )
}

make_xorwow!(
/// Xorwow implementation with __96__ bits of state
/// plus 32 bits for the modulo 2^32 counter.
///
/// # Example
/// ```
/// use rand_core::{SeedableRng, RngCore};
/// use xorwowgen::Xorwow96;
///
/// let mut rng = Xorwow96::seed_from_u64(4321);
/// 
/// for _ in 0..100 { rng.next_u32(); }
///
/// assert_eq!(4911005502369895850, rng.next_u64());
/// ```
    Xorwow96, 4);

make_xorwow!(
/// Xorwow implementation with __128__ bits of state
/// plus 32 bits for the modulo 2^32 counter.
///
/// # Example
/// ```
/// use rand_core::{SeedableRng, RngCore};
/// use xorwowgen::Xorwow128;
///
/// let mut rng = Xorwow128::seed_from_u64(4321);
/// 
/// for _ in 0..100 { rng.next_u32(); }
///
/// assert_eq!(4097996158316656424, rng.next_u64());
/// ```
    Xorwow128, 5);

make_xorwow!(
/// Xorwow implementation with __160__ bits of state
/// plus 32 bits for the modulo 2^32 counter.
///
/// # Example
/// ```
/// use rand_core::{SeedableRng, RngCore};
/// use xorwowgen::Xorwow160;
///
/// let mut rng = Xorwow160::seed_from_u64(4321);
/// 
/// for _ in 0..50 { rng.next_u32(); }
///
/// assert_eq!(1148765721, rng.next_u32());
/// ```
    Xorwow160, 6);

make_xorwow!(
/// Xorwow implementation with a footprint of __96__ bits
/// plus 32 bits of counter.
/// Uses _bitxor_ instead of _wrapping_add_ for
/// combining the regular Xorshift with the Weyl sequence.
///
/// # Example
/// ```
/// use rand_core::{SeedableRng, RngCore};
/// use xorwowgen::XorwowXor96;
///
/// let mut rng = XorwowXor96::seed_from_u64(4321);
///
/// for _ in 0..50 { rng.next_u32(); }
///
/// assert_eq!(1471510243, rng.next_u32());
/// ```
    XorwowXor96, 4);

make_xorwow!(
/// Xorwow implementation with a footprint of __128__ bits
/// plus 32 bits of counter.
/// Uses _bitxor_ instead of _wrapping_add_ for
/// combining the regular Xorshift with the Weyl sequence.
///
/// # Example
/// ```
/// use rand_core::{SeedableRng, RngCore};
/// use xorwowgen::XorwowXor128;
///
/// let mut rng = XorwowXor128::seed_from_u64(4321);
///
/// for _ in 0..50 { rng.next_u32(); }
///
/// assert_eq!(2515325973, rng.next_u32());
/// ```
    XorwowXor128, 5);

make_xorwow!(
/// Xorwow implementation with a footprint of __160__ bits
/// plus 32 bits of counter.
/// Uses _bitxor_ instead of _wrapping_add_ for
/// combining the regular Xorshift with the Weyl sequence.
///
/// # Example
/// ```
/// use rand_core::{SeedableRng, RngCore};
/// use xorwowgen::XorwowXor160;
///
/// let mut rng = XorwowXor160::seed_from_u64(4321);
///
/// for _ in 0..50 { rng.next_u32(); }
///
/// assert_eq!(1111799269, rng.next_u32());
/// ```
    XorwowXor160, 6);

macro_rules! impl_xorwow {
    ($name: ident, $mod: ident, $nr: expr, $shift: expr) => {
        impl $name {
           
            fn clock(&mut self) {
                let mut x = self.s[$nr - 2];

                let y = self.s[0];

                for i in (2..($nr - 1)).rev() {
                    self.s[i] = self.s[i - 1];
                }

                self.s[1] = y;
                
                x ^= x >> $shift.0;
                x ^= x << $shift.1;
                x ^= y ^ (y << $shift.2);

                self.s[0] = x;

                // according to the paper, '362437' could be any
                // odd number
                self.s[$nr - 1] = self.s[$nr - 1].wrapping_add(362437);
            }

            pub fn return_u32(&mut self) -> u32 {
                self.clock();

                // combining the regular Xorshift with the Weyl sequence
                // can be done using + or XOR
                self.s[0].$mod(self.s[$nr - 1])
            }
            
            pub fn return_u64(&mut self) -> u64 {
                self.clock();

                let be = self.s[1].$mod(self.s[$nr - 1]) as u64;
                let le = self.s[0].$mod(self.s[$nr - 1]) as u64;

                (be << 32) | le
            }

            pub fn dump_state(&self) -> [u32; $nr] {
                self.s
            }
        }
    };
}

impl_xorwow!(Xorwow96, wrapping_add, 4, (10, 5, 26));
impl_xorwow!(Xorwow128, wrapping_add, 5, (5, 14, 1));
impl_xorwow!(Xorwow160, wrapping_add, 6, (2, 1, 4));
impl_xorwow!(XorwowXor96, bitxor, 4, (10, 5, 26));
impl_xorwow!(XorwowXor128, bitxor, 5, (5, 14, 1));
impl_xorwow!(XorwowXor160, bitxor, 6, (2, 1, 4));

macro_rules! impl_seedable {
    ($name: ident, $nr: expr) => {
        impl SeedableRng for $name {
            type Seed = [u8; $nr * 4];

            fn from_seed(seed: [u8; $nr * 4]) -> Self {
                let mut state = [0u32; $nr];

                read_u32_into(&seed, &mut state);

                let mut all_zero = true;

                // check if all elements except the counter are zero
                for x in state.iter().take($nr - 1) {
                    if *x != 0 {
                        all_zero = false;
                        break;
                    }
                }

                // u32::MAX is used as an alternative seed
                if all_zero {
                    for x in state.iter_mut().take($nr - 1) {
                        *x = u32::MAX;
                    }
                }

                Self { s: state }
            }

            // Map 2^64 possible values to (2^n)-1 possible states.
            // The state must not be entirely zero.
            fn seed_from_u64(seed: u64) -> Self {
                let mut state = [0u32; $nr];

                let be = (seed >> 32) as u32;
                let le = seed as u32;

                for x in state.iter_mut().enumerate().take($nr - 1) {
                    match x.0 % 4 {
                        0 => *x.1 = le,
                        1 => *x.1 = !le,
                        2 => *x.1 = be,
                        3usize.. => *x.1 = !be,
                    }
                }

                Self { s: state }
            }
        }
    };
}

impl_seedable!(Xorwow96, 4);
impl_seedable!(Xorwow128, 5);
impl_seedable!(Xorwow160, 6);
impl_seedable!(XorwowXor96, 4);
impl_seedable!(XorwowXor128, 5);
impl_seedable!(XorwowXor160, 6);

macro_rules! impl_core {
    ($name: ident) => {
        impl RngCore for $name {
            fn next_u32(&mut self) -> u32 {
                self.return_u32()
            }

            fn next_u64(&mut self) -> u64 {
                self.return_u64()
            }

            fn fill_bytes(&mut self, dest: &mut [u8]) {
                fill_bytes_via_next(self, dest);
            }

            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
                self.fill_bytes(dest);
                Ok(())
            }
        }
    };
}

impl_core!(Xorwow96);
impl_core!(Xorwow128);
impl_core!(Xorwow160);
impl_core!(XorwowXor96);
impl_core!(XorwowXor128);
impl_core!(XorwowXor160);
