#![no_std]

#[cfg(feature = "embassy-executor")]
pub use embassy_executor;
pub use embassy_rp;
#[cfg(feature = "embassy-usb-logger")]
pub use embassy_usb_logger;
#[cfg(feature = "log")]
pub use log;

pub mod builder;
pub mod traits;
mod r#macro;
