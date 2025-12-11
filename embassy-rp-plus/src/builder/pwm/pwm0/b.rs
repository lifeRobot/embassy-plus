use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_1, PIN_17};

/// pwm0 b pin
pub enum Pwm0B<'d> {
    Pin1(Peri<'d, PIN_1>),
    Pin17(Peri<'d, PIN_17>),
}

// support pin.into()
crate::impl_from!(Pwm0B::Pin1(PIN_1),"pin_1");
crate::impl_from!(Pwm0B::Pin17(PIN_17),"pin_17");
