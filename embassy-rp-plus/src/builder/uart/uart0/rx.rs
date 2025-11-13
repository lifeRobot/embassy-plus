use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_1, PIN_13, PIN_17, PIN_29};

/// uart0 rx pin
pub enum Uart0Rx<'d> {
    Pin1(Peri<'d, PIN_1>),
    Pin13(Peri<'d, PIN_13>),
    Pin17(Peri<'d, PIN_17>),
    Pin29(Peri<'d, PIN_29>),
}

crate::impl_from!(Uart0Rx::Pin1(PIN_1),"pin_1");
crate::impl_from!(Uart0Rx::Pin13(PIN_13),"pin_13");
crate::impl_from!(Uart0Rx::Pin17(PIN_17),"pin_17");
crate::impl_from!(Uart0Rx::Pin29(PIN_29),"pin_29");
