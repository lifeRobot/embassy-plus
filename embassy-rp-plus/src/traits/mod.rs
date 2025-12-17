#[cfg(feature = "adc")]
pub mod adc;
#[cfg(feature = "gpio")]
pub mod gpio;
#[cfg(feature = "usb_log")]
pub mod usb;
#[cfg(any(feature = "i2c0", feature = "i2c1"))]
pub mod i2c;
#[cfg(feature = "flash")]
pub mod flash;
#[cfg(any(feature = "uart0", feature = "uart1"))]
pub mod uart;
#[cfg(any(feature = "pio0", feature = "pio1"))]
pub mod pio;
#[cfg(any(feature = "spi0", feature = "spi1"))]
pub mod spi;
#[cfg(any(feature = "gp_in", feature = "gp_out"))]
pub mod clock;
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
#[cfg(feature = "rtc")]
pub mod rtc;
#[cfg(feature = "watchdog")]
pub mod watchdog;