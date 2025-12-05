use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_10, PIN_14, PIN_26};

/// spi1 clk pin
pub enum Spi1Clk<'d> {
    Pin10(Peri<'d, PIN_10>),
    Pin14(Peri<'d, PIN_14>),
    Pin26(Peri<'d, PIN_26>),
}

// support pin.into()
crate::impl_from!(Spi1Clk::Pin10(PIN_10),"pin_10");
crate::impl_from!(Spi1Clk::Pin14(PIN_14),"pin_14");
crate::impl_from!(Spi1Clk::Pin26(PIN_26),"pin_26");
