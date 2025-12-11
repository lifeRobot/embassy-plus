use embassy_hal_internal::Peri;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::PWM_SLICE1;
use embassy_rp::pwm::{ChannelAPin, Config, InputMode, Pwm};
use crate::builder::pwm::base::PwmBase;
use crate::builder::pwm::pwm1::a::Pwm1A;
use crate::builder::pwm::pwm1::b::Pwm1B;

pub mod a;
pub mod b;

/// pwm1 builder
pub struct Pwm1Builder<'d> {
    /// pwm1 base data
    pub base: PwmBase<'d, PWM_SLICE1>,
}

/// custom method
impl<'d> Pwm1Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(pwm: Peri<'d, PWM_SLICE1>) -> Self {
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
    pub fn output_a(self, a: Pwm1A<'d>) -> Pwm<'d> {
        match a {
            Pwm1A::Pin2(pin_2) => Pwm::new_output_a(self.base.pwm, pin_2, self.base.config.unwrap_or_default()),
            Pwm1A::Pin18(pin_18) => Pwm::new_output_a(self.base.pwm, pin_18, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' pin as output.<br />
    /// more see [Pwm::new_output_b]
    #[inline]
    pub fn output_b(self, b: Pwm1B<'d>) -> Pwm<'d> {
        match b {
            Pwm1B::Pin3(pin_3) => Pwm::new_output_b(self.base.pwm, pin_3, self.base.config.unwrap_or_default()),
            Pwm1B::Pin19(pin_19) => Pwm::new_output_b(self.base.pwm, pin_19, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins as output.<br />
    /// more see [Pwm::new_output_ab]
    #[inline]
    pub fn output_ab(self, a: Pwm1A<'d>, b: Pwm1B<'d>) -> Pwm<'d> {
        match a {
            Pwm1A::Pin2(pin_2) => self.build_b(pin_2, b),
            Pwm1A::Pin18(pin_18) => self.build_b(pin_18, b),
        }
    }

    /// build output_ab by b
    #[inline]
    fn build_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE1>>, b: Pwm1B<'d>) -> Pwm<'d> {
        match b {
            Pwm1B::Pin3(pin_3) => Pwm::new_output_ab(self.base.pwm, a, pin_3, self.base.config.unwrap_or_default()),
            Pwm1B::Pin19(pin_19) => Pwm::new_output_ab(self.base.pwm, a, pin_19, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' as input pin.<br />
    /// more see [Pwm::new_input]
    #[inline]
    pub fn input(self, b: Pwm1B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b {
            Pwm1B::Pin3(pin_3) => Pwm::new_input(self.base.pwm, pin_3, pull, mode, self.base.config.unwrap_or_default()),
            Pwm1B::Pin19(pin_19) => Pwm::new_input(self.base.pwm, pin_19, pull, mode, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins in the desired input mode.<br />
    /// more see [Pwm::new_output_input]
    #[inline]
    pub fn output_input(self, a: Pwm1A<'d>, b: Pwm1B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match a {
            Pwm1A::Pin2(pin_2) => self.input_b(pin_2, b, pull, mode),
            Pwm1A::Pin18(pin_18) => self.input_b(pin_18, b, pull, mode),
        }
    }

    /// build output_input by b
    #[inline]
    fn input_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE1>>, b: Pwm1B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b {
            Pwm1B::Pin3(pin_3) => Pwm::new_output_input(self.base.pwm, a, pin_3, pull, mode, self.base.config.unwrap_or_default()),
            Pwm1B::Pin19(pin_19) => Pwm::new_output_input(self.base.pwm, a, pin_19, pull, mode, self.base.config.unwrap_or_default()),
        }
    }
}
