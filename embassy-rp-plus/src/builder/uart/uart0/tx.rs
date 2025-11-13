use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_0, PIN_12, PIN_16, PIN_28};

/// uart0 tx pin
pub enum Uart0Tx<'d> {
    Pin0(Peri<'d, PIN_0>),
    Pin12(Peri<'d, PIN_12>),
    Pin16(Peri<'d, PIN_16>),
    Pin28(Peri<'d, PIN_28>),
}

// support pin.into()
crate::impl_from!(Uart0Tx::Pin0(PIN_0),"pin_0");
crate::impl_from!(Uart0Tx::Pin12(PIN_12),"pin_12");
crate::impl_from!(Uart0Tx::Pin16(PIN_16),"pin_16");
crate::impl_from!(Uart0Tx::Pin28(PIN_28),"pin_28");
