use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_11, PIN_23, PIN_27, PIN_7};

/// uart1 rts pin
pub enum Uart1Rts<'d> {
    Pin7(Peri<'d, PIN_7>),
    Pin11(Peri<'d, PIN_11>),
    Pin23(Peri<'d, PIN_23>),
    Pin27(Peri<'d, PIN_27>),
}

crate::impl_from!(Uart1Rts::Pin7(PIN_7),"pin_7");
crate::impl_from!(Uart1Rts::Pin11(PIN_11),"pin_11");
crate::impl_from!(Uart1Rts::Pin23(PIN_23),"pin_23");
crate::impl_from!(Uart1Rts::Pin27(PIN_27),"pin_27");
