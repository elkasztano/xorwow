//! Very fast Xorwow derivatives. Consist of a single 64
//! bit state and a modulo 2^64 counter.

use rand_core::{SeedableRng, RngCore, Error};
use rand_core::impls::fill_bytes_via_next;
use rand_core::le::read_u64_into;
use std::ops::BitXor;
use crate::impl_core;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

macro_rules! make_xorwow64 {
    ($(#[$meta:meta])*
    $name: ident) => (
        $(#[$meta])*
        #[derive(Debug, Clone, Eq, PartialEq)]
        #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
        pub struct $name {
            s: [u64; 2]
        }
    )
}

make_xorwow64!(
/// Utilizes the following triple for the bit shift:
/// [_13, 7, 17_]
/// Modification of the regular xorshift is performed using
/// _wrapping_add_.
/// # Example
/// ```rust
/// use rand_core::{SeedableRng, RngCore};
/// use xorwowgen::xorwow64::WrapA;
///
/// let mut rng = WrapA::seed_from_u64(987654321);
/// for _ in 0..50 {
///     rng.next_u64();
/// }
///
/// assert_eq!(1090866054122946625, rng.next_u64());
/// ```
    WrapA);

make_xorwow64!(
/// Utilizes the following triple for the bit shift:
/// [_13, 19, 28_]
/// Modification of the regular xorshift is performed using
/// _wrapping_add_.
/// # Example
/// ```rust
/// use rand_core::{SeedableRng, RngCore};
/// use xorwowgen::xorwow64::WrapB;
///
/// let mut rng = WrapB::seed_from_u64(987654321);
/// for _ in 0..50 {
///     rng.next_u64();
/// }
///
/// assert_eq!(17419553017648195578, rng.next_u64());
/// ```
    WrapB);

make_xorwow64!(
/// Utilizes the following triple for the bit shift:
/// [_13, 7, 17_]
/// Modification of the regular xorshift is performed using
/// _bitxor_.
/// # Example
/// ```rust
/// use rand_core::{SeedableRng, RngCore};
/// use xorwowgen::xorwow64::XorA;
///
/// let mut rng = XorA::seed_from_u64(987654321);
/// for _ in 0..50 {
///     rng.next_u64();
/// }
///
/// assert_eq!(1086342340810259457, rng.next_u64());
/// ```
    XorA);

make_xorwow64!(
/// Utilizes the following triple for the bit shift:
/// [_13, 19, 28_]
/// Modification of the regular xorshift is performed using
/// _bitxor_.
/// # Example
/// ```rust
/// use rand_core::{SeedableRng, RngCore};
/// use xorwowgen::xorwow64::XorB;
///
/// let mut rng = XorB::seed_from_u64(987654321);
/// for _ in 0..50 {
///     rng.next_u64();
/// }
///
/// assert_eq!(17419550427514181626, rng.next_u64());
/// ```
    XorB);

macro_rules! impl_xorwow64 {
    ($name: ident, $mod: ident, $shift: expr) => {
        impl $name {
            fn clock(&mut self) {
                self.s[0] ^= self.s[0] << $shift.0;
                self.s[0] ^= self.s[0] >> $shift.1;
                self.s[0] ^= self.s[0] << $shift.2;
                self.s[1] = self.s[1].wrapping_add(0x587CC7F5F9DD5);
            }

            pub fn return_u32(&mut self) -> u32 {
                self.clock();
                (self.s[0].$mod(self.s[1])) as u32
            }

            pub fn return_u64(&mut self) -> u64 {
                self.clock();
                self.s[0].$mod(self.s[1])
            }

            pub fn dump_state(&self) -> [u64; 2] {
                self.s
            }
        }
    }
}

impl_xorwow64!(WrapA, wrapping_add, (13, 7, 17));
impl_xorwow64!(WrapB, wrapping_add, (13, 19, 28));
impl_xorwow64!(XorA, bitxor, (13, 7, 17));
impl_xorwow64!(XorB, bitxor, (13, 19, 28));

macro_rules! impl_seedable {
    ($name: ident) => {
        impl SeedableRng for $name {
            type Seed = [u8; 16];

            fn from_seed(seed: [u8; 16]) -> Self {
                let mut state = [0u64; 2];

                read_u64_into(&seed, &mut state);

                if state[0] == 0u64 {
                    state[0] = u64::MAX;
                }
                
                Self { s: state }
            }

            fn seed_from_u64(seed: u64) -> Self {
                let mut state = [0u64; 2];
                
                if seed == 0u64 {
                    state[0] = u64::MAX;
                } else {
                    state[0] = seed;
                }

                state[1] = seed;

                Self { s: state }
            }
        }
    };
}

impl_seedable!(WrapA);
impl_seedable!(WrapB);
impl_seedable!(XorA);
impl_seedable!(XorB);

impl_core!(WrapA);
impl_core!(WrapB);
impl_core!(XorA);
impl_core!(XorB);
