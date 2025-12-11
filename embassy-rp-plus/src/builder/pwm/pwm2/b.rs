use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_21, PIN_5};

/// pwm2 b pin
pub enum Pwm2B<'d> {
    Pin5(Peri<'d, PIN_5>),
    Pin21(Peri<'d, PIN_21>),
}

// support pin.into()
crate::impl_from!(Pwm2B::Pin5(PIN_5),"pin_5");
crate::impl_from!(Pwm2B::Pin21(PIN_21),"pin_21");
