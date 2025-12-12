use embassy_hal_internal::Peri;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::PWM_SLICE4;
use embassy_rp::pwm::{ChannelAPin, Config, InputMode, Pwm};
use crate::builder::pwm::base::PwmBase;
use crate::builder::pwm::pwm4::a::Pwm4A;
use crate::builder::pwm::pwm4::b::Pwm4B;

pub mod a;
pub mod b;

/// pwm4 builder
pub struct Pwm4Builder<'d> {
    /// pwm4 base data
    pub base: PwmBase<'d, PWM_SLICE4>,
}

/// custom method
impl<'d> Pwm4Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(pwm: Peri<'d, PWM_SLICE4>) -> Self {
        Self { base: PwmBase::new(pwm) }
    }

    /// set pwm config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// Create PWM driver without any configured pins.<br />
    /// more see [Pwm::new_free]
    #[inline]
    pub fn free(self) -> Pwm<'d> {
        Pwm::new_free(self.base.pwm, self.base.config.unwrap_or_default())
    }

    /// Create PWM driver with a single 'a' pin as output.<br />
    /// more see [Pwm::new_output_a]
    #[inline]
    pub fn output_a(self, a: impl Into<Pwm4A<'d>>) -> Pwm<'d> {
        match a.into() {
            Pwm4A::Pin8(pin_8) => Pwm::new_output_a(self.base.pwm, pin_8, self.base.config.unwrap_or_default()),
            Pwm4A::Pin24(pin_24) => Pwm::new_output_a(self.base.pwm, pin_24, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' pin as output.<br />
    /// more see [Pwm::new_output_b]
    #[inline]
    pub fn output_b(self, b: impl Into<Pwm4B<'d>>) -> Pwm<'d> {
        match b.into() {
            Pwm4B::Pin9(pin_9) => Pwm::new_output_b(self.base.pwm, pin_9, self.base.config.unwrap_or_default()),
            Pwm4B::Pin25(pin_25) => Pwm::new_output_b(self.base.pwm, pin_25, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins as output.<br />
    /// more see [Pwm::new_output_ab]
    #[inline]
    pub fn output_ab(self, a: impl Into<Pwm4A<'d>>, b: impl Into<Pwm4B<'d>>) -> Pwm<'d> {
        match a.into() {
            Pwm4A::Pin8(pin_8) => self.build_b(pin_8, b),
            Pwm4A::Pin24(pin_24) => self.build_b(pin_24, b),
        }
    }

    /// build output_ab by b
    #[inline]
    fn build_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE4>>, b: impl Into<Pwm4B<'d>>) -> Pwm<'d> {
        match b.into() {
            Pwm4B::Pin9(pin_9) => Pwm::new_output_ab(self.base.pwm, a, pin_9, self.base.config.unwrap_or_default()),
            Pwm4B::Pin25(pin_25) => Pwm::new_output_ab(self.base.pwm, a, pin_25, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' as input pin.<br />
    /// more see [Pwm::new_input]
    #[inline]
    pub fn input(self, b: impl Into<Pwm4B<'d>>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b.into() {
            Pwm4B::Pin9(pin_9) => Pwm::new_input(self.base.pwm, pin_9, pull, mode, self.base.config.unwrap_or_default()),
            Pwm4B::Pin25(pin_25) => Pwm::new_input(self.base.pwm, pin_25, pull, mode, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins in the desired input mode.<br />
    /// more see [Pwm::new_output_input]
    #[inline]
    pub fn output_input(self, a: impl Into<Pwm4A<'d>>, b: impl Into<Pwm4B<'d>>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match a.into() {
            Pwm4A::Pin8(pin_8) => self.input_b(pin_8, b, pull, mode),
            Pwm4A::Pin24(pin_24) => self.input_b(pin_24, b, pull, mode),
        }
    }

    /// build output_input by b
    #[inline]
    fn input_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE4>>, b: impl Into<Pwm4B<'d>>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b.into() {
            Pwm4B::Pin9(pin_9) => Pwm::new_output_input(self.base.pwm, a, pin_9, pull, mode, self.base.config.unwrap_or_default()),
            Pwm4B::Pin25(pin_25) => Pwm::new_output_input(self.base.pwm, a, pin_25, pull, mode, self.base.config.unwrap_or_default()),
        }
    }
}
