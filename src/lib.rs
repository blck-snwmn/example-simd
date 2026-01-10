#![cfg_attr(feature = "nightly", feature(portable_simd))]

pub mod scalar;
pub mod scalar_unrolled;
pub mod simd_wide;

#[cfg(feature = "nightly")]
pub mod simd_std;
