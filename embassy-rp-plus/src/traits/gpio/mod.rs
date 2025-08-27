use embassy_hal_internal::Peri;
use embassy_rp::gpio::{Flex, Input, Level, Output, Pin, Pull};

/// gpio trait
pub trait GpioTrait<'d>: Sized {
    /// Create GPIO input driver, default Pull is [Pull::Up]<br />
    /// more see [Self::input_with_pull] or [Input::new]
    #[inline]
    fn input(self) -> Input<'d> {
        self.input_with_pull(Pull::Up)
    }

    /// Create GPIO input driver, more see [Input::new]
    fn input_with_pull(self, pull: Pull) -> Input<'d>;

    /// Create GPIO output driver, default level is [Level::High]<br />
    /// more see [Self::output_with_level] or [Output::new]
    #[inline]
    fn output(self) -> Output<'d> {
        self.output_with_level(Level::High)
    }

    /// Create GPIO output driver, more see [Output::new]
    fn output_with_level(self, level: Level) -> Output<'d>;

    /// Create GPIO flex driver, more see [Flex::new]
    fn flex(self) -> Flex<'d>;
}

/// any pin support gpio trait
impl<'d,T: Pin> GpioTrait<'d> for Peri<'d,T> {
    #[inline]
    fn input_with_pull(self, pull: Pull) -> Input<'d> {
        Input::new(self, pull)
    }

    #[inline]
    fn output_with_level(self, level: Level) -> Output<'d> {
        Output::new(self, level)
    }

    #[inline]
    fn flex(self) -> Flex<'d> {
        Flex::new(self)
    }
}
