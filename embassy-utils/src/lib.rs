#![no_std]

// re export
pub use embassy_sync;
#[cfg(feature = "embassy-rp-plus")]
pub use embassy_rp_plus;

#[cfg(feature = "embassy-rp-plus")]
pub mod flash;
pub mod r#macro;