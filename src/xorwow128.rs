//! Xorwow derivatives with 2 * 64 = 128 bits of state and
//! a modulo 2^64 counter.
//! 
//! Source of the shift triple of the underlying Xorshift
//! generator:
//! [https://vigna.di.unimi.it/ftp/papers/xorshiftplus.pdf](https://vigna.di.unimi.it/ftp/papers/xorshiftplus.pdf)

use rand_core::{SeedableRng, RngCore, Error};
use rand_core::impls::fill_bytes_via_next;
use rand_core::le::read_u64_into;
use std::ops::BitXor;
use crate::impl_core;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

macro_rules! make_xorwow128 {
    ($(#[$meta:meta])*
    $name: ident) => (
        $(#[$meta])*
        #[derive(Debug, Clone, Eq, PartialEq)]
        #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
        pub struct $name {
            s: [u64; 3]
        }
    )
}

make_xorwow128!(
/// Modification of the underlying Xorshift stream is
/// performed using _wrapping_add_.
/// # Example
/// ```
/// use xorwowgen::xorwow128::LargeWrap;
/// use rand_core::{SeedableRng, RngCore};
///
/// let mut rng = LargeWrap::seed_from_u64(9876543214321);
/// for _ in 0..100 { rng.next_u64(); }
///
/// assert_eq!(6194833746010933040, rng.next_u64());
/// ```
    LargeWrap);

make_xorwow128!(
/// Modification of the underlying Xorshift stream is
/// performed using _bitxor_.
/// # Example
/// ```
/// use xorwowgen::xorwow128::LargeXor;
/// use rand_core::{SeedableRng, RngCore};
///
/// let mut rng = LargeXor::seed_from_u64(1234567898765);
/// for _ in 0..100 { rng.next_u64(); }
///
/// assert_eq!(2242453002770973956, rng.next_u64());
/// ```
    LargeXor);

// a = 23, b = 17, c = 26
macro_rules! impl_xorwow128 {
    ($name: ident, $mod: ident, $shift: expr) => {
        impl $name {
            fn clock(&mut self) {
                let mut a = self.s[0];
                let b = self.s[1];
                self.s[0] = b;
                a ^= a << $shift.0;
                a ^= a >> $shift.1;
                a ^= b ^ (b >> $shift.2);
                self.s[1] = a;
                self.s[2] = self.s[2].wrapping_add(0x587CC7F5F9DD5);
            }

            pub fn return_u32(&mut self) -> u32 {
                self.clock();
                (self.s[0].$mod(self.s[2])) as u32
            }

            pub fn return_u64(&mut self) -> u64 {
                self.clock();
                self.s[0].$mod(self.s[2])
            }

            pub fn dump_state(&self) -> [u64; 3] {
                self.s
            }
        }
    }
}

impl_xorwow128!(LargeWrap, wrapping_add, (23, 17, 26));
impl_xorwow128!(LargeXor, bitxor, (23, 17, 26));

macro_rules! impl_seedable {
    ($name: ident) => {
        impl SeedableRng for $name {
            type Seed = [u8; 24];

            fn from_seed(seed: [u8; 24]) -> Self {
                let mut state = [0u64; 3];

                read_u64_into(&seed, &mut state);

                if state[..2] == [0u64; 2] {
                    state[0] = u64::MAX;
                    state[1] = u64::MAX;
                }
                
                Self { s: state }
            }

            fn seed_from_u64(seed: u64) -> Self {
                let mut state = [0u64; 3];
                
                if seed == 0u64 {
                    state[0] = u64::MAX;
                } else {
                    state[0] = seed;
                }

                state[1] = seed;
                state[2] = !seed;

                Self { s: state }
            }
        }
    };
}

impl_seedable!(LargeWrap);
impl_seedable!(LargeXor);

impl_core!(LargeWrap);
impl_core!(LargeXor);
