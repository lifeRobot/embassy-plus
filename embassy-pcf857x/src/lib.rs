#![no_std]

#[cfg(feature = "embassy-rp")]
pub mod i2c_lock;
#[cfg(feature = "embassy-rp")]
pub mod pcf8575;
#[cfg(feature = "embassy-rp")]
pub mod pcf8574;