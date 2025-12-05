#[cfg(feature = "spi0")]
pub mod spi0;
#[cfg(feature = "spi0")]
pub mod spi1;
#[cfg(any(feature = "spi0", feature = "spi1"))]
pub mod base;