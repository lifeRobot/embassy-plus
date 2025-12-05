use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_18, PIN_2, PIN_22, PIN_6};

/// spi0 clk pin
pub enum Spi0Clk<'d> {
    Pin2(Peri<'d, PIN_2>),
    Pin6(Peri<'d, PIN_6>),
    Pin18(Peri<'d, PIN_18>),
    Pin22(Peri<'d, PIN_22>),
}

// support pin.into()
crate::impl_from!(Spi0Clk::Pin2(PIN_2),"pin_2");
crate::impl_from!(Spi0Clk::Pin6(PIN_6),"pin_6");
crate::impl_from!(Spi0Clk::Pin18(PIN_18),"pin_18");
crate::impl_from!(Spi0Clk::Pin22(PIN_22),"pin_22");
