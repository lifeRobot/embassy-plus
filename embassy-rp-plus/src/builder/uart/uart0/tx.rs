use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::{PIN_0, UART0};
use embassy_rp::uart::{Async, Blocking, Config, UartTx};
use crate::builder::uart::base::UartBase;

/// uart0 tx builder
pub struct Uart0TxBuilder<'d> {
    /// uart0 base data
    pub base: UartBase<'d, UART0>,
    /// tx pin
    pub tx: Peri<'d, PIN_0>,
}

/// custom method
impl<'d> Uart0TxBuilder<'d> {
    /// create builder
    #[inline]
    pub fn new(uart: Peri<'d, UART0>, tx: Peri<'d, PIN_0>) -> Self {
        Self { base: UartBase::new(uart), tx }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// build uart tx by async<br />
    /// more see [UartTx::new]
    #[inline]
    pub fn build(self, tx_dma: Peri<'d, impl Channel>) -> UartTx<'d, Async> {
        UartTx::new(self.base.uart, self.tx, tx_dma, self.base.config.unwrap_or_default())
    }

    /// build uart tx by blocking<br />
    /// more see [UartTx::<Blocking>::new_blocking]
    #[inline]
    pub fn build_b(self) -> UartTx<'d, Blocking> {
        UartTx::new_blocking(self.base.uart, self.tx, self.base.config.unwrap_or_default())
    }
}
