use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::UART1;
use embassy_rp::uart::{Async, Blocking, Config, UartTx};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart1::tx::Uart1Tx;

/// uart1 tx builder
pub struct Uart1TxBuilder<'d> {
    /// uart1 base data
    pub base: UartBase<'d, UART1>,
    /// tx pin
    pub tx: Uart1Tx<'d>,
}

/// custom method
impl<'d> Uart1TxBuilder<'d> {
    /// create builder
    #[inline]
    pub fn new(uart: Peri<'d, UART1>, tx: Uart1Tx<'d>) -> Self {
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
        match self.tx {
            Uart1Tx::Pin4(pin_4) => UartTx::new(self.base.uart, pin_4, tx_dma, self.base.config.unwrap_or_default()),
            Uart1Tx::Pin8(pin_8) => UartTx::new(self.base.uart, pin_8, tx_dma, self.base.config.unwrap_or_default()),
            Uart1Tx::Pin20(pin_20) => UartTx::new(self.base.uart, pin_20, tx_dma, self.base.config.unwrap_or_default()),
            Uart1Tx::Pin24(pin_24) => UartTx::new(self.base.uart, pin_24, tx_dma, self.base.config.unwrap_or_default()),
        }
    }

    /// build uart tx by blocking<br />
    /// more see [UartTx::<Blocking>::new_blocking]
    #[inline]
    pub fn build_blocking(self) -> UartTx<'d, Blocking> {
        match self.tx {
            Uart1Tx::Pin4(pin_4) => UartTx::new_blocking(self.base.uart, pin_4, self.base.config.unwrap_or_default()),
            Uart1Tx::Pin8(pin_8) => UartTx::new_blocking(self.base.uart, pin_8, self.base.config.unwrap_or_default()),
            Uart1Tx::Pin20(pin_20) => UartTx::new_blocking(self.base.uart, pin_20, self.base.config.unwrap_or_default()),
            Uart1Tx::Pin24(pin_24) => UartTx::new_blocking(self.base.uart, pin_24, self.base.config.unwrap_or_default()),
        }
    }
}
