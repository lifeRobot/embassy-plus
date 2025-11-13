use embassy_hal_internal::Peri;
use embassy_rp::{bind_interrupts, uart};
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::UART0;
use embassy_rp::uart::{Async, Blocking, Config, RtsPin, RxPin, TxPin, Uart};
use crate::builder::uart::base::UartBase;
use crate::builder::uart::uart0::cts::Uart0Cts;
use crate::builder::uart::uart0::rts::Uart0Rts;
use crate::builder::uart::uart0::rx::Uart0Rx;
use crate::builder::uart::uart0::tx::Uart0Tx;
use crate::builder::uart::uart0::tx_builder::Uart0TxBuilder;

pub mod rx;
pub mod rx_builder;
pub mod tx;
pub mod tx_builder;
pub mod rts;
pub mod cts;

bind_interrupts!(struct Irqs {
    UART0_IRQ => uart::InterruptHandler<UART0>;
});

/// uart0 builder
pub struct Uart0Builder<'d> {
    /// uart0 base data
    pub base: UartBase<'d, UART0>,
    /// rx pin
    pub rx: Uart0Rx<'d>,
    /// tx pin
    pub tx: Uart0Tx<'d>,
    /// use rts cts
    pub rts_cts: Option<(Uart0Rts<'d>, Uart0Cts<'d>)>,
}

/// custom method
impl<'d> Uart0Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(uart: Peri<'d, UART0>, rx: Uart0Rx<'d>, tx: Uart0Tx<'d>) -> Self {
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
    pub fn rts_cts(mut self, rts: Uart0Rts<'d>, cts: Uart0Cts<'d>) -> Self {
        self.rts_cts = Some((rts, cts));
        self
    }

    /// build uart0 by async<br />
    /// more see [Uart::<Async>::new] or [Uart::<Async>::new_with_rtscts]
    pub fn build(self, tx_dma: Peri<'d, impl Channel>, rx_dma: Peri<'d, impl Channel>) -> Uart<'d, Async> {
        let uart_tx = Uart0TxBuilder { base: self.base, tx: self.tx };
        match self.rx {
            Uart0Rx::Pin1(pin_1) => Self::build_tx(pin_1, uart_tx, self.rts_cts, tx_dma, rx_dma),
            Uart0Rx::Pin13(pin_13) => Self::build_tx(pin_13, uart_tx, self.rts_cts, tx_dma, rx_dma),
            Uart0Rx::Pin17(pin_17) => Self::build_tx(pin_17, uart_tx, self.rts_cts, tx_dma, rx_dma),
            Uart0Rx::Pin29(pin_29) => Self::build_tx(pin_29, uart_tx, self.rts_cts, tx_dma, rx_dma),
        }
    }

    /// build by tx
    fn build_tx(
        rx: Peri<'d, impl RxPin<UART0>>,
        uart_tx: Uart0TxBuilder<'d>,
        rts_cts: Option<(Uart0Rts<'d>, Uart0Cts<'d>)>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Uart<'d, Async> {
        match uart_tx.tx {
            Uart0Tx::Pin0(pin_0) => Self::build_rts_cts(rx, pin_0, uart_tx.base, rts_cts, tx_dma, rx_dma),
            Uart0Tx::Pin12(pin_12) => Self::build_rts_cts(rx, pin_12, uart_tx.base, rts_cts, tx_dma, rx_dma),
            Uart0Tx::Pin16(pin_16) => Self::build_rts_cts(rx, pin_16, uart_tx.base, rts_cts, tx_dma, rx_dma),
            Uart0Tx::Pin28(pin_28) => Self::build_rts_cts(rx, pin_28, uart_tx.base, rts_cts, tx_dma, rx_dma),
        }
    }

    /// build by rts cts
    #[inline]
    fn build_rts_cts(
        rx: Peri<'d, impl RxPin<UART0>>,
        tx: Peri<'d, impl TxPin<UART0>>,
        base: UartBase<'d, UART0>,
        rts_cts: Option<(Uart0Rts<'d>, Uart0Cts<'d>)>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Uart<'d, Async> {
        let (rts, cts) = match rts_cts {
            Some(rts_cts) => rts_cts,
            None => return Uart::new(base.uart, tx, rx, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default())
        };
        match rts {
            Uart0Rts::Pin3(pin_3) => Self::build_cts(rx, tx, base, pin_3, cts, tx_dma, rx_dma),
            Uart0Rts::Pin15(pin_15) => Self::build_cts(rx, tx, base, pin_15, cts, tx_dma, rx_dma),
            Uart0Rts::Pin19(pin_19) => Self::build_cts(rx, tx, base, pin_19, cts, tx_dma, rx_dma),
        }
    }

    /// build by rts
    fn build_cts(
        rx: Peri<'d, impl RxPin<UART0>>,
        tx: Peri<'d, impl TxPin<UART0>>,
        base: UartBase<'d, UART0>,
        rts: Peri<'d, impl RtsPin<UART0>>,
        cts: Uart0Cts<'d>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Uart<'d, Async> {
        match cts {
            Uart0Cts::Pin2(pin_2) => Uart::new_with_rtscts(base.uart, tx, rx, rts, pin_2, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Uart0Cts::Pin14(pin_14) => Uart::new_with_rtscts(base.uart, tx, rx, rts, pin_14, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Uart0Cts::Pin18(pin_18) => Uart::new_with_rtscts(base.uart, tx, rx, rts, pin_18, Irqs, tx_dma, rx_dma, base.config.unwrap_or_default()),
        }
    }

    /// build uart0 by blocking<br />
    /// more see [Uart::<Blocking>::new] or [Uart::<Blocking>::new_with_rtscts_blocking]
    #[inline]
    pub fn build_blocking(self) -> Uart<'d, Blocking> {
        let uart_tx = Uart0TxBuilder { base: self.base, tx: self.tx };
        match self.rx {
            Uart0Rx::Pin1(pin_1) => Self::build_blocking_tx(pin_1, uart_tx, self.rts_cts),
            Uart0Rx::Pin13(pin_13) => Self::build_blocking_tx(pin_13, uart_tx, self.rts_cts),
            Uart0Rx::Pin17(pin_17) => Self::build_blocking_tx(pin_17, uart_tx, self.rts_cts),
            Uart0Rx::Pin29(pin_29) => Self::build_blocking_tx(pin_29, uart_tx, self.rts_cts),
        }
    }

    /// build blocking by tx
    fn build_blocking_tx(
        rx: Peri<'d, impl RxPin<UART0>>,
        uart_tx: Uart0TxBuilder<'d>,
        rts_cts: Option<(Uart0Rts<'d>, Uart0Cts<'d>)>) -> Uart<'d, Blocking> {
        match uart_tx.tx {
            Uart0Tx::Pin0(pin_0) => Self::build_blocking_rts_cts(rx, pin_0, uart_tx.base, rts_cts),
            Uart0Tx::Pin12(pin_12) => Self::build_blocking_rts_cts(rx, pin_12, uart_tx.base, rts_cts),
            Uart0Tx::Pin16(pin_16) => Self::build_blocking_rts_cts(rx, pin_16, uart_tx.base, rts_cts),
            Uart0Tx::Pin28(pin_28) => Self::build_blocking_rts_cts(rx, pin_28, uart_tx.base, rts_cts),
        }
    }

    /// build blocking by rts cts
    fn build_blocking_rts_cts(
        rx: Peri<'d, impl RxPin<UART0>>,
        tx: Peri<'d, impl TxPin<UART0>>,
        base: UartBase<'d, UART0>,
        rts_cts: Option<(Uart0Rts<'d>, Uart0Cts<'d>)>) -> Uart<'d, Blocking> {
        let (rts, cts) = match rts_cts {
            Some(rts_cts) => rts_cts,
            None => return Uart::new_blocking(base.uart, tx, rx, base.config.unwrap_or_default()),
        };

        match rts {
            Uart0Rts::Pin3(pin_3) => Self::build_blocking_cts(rx, tx, base, pin_3, cts),
            Uart0Rts::Pin15(pin_15) => Self::build_blocking_cts(rx, tx, base, pin_15, cts),
            Uart0Rts::Pin19(pin_19) => Self::build_blocking_cts(rx, tx, base, pin_19, cts),
        }
    }

    /// build blocking by rts
    fn build_blocking_cts(
        rx: Peri<'d, impl RxPin<UART0>>,
        tx: Peri<'d, impl TxPin<UART0>>,
        base: UartBase<'d, UART0>,
        rts: Peri<'d, impl RtsPin<UART0>>,
        cts: Uart0Cts<'d>) -> Uart<'d, Blocking> {
        match cts {
            Uart0Cts::Pin2(pin_2) => Uart::new_with_rtscts_blocking(base.uart, tx, rx, rts, pin_2, base.config.unwrap_or_default()),
            Uart0Cts::Pin14(pin_14) => Uart::new_with_rtscts_blocking(base.uart, tx, rx, rts, pin_14, base.config.unwrap_or_default()),
            Uart0Cts::Pin18(pin_18) => Uart::new_with_rtscts_blocking(base.uart, tx, rx, rts, pin_18, base.config.unwrap_or_default()),
        }
    }
}
