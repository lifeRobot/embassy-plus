use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_21, PIN_25, PIN_5, PIN_9};

/// uart1 rx pin
pub enum Uart1Rx<'d> {
    Pin5(Peri<'d, PIN_5>),
    Pin9(Peri<'d, PIN_9>),
    Pin21(Peri<'d, PIN_21>),
    Pin25(Peri<'d, PIN_25>),
}

crate::impl_from!(Uart1Rx::Pin5(PIN_5),"pin_5");
crate::impl_from!(Uart1Rx::Pin9(PIN_9),"pin_9");
crate::impl_from!(Uart1Rx::Pin21(PIN_21),"pin_21");
crate::impl_from!(Uart1Rx::Pin25(PIN_25),"pin_25");
