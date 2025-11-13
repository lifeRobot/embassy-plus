use embassy_hal_internal::Peri;
use embassy_rp::{bind_interrupts, uart};
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::UART1;
use embassy_rp::uart::{Async, Blocking, Config, RtsPin, RxPin, TxPin, Uart};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart1::cts::Uart1Cts;
use crate::builder::uart::uart1::rts::Uart1Rts;
use crate::builder::uart::uart1::rx::Uart1Rx;
use crate::builder::uart::uart1::tx::Uart1Tx;
use crate::builder::uart::uart1::tx_builder::Uart1TxBuilder;

pub mod rx;
pub mod rx_builder;
pub mod tx;
pub mod tx_builder;
pub mod rts;
pub mod cts;

bind_interrupts!(struct Irqs {
    UART1_IRQ => uart::InterruptHandler<UART1>;
});

/// uart1 builder
pub struct Uart1Builder<'d> {
    /// uart1 base data
    pub base: UartBase<'d, UART1>,
    /// rx pin
    pub rx: Uart1Rx<'d>,
    /// tx pin
    pub tx: Uart1Tx<'d>,
    /// use rts cts
    pub rts_cts: Option<(Uart1Rts<'d>, Uart1Cts<'d>)>,
}

/// custom method
impl<'d> Uart1Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(uart: Peri<'d, UART1>, rx: Uart1Rx<'d>, tx: Uart1Tx<'d>) -> Self {
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
    pub fn rts_cts(mut self, rts: Uart1Rts<'d>, cts: Uart1Cts<'d>) -> Self {
        self.rts_cts = Some((rts, cts));
        self
    }

    /// build uart1 by async<br />
    /// more see [Uart::<Async>::new] or [Uart::<Async>::new_with_rtscts]
    pub fn build(self, tx_dma: Peri<'d, impl Channel>, rx_dma: Peri<'d, impl Channel>) -> Uart<'d, Async> {
        let uart_tx = Uart1TxBuilder { base: self.base, tx: self.tx };
        match self.rx {
            Uart1Rx::Pin5(pin_5) => Self::build_tx(pin_5, uart_tx, self.rts_cts, tx_dma, rx_dma),
            Uart1Rx::Pin9(pin_9) => Self::build_tx(pin_9, uart_tx, self.rts_cts, tx_dma, rx_dma),
            Uart1Rx::Pin21(pin_21) => Self::build_tx(pin_21, uart_tx, self.rts_cts, tx_dma, rx_dma),
            Uart1Rx::Pin25(pin_25) => Self::build_tx(pin_25, uart_tx, self.rts_cts, tx_dma, rx_dma),
        }
    }

    /// build by tx
    fn build_tx(
        rx: Peri<'d, impl RxPin<UART1>>,
        uart_tx: Uart1TxBuilder<'d>,
        rts_cts: Option<(Uart1Rts<'d>, Uart1Cts<'d>)>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Uart<'d, Async> {
        match uart_tx.tx {
            Uart1Tx::Pin4(pin_4) => Self::build_rts_cts(rx, pin_4, uart_tx.base, rts_cts, tx_dma, rx_dma),
            Uart1Tx::Pin8(pin_8) => Self::build_rts_cts(rx, pin_8, uart_tx.base, rts_cts, tx_dma, rx_dma),
            Uart1Tx::Pin20(pin_20) => Self::build_rts_cts(rx, pin_20, uart_tx.base, rts_cts, tx_dma, rx_dma),
            Uart1Tx::Pin24(pin_24) => Self::build_rts_cts(rx, pin_24, uart_tx.base, rts_cts, tx_dma, rx_dma),
        }
    }

    /// build by rts cts
    #[inline]
    fn build_rts_cts(
        rx: Peri<'d, impl RxPin<UART1>>,
        tx: Peri<'d, impl TxPin<UART1>>,
        base: UartBase<'d, UART1>,
        rts_cts: Option<(Uart1Rts<'d>, Uart1Cts<'d>)>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Uart<'d, Async> {
        let (rts, cts) = match rts_cts {
            Some(rts_cts) => rts_cts,
            None => return Uart::new(base.uart, tx, rx, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default())
        };

        match rts {
            Uart1Rts::Pin7(pin_7) => Self::build_cts(rx, tx, base, pin_7, cts, tx_dma, rx_dma),
            Uart1Rts::Pin11(pin_11) => Self::build_cts(rx, tx, base, pin_11, cts, tx_dma, rx_dma),
            Uart1Rts::Pin23(pin_23) => Self::build_cts(rx, tx, base, pin_23, cts, tx_dma, rx_dma),
            Uart1Rts::Pin27(pin_27) => Self::build_cts(rx, tx, base, pin_27, cts, tx_dma, rx_dma),
        }
    }

    /// build by rts
    fn build_cts(
        rx: Peri<'d, impl RxPin<UART1>>,
        tx: Peri<'d, impl TxPin<UART1>>,
        base: UartBase<'d, UART1>,
        rts: Peri<'d, impl RtsPin<UART1>>,
        cts: Uart1Cts<'d>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Uart<'d, Async> {
        match cts {
            Uart1Cts::Pin6(pin_6) => Uart::new_with_rtscts(base.uart, tx, rx, rts, pin_6, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Uart1Cts::Pin10(pin_10) => Uart::new_with_rtscts(base.uart, tx, rx, rts, pin_10, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Uart1Cts::Pin22(pin_22) => Uart::new_with_rtscts(base.uart, tx, rx, rts, pin_22, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Uart1Cts::Pin26(pin_26) => Uart::new_with_rtscts(base.uart, tx, rx, rts, pin_26, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()),
        }
    }

    /// build uart1 by blocking<br />
    /// more see [Uart::<Blocking>::new] or [Uart::<Blocking>::new_with_rtscts_blocking]
    #[inline]
    pub fn build_blocking(self) -> Uart<'d, Blocking> {
        let uart_tx = Uart1TxBuilder { base: self.base, tx: self.tx };
        match self.rx {
            Uart1Rx::Pin5(pin_5) => Self::build_blocking_tx(pin_5, uart_tx, self.rts_cts),
            Uart1Rx::Pin9(pin_9) => Self::build_blocking_tx(pin_9, uart_tx, self.rts_cts),
            Uart1Rx::Pin21(pin_21) => Self::build_blocking_tx(pin_21, uart_tx, self.rts_cts),
            Uart1Rx::Pin25(pin_25) => Self::build_blocking_tx(pin_25, uart_tx, self.rts_cts),
        }
    }

    /// build blocking by tx
    fn build_blocking_tx(
        rx: Peri<'d, impl RxPin<UART1>>,
        uart_tx: Uart1TxBuilder<'d>,
        rts_cts: Option<(Uart1Rts<'d>, Uart1Cts<'d>)>) -> Uart<'d, Blocking> {
        match uart_tx.tx {
            Uart1Tx::Pin4(pin_4) => Self::build_blocking_rts_cts(rx, pin_4, uart_tx.base, rts_cts),
            Uart1Tx::Pin8(pin_8) => Self::build_blocking_rts_cts(rx, pin_8, uart_tx.base, rts_cts),
            Uart1Tx::Pin20(pin_20) => Self::build_blocking_rts_cts(rx, pin_20, uart_tx.base, rts_cts),
            Uart1Tx::Pin24(pin_24) => Self::build_blocking_rts_cts(rx, pin_24, uart_tx.base, rts_cts),
        }
    }

    /// build blocking by rts cts
    #[inline]
    fn build_blocking_rts_cts(
        rx: Peri<'d, impl RxPin<UART1>>,
        tx: Peri<'d, impl TxPin<UART1>>,
        base: UartBase<'d, UART1>,
        rts_cts: Option<(Uart1Rts<'d>, Uart1Cts<'d>)>) -> Uart<'d, Blocking> {
        let (rts, cts) = match rts_cts {
            Some(rts_cts) => rts_cts,
            None => return Uart::new_blocking(base.uart, tx, rx, base.config.unwrap_or_default())
        };

        match rts {
            Uart1Rts::Pin7(pin_7) => Self::build_blocking_cts(rx, tx, base, pin_7, cts),
            Uart1Rts::Pin11(pin_11) => Self::build_blocking_cts(rx, tx, base, pin_11, cts),
            Uart1Rts::Pin23(pin_23) => Self::build_blocking_cts(rx, tx, base, pin_23, cts),
            Uart1Rts::Pin27(pin_27) => Self::build_blocking_cts(rx, tx, base, pin_27, cts),
        }
    }

    /// build blocking by rts
    fn build_blocking_cts(
        rx: Peri<'d, impl RxPin<UART1>>,
        tx: Peri<'d, impl TxPin<UART1>>,
        base: UartBase<'d, UART1>,
        rts: Peri<'d, impl RtsPin<UART1>>,
        cts: Uart1Cts<'d>) -> Uart<'d, Blocking> {
        match cts {
            Uart1Cts::Pin6(pin_6) => Uart::new_with_rtscts_blocking(base.uart, tx, rx, rts, pin_6, base.config.unwrap_or_default()),
            Uart1Cts::Pin10(pin_10) => Uart::new_with_rtscts_blocking(base.uart, tx, rx, rts, pin_10, base.config.unwrap_or_default()),
            Uart1Cts::Pin22(pin_22) => Uart::new_with_rtscts_blocking(base.uart, tx, rx, rts, pin_22, base.config.unwrap_or_default()),
            Uart1Cts::Pin26(pin_26) => Uart::new_with_rtscts_blocking(base.uart, tx, rx, rts, pin_26, base.config.unwrap_or_default()),
        }
    }
}
