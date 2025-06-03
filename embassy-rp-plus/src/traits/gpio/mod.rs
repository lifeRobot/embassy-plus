use embassy_rp::gpio::{Flex, Input, Level, Output, Pin, Pull};

/// gpio trait
pub trait GpioTrait: Pin {
    /// Create GPIO input driver, default Pull is [Pull::Up]<br />
    /// more see [Self::input_with_pull] or [Input::new]
    #[inline]
    fn input(self) -> Input<'static> {
        self.input_with_pull(Pull::Up)
    }

    /// Create GPIO input driver, more see [Input::new]
    #[inline]
    fn input_with_pull(self, pull: Pull) -> Input<'static> {
        Input::new(self, pull)
    }

    /// Create GPIO output driver, default level is [Level::High]<br />
    /// more see [Self::output_with_level] or [Output::new]
    #[inline]
    fn output(self) -> Output<'static> {
        self.output_with_level(Level::High)
    }

    /// Create GPIO output driver, more see [Output::new]
    #[inline]
    fn output_with_level(self, level: Level) -> Output<'static> {
        Output::new(self, level)
    }

    /// Create GPIO flex driver, more see [Flex::new]
    #[inline]
    fn flex(self) -> Flex<'static> {
        Flex::new(self)
    }
}

/// any pin support gpio trait
impl<T: Pin> GpioTrait for T {}
