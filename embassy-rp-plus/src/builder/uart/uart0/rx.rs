use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::{PIN_1, UART0};
use embassy_rp::uart::{Async, Blocking, Config, UartRx};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart0::Irqs;

/// uart0 rx builder
pub struct Uart0RxBuilder<'d> {
    /// uart0 base data
    pub base: UartBase<'d, UART0>,
    /// rx pin
    pub rx: Peri<'d, PIN_1>,
}

/// custom method
impl<'d> Uart0RxBuilder<'d> {
    /// create builder
    #[inline]
    pub fn new(uart: Peri<'d, UART0>, rx: Peri<'d, PIN_1>) -> Self {
        Self { base: UartBase::new(uart), rx: rx }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// build uart tx by async<br />
    /// more see [UartRx::new]
    #[inline]
    pub fn build(self, rx_dma: Peri<'d, impl Channel>) -> UartRx<'d, Async> {
        UartRx::new(self.base.uart, self.rx, Irqs, rx_dma, self.base.config.unwrap_or_default())
    }

    /// build uart tx by blocking<br />
    /// more see [UartRx::<Blocking>::new_blocking]
    #[inline]
    pub fn build_blocking(self) -> UartRx<'d, Blocking> {
        UartRx::new_blocking(self.base.uart, self.rx, self.base.config.unwrap_or_default())
    }
}
