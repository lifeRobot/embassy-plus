#[cfg(feature = "uart0")]
pub mod uart0;
#[cfg(feature = "uart1")]
pub mod uart1;
#[cfg(any(feature = "uart0", feature = "uart1"))]
pub mod base;