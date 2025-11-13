use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_15, PIN_19, PIN_3};

/// uart0 rts pin
pub enum Uart0Rts<'d> {
    Pin3(Peri<'d, PIN_3>),
    Pin15(Peri<'d, PIN_15>),
    Pin19(Peri<'d, PIN_19>),
}

crate::impl_from!(Uart0Rts::Pin3(PIN_3),"pin_3");
crate::impl_from!(Uart0Rts::Pin15(PIN_15),"pin_15");
crate::impl_from!(Uart0Rts::Pin19(PIN_19),"pin_19");
