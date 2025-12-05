use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_11, PIN_15, PIN_27};

/// spi1 mosi pin
pub enum Spi1Mosi<'d> {
    Pin11(Peri<'d, PIN_11>),
    Pin15(Peri<'d, PIN_15>),
    Pin27(Peri<'d, PIN_27>),
}

// support pin.into()
crate::impl_from!(Spi1Mosi::Pin11(PIN_11),"pin_11");
crate::impl_from!(Spi1Mosi::Pin15(PIN_15),"pin_15");
crate::impl_from!(Spi1Mosi::Pin27(PIN_27),"pin_27");
