use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_19, PIN_3};

/// pwm1 b pin
pub enum Pwm1B<'d> {
    Pin3(Peri<'d, PIN_3>),
    Pin19(Peri<'d, PIN_19>),
}

// support pin.into()
crate::impl_from!(Pwm1B::Pin3(PIN_3),"pin_3");
crate::impl_from!(Pwm1B::Pin19(PIN_19),"pin_19");
