use embassy_hal_internal::Peri;
use embassy_rp::{bind_interrupts, uart};
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::{PIN_0, PIN_1, PIN_2, PIN_3, UART0};
use embassy_rp::uart::{Async, Blocking, Config, Uart};
use crate::builder::uart::base::UartBase;

pub mod rx;
pub mod tx;

bind_interrupts!(struct Irqs {
    UART0_IRQ => uart::InterruptHandler<UART0>;
});

/// uart0 builder
pub struct Uart0Builder<'d> {
    /// uart0 base data
    pub base: UartBase<'d, UART0>,
    /// rx pin
    pub rx: Peri<'d, PIN_1>,
    /// tx pin
    pub tx: Peri<'d, PIN_0>,
    /// use rts cts
    pub rts_cts: Option<(Peri<'d, PIN_3>, Peri<'d, PIN_2>)>,
}

/// custom method
impl<'d> Uart0Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(uart: Peri<'d, UART0>, rx: Peri<'d, PIN_1>, tx: Peri<'d, PIN_0>) -> Self {
        Self { base: UartBase::new(uart), rx, tx, rts_cts: None }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// set rts cts
    #[inline]
    pub fn rts_cts(mut self, rts: Peri<'d, PIN_3>, cts: Peri<'d, PIN_2>) -> Self {
        self.rts_cts = Some((rts, cts));
        self
    }

    /// build uart0 by async<br />
    /// more see [Uart::<Async>::new] or [Uart::<Async>::new_with_rtscts]
    #[inline]
    pub fn build(self, tx_dma: Peri<'d, impl Channel>, rx_dma: Peri<'d, impl Channel>) -> Uart<'d, Async> {
        match self.rts_cts {
            Some((rts, cts)) => Uart::new_with_rtscts(self.base.uart, self.tx, self.rx, rts, cts, Irqs, tx_dma, rx_dma, self.base.config.unwrap_or_default()),
            None => Uart::new(self.base.uart, self.tx, self.rx, Irqs, tx_dma, rx_dma, self.base.config.unwrap_or_default())
        }
    }

    /// build uart0 by blocking<br />
    /// more see [Uart::<Blocking>::new] or [Uart::<Blocking>::new_with_rtscts_blocking]
    #[inline]
    pub fn build_blocking(self) -> Uart<'d, Blocking> {
        match self.rts_cts {
            Some((rts, cts)) => Uart::new_with_rtscts_blocking(self.base.uart, self.tx, self.rx, rts, cts, self.base.config.unwrap_or_default()),
            None => Uart::new_blocking(self.base.uart, self.tx, self.rx, self.base.config.unwrap_or_default())
        }
    }
}
