use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::UART0;
use embassy_rp::uart::{Async, Blocking, Config, UartTx};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart0::tx::Uart0Tx;

/// uart0 tx builder
pub struct Uart0TxBuilder<'d> {
    /// uart0 base data
    pub base: UartBase<'d, UART0>,
    /// tx pin
    pub tx: Uart0Tx<'d>,
}

/// custom method
impl<'d> Uart0TxBuilder<'d> {
    /// create builder
    #[inline]
    pub fn new(uart: Peri<'d, UART0>, tx: Uart0Tx<'d>) -> Self {
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
            Uart0Tx::Pin0(pin_0) => UartTx::new(self.base.uart, pin_0, tx_dma, self.base.config.unwrap_or_default()),
            Uart0Tx::Pin12(pin_12) => UartTx::new(self.base.uart, pin_12, tx_dma, self.base.config.unwrap_or_default()),
            Uart0Tx::Pin16(pin_16) => UartTx::new(self.base.uart, pin_16, tx_dma, self.base.config.unwrap_or_default()),
            Uart0Tx::Pin28(pin_28) => UartTx::new(self.base.uart, pin_28, tx_dma, self.base.config.unwrap_or_default()),
        }
    }

    /// build uart tx by blocking<br />
    /// more see [UartTx::<Blocking>::new_blocking]
    #[inline]
    pub fn build_blocking(self) -> UartTx<'d, Blocking> {
        match self.tx {
            Uart0Tx::Pin0(pin_0) => UartTx::new_blocking(self.base.uart, pin_0, self.base.config.unwrap_or_default()),
            Uart0Tx::Pin12(pin_12) => UartTx::new_blocking(self.base.uart, pin_12, self.base.config.unwrap_or_default()),
            Uart0Tx::Pin16(pin_16) => UartTx::new_blocking(self.base.uart, pin_16, self.base.config.unwrap_or_default()),
            Uart0Tx::Pin28(pin_28) => UartTx::new_blocking(self.base.uart, pin_28, self.base.config.unwrap_or_default()),
        }
    }
}

