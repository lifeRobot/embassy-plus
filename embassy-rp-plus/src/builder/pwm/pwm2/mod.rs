use embassy_hal_internal::Peri;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::PWM_SLICE2;
use embassy_rp::pwm::{ChannelAPin, Config, InputMode, Pwm};
use crate::builder::pwm::base::PwmBase;
use crate::builder::pwm::pwm2::a::Pwm2A;
use crate::builder::pwm::pwm2::b::Pwm2B;

pub mod a;
pub mod b;

/// pwm2 builder
pub struct Pwm2Builder<'d> {
    /// pwm2 base data
    pub base: PwmBase<'d, PWM_SLICE2>,
}

/// custom method
impl<'d> Pwm2Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(pwm: Peri<'d, PWM_SLICE2>) -> Self {
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
    pub fn output_a(self, a: Pwm2A<'d>) -> Pwm<'d> {
        match a {
            Pwm2A::Pin4(pin_4) => Pwm::new_output_a(self.base.pwm, pin_4, self.base.config.unwrap_or_default()),
            Pwm2A::Pin20(pin_20) => Pwm::new_output_a(self.base.pwm, pin_20, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' pin as output.<br />
    /// more see [Pwm::new_output_b]
    #[inline]
    pub fn output_b(self, b: Pwm2B<'d>) -> Pwm<'d> {
        match b {
            Pwm2B::Pin5(pin_5) => Pwm::new_output_b(self.base.pwm, pin_5, self.base.config.unwrap_or_default()),
            Pwm2B::Pin21(pin_21) => Pwm::new_output_b(self.base.pwm, pin_21, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins as output.<br />
    /// more see [Pwm::new_output_ab]
    #[inline]
    pub fn output_ab(self, a: Pwm2A<'d>, b: Pwm2B<'d>) -> Pwm<'d> {
        match a {
            Pwm2A::Pin4(pin_4) => self.build_b(pin_4, b),
            Pwm2A::Pin20(pin_20) => self.build_b(pin_20, b),
        }
    }

    /// build output_ab by b
    #[inline]
    fn build_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE2>>, b: Pwm2B<'d>) -> Pwm<'d> {
        match b {
            Pwm2B::Pin5(pin_5) => Pwm::new_output_ab(self.base.pwm, a, pin_5, self.base.config.unwrap_or_default()),
            Pwm2B::Pin21(pin_21) => Pwm::new_output_ab(self.base.pwm, a, pin_21, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' as input pin.<br />
    /// more see [Pwm::new_input]
    #[inline]
    pub fn input(self, b: Pwm2B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b {
            Pwm2B::Pin5(pin_5) => Pwm::new_input(self.base.pwm, pin_5, pull, mode, self.base.config.unwrap_or_default()),
            Pwm2B::Pin21(pin_21) => Pwm::new_input(self.base.pwm, pin_21, pull, mode, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins in the desired input mode.<br />
    /// more see [Pwm::new_output_input]
    #[inline]
    pub fn output_input(self, a: Pwm2A<'d>, b: Pwm2B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match a {
            Pwm2A::Pin4(pin_4) => self.input_b(pin_4, b, pull, mode),
            Pwm2A::Pin20(pin_20) => self.input_b(pin_20, b, pull, mode),
        }
    }

    /// build output_input by b
    #[inline]
    fn input_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE2>>, b: Pwm2B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b {
            Pwm2B::Pin5(pin_5) => Pwm::new_output_input(self.base.pwm, a, pin_5, pull, mode, self.base.config.unwrap_or_default()),
            Pwm2B::Pin21(pin_21) => Pwm::new_output_input(self.base.pwm, a, pin_21, pull, mode, self.base.config.unwrap_or_default()),
        }
    }
}
