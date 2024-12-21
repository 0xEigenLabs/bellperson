#![cfg_attr(
    all(target_arch = "aarch64", nightly),
    feature(stdarch_aarch64_prefetch)
)]

#[cfg(test)]
#[macro_use]
extern crate hex_literal;

pub mod domain;
pub mod gadgets;
pub mod gpu;
#[cfg(feature = "groth16")]
pub mod groth16;
pub mod multiexp;
pub mod util_cs;

pub(crate) mod lc;
pub use bellpepper_core::{Circuit, ConstraintSystem, Namespace, SynthesisError};
pub use bellpepper_core::{Index, LinearCombination, Variable};

pub const BELLMAN_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "groth16")]
pub(crate) fn le_bytes_to_u64s(le_bytes: &[u8]) -> Vec<u64> {
    assert_eq!(
        le_bytes.len() % 8,
        0,
        "length must be divisible by u64 byte length (8-bytes)"
    );
    le_bytes
        .chunks(8)
        .map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()))
        .collect()
}
