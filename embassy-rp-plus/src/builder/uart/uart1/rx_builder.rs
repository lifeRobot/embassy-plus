use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::UART1;
use embassy_rp::uart::{Async, Blocking, Config, UartRx};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart1::Irqs;
use crate::builder::uart::uart1::rx::Uart1Rx;

/// uart1 rx builder
pub struct Uart1RxBuilder<'d> {
    /// uart1 base data
    pub base: UartBase<'d, UART1>,
    /// rx pin
    pub rx: Uart1Rx<'d>,
}

/// custom method
impl<'d> Uart1RxBuilder<'d> {
    /// create builder
    #[inline]
    pub fn new(uart: Peri<'d, UART1>, rx: Uart1Rx<'d>) -> Self {
        Self { base: UartBase::new(uart), rx }
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
        match self.rx {
            Uart1Rx::Pin5(pin_5) => UartRx::new(self.base.uart, pin_5, Irqs, rx_dma, self.base.config.unwrap_or_default()),
            Uart1Rx::Pin9(pin_9) => UartRx::new(self.base.uart, pin_9, Irqs, rx_dma, self.base.config.unwrap_or_default()),
            Uart1Rx::Pin21(pin_21) => UartRx::new(self.base.uart, pin_21, Irqs, rx_dma, self.base.config.unwrap_or_default()),
            Uart1Rx::Pin25(pin_25) => UartRx::new(self.base.uart, pin_25, Irqs, rx_dma, self.base.config.unwrap_or_default()),
        }
    }

    /// build uart tx by blocking<br />
    /// more see [UartRx::<Blocking>::new_blocking]
    #[inline]
    pub fn build_blocking(self) -> UartRx<'d, Blocking> {
        match self.rx {
            Uart1Rx::Pin5(pin_5) => UartRx::new_blocking(self.base.uart, pin_5, self.base.config.unwrap_or_default()),
            Uart1Rx::Pin9(pin_9) => UartRx::new_blocking(self.base.uart, pin_9, self.base.config.unwrap_or_default()),
            Uart1Rx::Pin21(pin_21) => UartRx::new_blocking(self.base.uart, pin_21, self.base.config.unwrap_or_default()),
            Uart1Rx::Pin25(pin_25) => UartRx::new_blocking(self.base.uart, pin_25, self.base.config.unwrap_or_default()),
        }
    }
}
