use embassy_hal_internal::Peri;
use embassy_rp::adc::{Adc, Async, Blocking, Config, InterruptHandler};
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::ADC;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

/// adc trait
pub trait AdcTrait<'d>: Sized {
    /// create adc driver, config use default<br />
    /// more see [Adc::new]
    #[inline]
    fn build(self) -> Adc<'d, Async> {
        self.build_with_config(Config::default())
    }

    /// create adc blocking driver, config use default<br />
    /// more see [Adc::new_blocking]
    #[inline]
    fn build_blocking(self) -> Adc<'d, Blocking> {
        self.build_blocking_with_config(Config::default())
    }

    /// create adc driver, config custom
    /// more see [Adc::new]
    fn build_with_config(self, config: Config) -> Adc<'d, Async>;

    /// create adc blocking driver, config custom
    /// more see [Adc::new_blocking]
    fn build_blocking_with_config(self, config: Config) -> Adc<'d, Blocking>;
}

/// adc support adc trait
impl<'d> AdcTrait<'d> for Peri<'d, ADC> {
    #[inline]
    fn build_with_config(self, config: Config) -> Adc<'d, Async> {
        Adc::new(self, Irqs, config)
    }

    #[inline]
    fn build_blocking_with_config(self, config: Config) -> Adc<'d, Blocking> {
        Adc::new_blocking(self, config)
    }
}
