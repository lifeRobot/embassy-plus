use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::UART0;
use embassy_rp::uart::{Async, Blocking, Config, UartRx};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart0::Irqs;
use crate::builder::uart::uart0::rx::Uart0Rx;

/// uart0 rx builder
pub struct Uart0RxBuilder<'d> {
    /// uart0 base data
    pub base: UartBase<'d, UART0>,
    /// rx pin
    pub rx: Uart0Rx<'d>,
}

/// custom method
impl<'d> Uart0RxBuilder<'d> {
    /// create builder
    #[inline]
    pub fn new(uart: Peri<'d, UART0>, rx: Uart0Rx<'d>) -> Self {
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
            Uart0Rx::Pin1(pin_1) => UartRx::new(self.base.uart, pin_1, Irqs, rx_dma, self.base.config.unwrap_or_default()),
            Uart0Rx::Pin13(pin_13) => UartRx::new(self.base.uart, pin_13, Irqs, rx_dma, self.base.config.unwrap_or_default()),
            Uart0Rx::Pin17(pin_17) => UartRx::new(self.base.uart, pin_17, Irqs, rx_dma, self.base.config.unwrap_or_default()),
            Uart0Rx::Pin29(pin_29) => UartRx::new(self.base.uart, pin_29, Irqs, rx_dma, self.base.config.unwrap_or_default()),
        }
    }

    /// build uart tx by blocking<br />
    /// more see [UartRx::<Blocking>::new_blocking]
    #[inline]
    pub fn build_blocking(self) -> UartRx<'d, Blocking> {
        match self.rx {
            Uart0Rx::Pin1(pin_1) => UartRx::new_blocking(self.base.uart, pin_1, self.base.config.unwrap_or_default()),
            Uart0Rx::Pin13(pin_13) => UartRx::new_blocking(self.base.uart, pin_13, self.base.config.unwrap_or_default()),
            Uart0Rx::Pin17(pin_17) => UartRx::new_blocking(self.base.uart, pin_17, self.base.config.unwrap_or_default()),
            Uart0Rx::Pin29(pin_29) => UartRx::new_blocking(self.base.uart, pin_29, self.base.config.unwrap_or_default()),
        }
    }
}
