use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_20, PIN_24, PIN_4, PIN_8};

/// uart1 tx pin
pub enum Uart1Tx<'d> {
    Pin4(Peri<'d, PIN_4>),
    Pin8(Peri<'d, PIN_8>),
    Pin20(Peri<'d, PIN_20>),
    Pin24(Peri<'d, PIN_24>),
}

crate::impl_from!(Uart1Tx::Pin4(PIN_4),"pin_4");
crate::impl_from!(Uart1Tx::Pin8(PIN_8),"pin_8");
crate::impl_from!(Uart1Tx::Pin20(PIN_20),"pin_20");
crate::impl_from!(Uart1Tx::Pin24(PIN_24),"pin_24");
