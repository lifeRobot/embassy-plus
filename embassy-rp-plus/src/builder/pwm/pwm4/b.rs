use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_25, PIN_9};

/// pwm4 b pin
pub enum Pwm4B<'d> {
    Pin9(Peri<'d, PIN_9>),
    Pin25(Peri<'d, PIN_25>),
}

// support pin.into()
crate::impl_from!(Pwm4B::Pin9(PIN_9),"pin_9");
crate::impl_from!(Pwm4B::Pin25(PIN_25),"pin_25");