use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_14, PIN_18, PIN_2};

/// uart0 cts pin
pub enum Uart0Cts<'d> {
    Pin2(Peri<'d, PIN_2>),
    Pin14(Peri<'d, PIN_14>),
    Pin18(Peri<'d, PIN_18>),
}

// support pin.into()
crate::impl_from!(Uart0Cts::Pin2(PIN_2),"pin_2");
crate::impl_from!(Uart0Cts::Pin14(PIN_14),"pin_14");
crate::impl_from!(Uart0Cts::Pin18(PIN_18),"pin_18");
