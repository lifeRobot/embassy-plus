use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_10, PIN_22, PIN_26, PIN_6};

/// uart1 cts pin
pub enum Uart1Cts<'d> {
    Pin6(Peri<'d, PIN_6>),
    Pin10(Peri<'d, PIN_10>),
    Pin22(Peri<'d, PIN_22>),
    Pin26(Peri<'d, PIN_26>),
}

crate::impl_from!(Uart1Cts::Pin6(PIN_6),"pin_6");
crate::impl_from!(Uart1Cts::Pin10(PIN_10),"pin_10");
crate::impl_from!(Uart1Cts::Pin22(PIN_22),"pin_22");
crate::impl_from!(Uart1Cts::Pin26(PIN_26),"pin_26");
