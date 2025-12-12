use embassy_hal_internal::Peri;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::PWM_SLICE6;
use embassy_rp::pwm::{ChannelAPin, Config, InputMode, Pwm};
use crate::builder::pwm::base::PwmBase;
use crate::builder::pwm::pwm6::a::Pwm6A;
use crate::builder::pwm::pwm6::b::Pwm6B;

pub mod a;
pub mod b;

/// pwm6 builder
pub struct Pwm6Builder<'d> {
    /// pwm6 base data
    pub base: PwmBase<'d, PWM_SLICE6>,
}

/// custom method
impl<'d> Pwm6Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(pwm: Peri<'d, PWM_SLICE6>) -> Self {
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
    pub fn output_a(self, a: impl Into<Pwm6A<'d>>) -> Pwm<'d> {
        match a.into() {
            Pwm6A::Pin12(pin_12) => Pwm::new_output_a(self.base.pwm, pin_12, self.base.config.unwrap_or_default()),
            Pwm6A::Pin28(pin_28) => Pwm::new_output_a(self.base.pwm, pin_28, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' pin as output.<br />
    /// more see [Pwm::new_output_b]
    #[inline]
    pub fn output_b(self, b: impl Into<Pwm6B<'d>>) -> Pwm<'d> {
        match b.into() {
            Pwm6B::Pin13(pin_13) => Pwm::new_output_b(self.base.pwm, pin_13, self.base.config.unwrap_or_default()),
            Pwm6B::Pin29(pin_29) => Pwm::new_output_b(self.base.pwm, pin_29, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins as output.<br />
    /// more see [Pwm::new_output_ab]
    #[inline]
    pub fn output_ab(self, a: impl Into<Pwm6A<'d>>, b: impl Into<Pwm6B<'d>>) -> Pwm<'d> {
        match a.into() {
            Pwm6A::Pin12(pin_12) => self.build_b(pin_12, b),
            Pwm6A::Pin28(pin_28) => self.build_b(pin_28, b),
        }
    }

    /// build output_ab by b
    #[inline]
    fn build_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE6>>, b: impl Into<Pwm6B<'d>>) -> Pwm<'d> {
        match b.into() {
            Pwm6B::Pin13(pin_13) => Pwm::new_output_ab(self.base.pwm, a, pin_13, self.base.config.unwrap_or_default()),
            Pwm6B::Pin29(pin_29) => Pwm::new_output_ab(self.base.pwm, a, pin_29, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' as input pin.<br />
    /// more see [Pwm::new_input]
    #[inline]
    pub fn input(self, b: impl Into<Pwm6B<'d>>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b.into() {
            Pwm6B::Pin13(pin_13) => Pwm::new_input(self.base.pwm, pin_13, pull, mode, self.base.config.unwrap_or_default()),
            Pwm6B::Pin29(pin_29) => Pwm::new_input(self.base.pwm, pin_29, pull, mode, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins in the desired input mode.<br />
    /// more see [Pwm::new_output_input]
    #[inline]
    pub fn output_input(self, a: impl Into<Pwm6A<'d>>, b: impl Into<Pwm6B<'d>>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match a.into() {
            Pwm6A::Pin12(pin_12) => self.input_b(pin_12, b, pull, mode),
            Pwm6A::Pin28(pin_28) => self.input_b(pin_28, b, pull, mode),
        }
    }

    /// build output_input by b
    #[inline]
    fn input_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE6>>, b: impl Into<Pwm6B<'d>>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b.into() {
            Pwm6B::Pin13(pin_13) => Pwm::new_output_input(self.base.pwm, a, pin_13, pull, mode, self.base.config.unwrap_or_default()),
            Pwm6B::Pin29(pin_29) => Pwm::new_output_input(self.base.pwm, a, pin_29, pull, mode, self.base.config.unwrap_or_default()),
        }
    }
}
