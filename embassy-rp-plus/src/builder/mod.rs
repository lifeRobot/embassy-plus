#[cfg(feature = "usb_log")]
pub mod usb;
#[cfg(any(feature = "i2c0", feature = "i2c1"))]
pub mod i2c;
#[cfg(any(feature = "uart0", feature = "uart1"))]
pub mod uart;
#[cfg(any(feature = "spi0", feature = "spi1"))]
pub mod spi;
#[cfg(any(
    feature = "pwm0",
    feature = "pwm1",
    feature = "pwm2",
    feature = "pwm3",
    feature = "pwm4",
    feature = "pwm5",
    feature = "pwm6",
    feature = "pwm7"
))]
pub mod pwm;